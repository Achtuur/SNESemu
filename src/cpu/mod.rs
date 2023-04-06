use std::sync::{Arc, Mutex};

use crate::memory::Memory;

use self::{processorstatusflag::ProcessorStatusFlags, instructions::instructions::Instruction};

pub mod processorstatusflag;
pub mod instructions;
mod execute;

pub enum CpuError {
    PlaceHolder,
}

pub struct Cpu {
    // Registers
    /// Stack pointer
    /// (Points to the next available(unused) location on the stack.)
    sp: u16,

    /// Program bank register
    /// (Holds the bank address of the currently executing instruction)
    pbr: u8,

    /// Program counter
    /// (Holds the address of the current instruction to execute.)
    pc: u16,

    /// Accumulator
    /// (This is the math register. It stores one of two operands or the result of most arithmetic and logical operations.)
    acc: u16,

    /// Processor status flags
    status: ProcessorStatusFlags,

    /// Index register X
    /// (Can be used to reference memory, to pass data to memory, or as counters for loops.)
    x: u16,

    /// Index register Y
    /// (Can be used to reference memory, to pass data to memory, or as counters for loops.)        
    y: u16,

    /// Direct page register
    /// 
    /// Holds the memory bank address the CPU is currently accessing
    dp: u16,
    /// Data bank register
    /// 
    /// Any data that is read from memory is first stored in this register
    dbr: u8,

    /// Memory read/write register, either holds last byte that is read from memory or the byte that should be written to memory
    /// 
    /// When `Cpu::mem_read(addr)` is called, mdr will be updated if the address specified by `addr` is not open bus, else it will keep the old value
    /// 
    /// When `Cpu::mem_write(addr)` is called, the byte in `mdr` will be written to the address specified by `addr`
    mdr: u8,

    /// Reference to global Memory mutex that is also used by ppu and apu
    memory: Arc<Mutex<Memory>>,

