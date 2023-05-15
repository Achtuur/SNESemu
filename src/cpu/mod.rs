use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use self::{statusflag::StatusFlags, instructions::instructions::Instruction, memory::CpuMemory};

pub mod statusflag;
pub mod instructions;
pub mod memory;
mod execute;

lazy_static! {
    pub static ref NMI_PENDING: Mutex<bool> = Mutex::new(false);
    pub static ref IRQ_PENDING: Mutex<bool> = Mutex::new(false);
}

pub enum CpuError {
    PlaceHolder,
}

pub struct SCpu {
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
    status: StatusFlags,

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

    /// Cpu Memory, holds ram, rom and other cpu registers. Is also used to communicate with controllers, apu and ppu
    pub memory: CpuMemory,

    /// Amount of cycles to wait before next instruction is called (simulates instructions taking x cycles)
    wait_cycles: usize,

}

impl SCpu {
    pub fn new() -> Self {
        SCpu {
            memory: CpuMemory::new(),
            status: StatusFlags::startup_state(),
            sp: 0x1FF,
            pc: 0,
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
        self.status = StatusFlags::startup_state();
        self.sp = 0x1FF;
        self.pc = self.mem_read_long(0xFFFC, 0xFFFD); // reset vector
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
    pub fn tick(&mut self) -> Result<(), CpuError> {

        if self.status.contains(StatusFlags::Emulation) {
            self.sp = (1 << 8) | (self.sp & 0x00FF); //force high byte to be 1
        }

        // force x and y to zero
        if self.status.contains(StatusFlags::XYreg8bit) {
            self.x &= 0x00FF;
            self.y &= 0x00FF;
        }

        if self.wait_cycles > 0 {
            self.wait_cycles -= 1;
        }

        // NMI pending -> execute NMI
        if *NMI_PENDING.lock().unwrap() {
            return self.execute_nmi();
        }
        // IRQ pending and IRQ not disabled -> execute IRQ
        else if *IRQ_PENDING.lock().unwrap() && !self.status.contains(StatusFlags::IRQdisable) {
            return self.execute_irq();
        }
        // IRQ pending and irq disabled AND WAI flag on -> execute instruction as normal
        else if *IRQ_PENDING.lock().unwrap() && self.status.contains(StatusFlags::IRQdisable | StatusFlags::WaitForInterrupt) {
            self.status.clear_flag(StatusFlags::WaitForInterrupt);
        }

        // read & execute instruction
        let op = self.mem_read(self.get_pc_addr());
        let instr = Instruction::from_op(op);
        println!("[{0:04X}]: {1:?}\t[{2:?}]", self.pc, instr, self.status);
        println!("sp: [{0:04X}]\tA: [{1:04X}]\tX: [{2:04X}]\tY: [{3:04X}]", self.sp, self.acc, self.x, self.y);
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
        if let Some(byte) = self.memory.read(addr) {
            self.mdr = byte;
        }
        self.mdr
    }

    /// Write to memory using a 16 bit address, the final address is `$DDHHLL` where `$DD` is equal to `self.dbr` and `$HHLL` is equal to `addr`.
    /// `byte` is passed to `self.mdr` and is written to memory
    pub fn mem_write(&mut self, addr: u32, byte: u8) {
        self.mdr = byte;
        self.memory.write(addr, self.mdr);
    }

    /// Returns `0_u16` if carry flag is unset, `1_u16` if carry flag is set
    pub fn carry(&self) -> u16 {
        match self.status.contains(StatusFlags::Carry) {
            true => 1,
            false => 0,
        }
    }

    /// Returns accumulator value as either 16 bit or 8 bits depending on accumulator 8 bit flag. 
    /// 
    /// 8 bit value is actually just 16 bit value ANDed with `0xFF`
    pub fn get_acc(&self) -> u16 {
        match self.status.contains(StatusFlags::Accumulator8bit) {
            true => self.acc & 0xFF,
            false => self.acc,
        }
    }

    /// Set accumulator, sets lower bytes of accumulator to `val` if 8 bit mode for accumulator register is enabled,
    /// else set accumulator to `val`
    pub fn set_acc(&mut self, val: u16) {
        match self.status.contains(StatusFlags::Accumulator8bit) {
            true => self.acc = (self.acc & 0xFF00) | (val & 0xFF),
            false => self.acc = val,
        }
    }

    /// Call this function after setting accumulator to set negative and zero flags. Takes into account 16/8 bit mode
    pub fn set_acc_nz_flag(&mut self) {
        self.status.set(StatusFlags::Zero, self.acc == 0);
        match self.status.contains(StatusFlags::Accumulator8bit) {
            true => self.status.set(StatusFlags::Negative, (self.acc as i8) < 0),
            false => self.status.set(StatusFlags::Negative, (self.acc as i16) < 0),
        }
    }

    /// Returns x register value as either 16 bit or 8 bits depending on x register 8 bit flag. 
    /// 
    /// 8 bit value is actually just 16 bit value ANDed with `0xFF`
    pub fn get_x(&self) -> u16 {
        match self.status.contains(StatusFlags::XYreg8bit) {
            true => self.x & 0xFF,
            false => self.x,
        }
    }

    /// Set x, ANDS `val` with `0xFF` if 8 bit mode for x register is enabled
    pub fn set_x(&mut self, val: u16) {
        match self.status.contains(StatusFlags::XYreg8bit) {
            true => self.x = val & 0xFF,
            false => self.x = val,
        }
    }

    /// Call this function after setting x register to set negative and zero flags. Takes into account 16/8 bit mode
    pub fn set_x_nz_flag(&mut self) {
        self.status.set(StatusFlags::Zero, self.x == 0);
        match self.status.contains(StatusFlags::XYreg8bit) {
			true => self.status.set(StatusFlags::Negative, (self.x as i8) < 0),
            false => self.status.set(StatusFlags::Negative, (self.x as i16) < 0),
		}
    }

    /// Returns y register value as either 16 bit or 8 bits depending on y register 8 bit flag. 
    /// 
    /// 8 bit value is actually just 16 bit value ANDed with `0xFF`
    pub fn get_y(&self) -> u16 {
        match self.status.contains(StatusFlags::XYreg8bit) {
            true => self.y & 0xFF,
            false => self.y,
        }
    }

    /// Set y, ANDS `val` with `0xFF` if 8 bit mode for y register is enabled
    pub fn set_y(&mut self, val: u16) {
        match self.status.contains(StatusFlags::XYreg8bit) {
            true => self.y = val & 0xFF,
            false => self.y = val,
        }
    }

    /// Call this function after setting y register to set negative and zero flags. Takes into account 16/8 bit mode
    pub fn set_y_nz_flag(&mut self) {
        self.status.set(StatusFlags::Zero, self.y == 0);
        match self.status.contains(StatusFlags::XYreg8bit) {
			true => self.status.set(StatusFlags::Negative, (self.y as i8) < 0),
            false => self.status.set(StatusFlags::Negative, (self.y as i16) < 0),
		}
    }

    /// Puts `byte` in `self.mdr` and then pushes it onto the stack, decrement stack pointer after
    /// 
    /// Locks `self.memory`
    pub fn push_byte_stack(&mut self, byte: u8) {
        self.mem_write(self.sp as u32, byte);
        self.sp = self.sp.wrapping_sub(1);
    }

    /// Pull single byte from stack and puts it in `self.mdr` and returns `self.mdr`.
    /// Increments stack pointer.
    pub fn pull_byte_stack(&mut self) -> u8 {
        self.mem_read(self.sp as u32);
        self.sp = self.sp.wrapping_add(1);
        self.mdr
    }

    /// Push a long onto the stack, first pushes high byte, then low byte
    /// 
    /// if `sp = $1FFE` and `long = $#1234`, then `$1FFE = $#12` and `$1FFD = $#34`
    pub fn push_long_stack(&mut self, long: u16) {
        self.push_byte_stack((long >> 8) as u8);
        self.push_byte_stack(long as u8);
    }


    /// Pulls long from stack, assumption is that high byte is stored before low byte of long (see [push_long_stack])
    /// 
    /// It should be no problem if the lock is lost in between calls to [pull_byte_stack], as no other component of the SNES should read/write from stack or edit the stack pointer
    pub fn pull_long_stack(&mut self) -> u16 {
        let lower = self.pull_byte_stack() as u16;
        let upper = self.pull_byte_stack() as u16;
        (upper << 8) | lower
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::{statusflag::StatusFlags, SCpu}, arc_mut, ppu::memory::PpuMemory, apu::memory::ApuMemory};

	fn get_test_cpu() -> SCpu {
		let mut cpu = SCpu::new();
		let ppumem = arc_mut!(PpuMemory::new());
        cpu.memory.set_ppumemory_ref(ppumem.clone());
		let apumem = arc_mut!(ApuMemory::new());
        cpu.memory.set_apumemory_ref(apumem.clone());
		cpu
	}

    #[test]
    fn test_acc() {
        let mut cpu = get_test_cpu();

        cpu.status.set_flag(StatusFlags::Accumulator8bit);
        cpu.acc = 0xABCD;
        cpu.set_acc(0xAB);
        assert_eq!(cpu.get_acc(), 0x00AB);
        assert_eq!(cpu.acc, 0xABAB);

        cpu.status.clear_flag(StatusFlags::Accumulator8bit);
        cpu.acc = 0xABCD;
        cpu.set_acc(0xABCD);
        assert_eq!(cpu.get_acc(), 0xABCD);
        assert_eq!(cpu.acc, 0xABCD);

    }
}
