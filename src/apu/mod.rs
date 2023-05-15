use std::sync::{Arc, Mutex};

use crate::{arc_mut, bit_slice, to_word};

use self::memory::ApuMemory;

pub mod memory;
pub mod divider;
pub mod timer;


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
    pc: u8,
    /// Processor status word
    psw: u8,
    memory: Arc<Mutex<ApuMemory>>,
}


impl SApu {
    pub fn new() -> Self {
        SApu {
            acc: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0,
            psw: 0,            
            memory: arc_mut!(ApuMemory::new()),
        }
    }

    pub fn set_apumemory_ref(&mut self, memref: Arc<Mutex<ApuMemory>>) {
        self.memory = memref;
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