    /// Amount of cycles to wait before next instruction is called (simulates instructions taking x cycles)
    wait_cycles: usize,

}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: Arc::new(Mutex::new(Memory::new())),
            status: ProcessorStatusFlags::from_bits(0).unwrap(),
            sp: 0,
            pc: 0xFFFC,
            acc: 0,
            x: 0,
            y: 0,
            dp: 0,
            dbr: 0,
            pbr: 0,
            mdr: 0,
            wait_cycles: 0,
        }
    }

    /// Reset CPU, sets all values to initial state
    pub fn reset(&mut self) {
        // self.memory.lock().unwrap().reset(); // todo -> implement memory reset
        self.status = ProcessorStatusFlags::from_bits(0).unwrap();
        self.sp = 0;
        self.pc = 0xFFFC; // reset vector
        self.acc = 0;
        self.x = 0;
        self.y = 0;
        self.dp = 0;
        self.dbr = 0;
        self.pbr = 0;
        self.mdr = 0;
        self.wait_cycles = 0;
    }

    // This function is called every 'clock cycle'
    pub fn tick(&mut self, nmi_pending: bool, irq_pending: bool) -> Result<(), CpuError> {
        if self.wait_cycles > 0 {
            self.wait_cycles -= 1;
        }

        // NMI pending -> execute NMI
        if nmi_pending {
            return self.execute_nmi();
        }
        // IRQ pending and IRQ not disabled -> execute IRQ
        else if irq_pending && !self.status.contains(ProcessorStatusFlags::IRQdisable) {
            return self.execute_irq();
        }
        // IRQ pending and irq disabled AND WAI flag on -> execute instruction as normal
        else if irq_pending && self.status.contains(ProcessorStatusFlags::IRQdisable | ProcessorStatusFlags::WaitForInterrupt) {
            self.status.clear_flag(ProcessorStatusFlags::WaitForInterrupt);
        }

        // read & execute instruction
        let op = self.mem_read(self.get_pc_addr());
        let instr = Instruction::from_op(op);
        self.execute_instruction(instr)
    }

    /// Returns pc long address
    pub fn get_pc_addr(&self) -> u32 {
        ((self.pbr as u32) << 16) | self.pc as u32
    }

    /// Returns current bank as `$BB0000`, where `$BB == self.dbr`
    pub fn get_bank(&self) -> u32 {
        (self.dbr as u32) << 16
    }

    /// Give the cpu a reference to the memory that is shared between every device on the SNES
    pub fn set_memory(&mut self, memory: Arc<Mutex<Memory>>) {
        self.memory = memory;
    }

    /// Read from memory using a 24 bit address, result is stored in `self.mdr`
    pub fn mem_read_long(&mut self, low_addr: u32, high_addr: u32) -> u16 {
        let low_byte = self.mem_read(low_addr) as u16;
        let high_byte = self.mem_read(high_addr) as u16; 
        (high_byte << 8) | low_byte
    }

    /// Write 16 bit value to memory, low byte is written to low_addr and high byte is written to high_addr
    pub fn mem_write_long(&mut self, low_addr: u32, high_addr: u32, val: u16) {
        self.mem_write(low_addr, val as u8);
        self.mem_write(high_addr, (val >> 8) as u8);
    }

    /// Read from memory using a 24 bit address,
    /// result is stored in `self.mdr` and returned.
    pub fn mem_read(&mut self, addr: u32) -> u8 {
        if let Some(byte) = self.memory.lock().unwrap().read(addr) {
            self.mdr = byte;
        }
        self.mdr
    }

    /// Write to memory using a 16 bit address, the final address is `$DDHHLL` where `$DD` is equal to `self.dbr` and `$HHLL` is equal to `addr`.
    /// `byte` is passed to `self.mdr` and is written to memory
    pub fn mem_write(&mut self, addr: u32, byte: u8) {
        self.mdr = byte;
        self.memory.lock().unwrap().write(addr, self.mdr);
    }

    /// Returns `0_u16` if carry flag is unset, `1_u16` if carry flag is set
    pub fn carry(&self) -> u16 {
        match self.status.contains(ProcessorStatusFlags::Carry) {
            true => 1,
            false => 0,
        }
    }

    /// Returns accumulator value as either 16 bit or 8 bits depending on accumulator 8 bit flag. 
    /// 
    /// 8 bit value is actually just 16 bit value ANDed with `0xF`
    pub fn get_acc(&self) -> u16 {
        match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
            true => self.acc & 0xF,
            false => self.acc,
        }
    }

    /// Set accumulator, sets lower bytes of accumulator to `val` if 8 bit mode for accumulator register is enabled,
    /// else set accumulator to `val`
    pub fn set_acc(&mut self, val: u16) {
        match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
            true => self.acc = (self.acc & 0xF0) | (val & 0xF),
            false => self.acc = val,
        }
    }

    /// Call this function after setting accumulator to set negative and zero flags. Takes into account 16/8 bit mode
    pub fn set_acc_nz_flag(&mut self) {
        self.status.set(ProcessorStatusFlags::Zero, self.acc == 0);
        match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
            true => self.status.set(ProcessorStatusFlags::Negative, (self.acc as i8) < 0),
            false => self.status.set(ProcessorStatusFlags::Negative, (self.acc as i16) < 0),
        }
    }

    /// Returns x register value as either 16 bit or 8 bits depending on x register 8 bit flag. 
    /// 
    /// 8 bit value is actually just 16 bit value ANDed with `0xF`
    pub fn get_x(&self) -> u16 {
        match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
            true => self.x & 0xF,
            false => self.x,
        }
    }

    /// Set x, ANDS `val` with `0xF` if 8 bit mode for x register is enabled
    pub fn set_x(&mut self, val: u16) {
        match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
            true => self.x = val & 0xF,
            false => self.x = val,
        }
    }

    /// Call this function after setting x register to set negative and zero flags. Takes into account 16/8 bit mode
    pub fn set_x_nz_flag(&mut self) {
        self.status.set(ProcessorStatusFlags::Zero, self.x == 0);
        match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => self.status.set(ProcessorStatusFlags::Negative, (self.x as i8) < 0),
            false => self.status.set(ProcessorStatusFlags::Negative, (self.x as i16) < 0),
		}
    }

    /// Returns y register value as either 16 bit or 8 bits depending on y register 8 bit flag. 
    /// 
    /// 8 bit value is actually just 16 bit value ANDed with `0yF`
    pub fn get_y(&self) -> u16 {
        match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
            true => self.y & 0xF,
            false => self.y,
        }
    }

    /// Set y, ANDS `val` with `0yF` if 8 bit mode for y register is enabled
    pub fn set_y(&mut self, val: u16) {
        match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
            true => self.y = val & 0xF,
            false => self.y = val,
        }
    }

    /// Call this function after setting y register to set negative and zero flags. Takes into account 16/8 bit mode
    pub fn set_y_nz_flag(&mut self) {
        self.status.set(ProcessorStatusFlags::Zero, self.y == 0);
        match self.status.contains(ProcessorStatusFlags::XYreg8bit) {
			true => self.status.set(ProcessorStatusFlags::Negative, (self.y as i8) < 0),
            false => self.status.set(ProcessorStatusFlags::Negative, (self.y as i16) < 0),
		}
    }

    /// Puts `byte` in `self.mdr` and then pushes it onto the stack, decrement stack pointer after
    /// 
    /// Locks `self.memory`
    pub fn push_byte_stack(&mut self, byte: u8) {
        self.mem_write(self.sp as u32, byte);
        self.sp -= 1;
    }

    /// Pull single byte from stack and puts it in `self.mdr` and returns `self.mdr`.
    /// Increments stack pointer.
    /// 
    /// Locks `self.memory`
    pub fn pull_byte_stack(&mut self) -> u8 {
        self.mem_read(self.sp as u32);
        self.sp += 1;
        self.mdr
    }

    /// Push a long onto the stack, first pushes high byte, then low byte
    /// 
    /// if `sp = $1FFE` and `long = $#1234`, then `$1FFE = $#12` and `$1FFD = $#34`
    /// 
    /// Locks `self.memory`
    pub fn push_long_stack(&mut self, long: u16) {
        self.push_byte_stack((long >> 8) as u8);
        self.push_byte_stack(long as u8);
    }


    /// Pulls long from stack, assumption is that high byte is stored before low byte of long (see [push_long_stack])
    /// 
    /// Locks `self.memory` by calling pull_byte_stack
    /// 
    /// It should be no problem if the lock is lost in between calls to [pull_byte_stack], as no other component of the SNES should read/write from stack or edit the stack pointer
    pub fn pull_long_stack(&mut self) -> u16 {
        let lower = self.pull_byte_stack() as u16;
        let upper = self.pull_byte_stack() as u16;
        (upper << 8) | lower
    }
}
