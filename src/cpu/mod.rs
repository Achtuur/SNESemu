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
    /// Program bank register
    /// (Holds the bank address of all instruction fetches.)
    pbr: u8,

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

    /// Give the cpu a reference to the memory that is shared between every device on the SNES
    pub fn set_memory(&mut self, memory: Arc<Mutex<Memory>>) {
        self.memory = memory;
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

    /// Set x, ANDS `val` with `0xF` if 8 bit mode for accumulator register is enabled
    pub fn set_acc(&mut self, val: u16) {
        match self.status.contains(ProcessorStatusFlags::Accumulator8bit) {
            true => self.acc = val & 0xF,
            false => self.acc = val,
        }
    }

    /// Returns x register value as either 16 bit or 8 bits depending on x register 8 bit flag. 
    /// 
    /// 8 bit value is actually just 16 bit value ANDed with `0xF`
    pub fn get_x(&self) -> u16 {
        match self.status.contains(ProcessorStatusFlags::Xreg8bit) {
            true => self.x & 0xF,
            false => self.x,
        }
    }

    /// Set x, ANDS `val` with `0xF` if 8 bit mode for x register is enabled
    pub fn set_x(&mut self, val: u16) {
        match self.status.contains(ProcessorStatusFlags::Xreg8bit) {
            true => self.x = val & 0xF,
            false => self.x = val,
        }
    }
}
