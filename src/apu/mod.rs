use std::sync::{Arc, Mutex};

use crate::{arc_mut, bit_slice, to_word, low_byte, high_byte};
use self::statusword::StatusWord;
use self::memory::ApuMemory;

pub mod divider;
pub mod timer;
pub mod dsp;
pub mod memory;
pub mod statusword;
pub mod instructions;
pub mod execute;

/// Audio processing unit
pub struct SApu {
    /// Accumulator
    acc: u8,
    /// X register
    x: u8,
    /// Y register
    y: u8,
    /// Stack pointer
    sp: u8,
    /// Program counter
    pc: u16,
    /// Processor status word
    status: StatusWord,
    memory: Arc<Mutex<ApuMemory>>,

    cycle_time: usize,
}


impl SApu {
    pub fn new() -> Self {
        SApu {
            acc: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            status: StatusWord::startup_state(),            
            memory: arc_mut!(ApuMemory::new()),
            cycle_time: 0,
        }
    }

    pub fn mem_read(&mut self, addr: u16) -> u8 {
        todo!()
    }

    pub fn mem_write(&mut self, addr: u16, byte: u8) {
        todo!()
    }

    pub fn stack_push_long(&mut self, data: u16) {
        self.stack_push(low_byte!(data));
        self.stack_push(high_byte!(data));
    }

    pub fn stack_pop_long(&mut self) -> u16 {
        let hb = self.stack_pop();
        let lb = self.stack_pop();
        to_word!(hb, lb)
    }

    pub fn stack_push(&mut self, data: u8) {
        let sp_addr = to_word!(0x01, self.sp);
        self.mem_write(sp_addr, data);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn stack_pop(&mut self) -> u8 {
        let sp_addr = to_word!(0x01, self.sp);
        let data = self.mem_read(sp_addr);
        self.sp = self.sp.wrapping_add(1);
        data
    }

    pub fn set_apumemory_ref(&mut self, memref: Arc<Mutex<ApuMemory>>) {
        self.memory = memref;
    }

    pub fn get_dp_address(&self, low_byte: u8) -> u16 {
        match self.status.contains(StatusWord::DPFlag) {
            false => to_word!(0x00, low_byte),
            true => to_word!(0x01, low_byte),
        }
    }

    fn carry(&self) -> u8 {
        match self.status.contains(StatusWord::Carry) {
            false => 0,
            true => 1,
        }
    }

    /// Returns `y` concatenated with `acc`, used by some instructions
    pub fn get_ya(&self) -> u16 {
        to_word!(self.y, self.acc)
    }

    pub fn set_ya(&mut self, val: u16) {
        self.acc = bit_slice!(val, 0, 7) as u8;
        self.y = bit_slice!(val, 8, 15) as u8;
    }

}