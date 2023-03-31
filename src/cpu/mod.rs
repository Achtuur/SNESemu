use std::sync::{Arc, Mutex};

use crate::memory::Memory;

use self::processorstatusflag::ProcessorStatusFlags;

pub mod processorstatusflag;
pub mod instructions;

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

    memory: Arc<Mutex<Memory>>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            sp: 0,
            pc: 0, //todo -> set to correct init value
            acc: 0,
            status: ProcessorStatusFlags::from_bits(0).unwrap(),
            memory: Arc::new(Mutex::new(Memory::new())),
            x: 0,
            y: 0,
            dp: 0,
            dbr: 0,
            pbr: 0,
        }
    }

    /// Initialize variables
    /// 
    /// Currently unused, could be removed as most initial values can be set in `Cpu::new()`
    pub fn init(&mut self) {

    }

    // This function is called every 'clock cycle'
    pub fn tick(&mut self) {
        // read instruction

        // read data needed by instruction

        // execute instruction
    }

    /// Give the cpu a reference to the memory that is shared between every device on the SNES
    pub fn set_memory(&mut self, memory: Arc<Mutex<Memory>>) {
        self.memory = memory;
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        // abs_addr = $DBAABB where $DB is cpu dbr register and $AABB are the bytes of addr
        let abs_addr = (self.dbr as u32) << 16 | (addr as u32);
        self.memory.lock().unwrap().read(abs_addr)
    }

    pub fn mem_write(&mut self, addr: u16, val: u8) {
        let abs_addr = (self.dbr as u32) << 16 | (addr as u32);
        self.memory.lock().unwrap().write(abs_addr, val);
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

    /// Push `val` onto stack, decrement stack pointer after
    /// 
    /// Locks `self.memory`
    pub fn push_byte_stack(&mut self, byte: u8) {
        self.memory.lock().unwrap().write(self.sp as u32, byte);
        self.sp -= 1;
    }

    /// Pull single byte from stack, increments stack pointer after
    /// 
    /// Locks `self.memory`
    pub fn pull_byte_stack(&mut self) -> u8 {
        let byte = self.memory.lock().unwrap().read(self.sp as u32);
        self.sp += 1;
        byte
    }

    /// Push a long onto the stack, first pushes low bits, then high bits
    /// 
    /// if `sp = $1FFE` and `long = $#1234`, then `$1FFE = $#34` and `$1FFD = $#12`
    /// 
    /// Locks `self.memory`
    pub fn push_long_stack(&mut self, long: u16) {
        self.push_byte_stack(long as u8);
        self.push_byte_stack((long >> 8) as u8);
    }


    /// Pulls long from stack, assumption is that high byte is stored after low byte of long (see [push_long_stack])
    /// 
    /// Locks `self.memory` by calling pull_byte_stack
    /// 
    /// It should be no problem if the lock is lost in between calls to [pull_byte_stack], as no other component of the SNES should read/write from stack or edit the stack pointer
    pub fn pull_long_stack(&mut self) -> u16 {
        let upper = self.pull_byte_stack() as u16;
        let lower = self.pull_byte_stack() as u16;
        (upper << 8) | lower
    }
}
