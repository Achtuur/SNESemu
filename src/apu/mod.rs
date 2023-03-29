use std::sync::{Mutex, Arc};

use crate::{arc_mut, memory::{Memory, self}};


/// Audio processing unit
pub struct Apu {
    memory: Arc<Mutex<Memory>>,

}


impl Apu {
    pub fn new() -> Self {
        Apu {
            memory: arc_mut!(Memory::new()),
        }
    }

    /// Give APU a reference to memory shared between CPU, PPU and APU
    pub fn set_memory(&mut self, memory: Arc<Mutex<Memory>>) {
        self.memory = memory;
    }
}