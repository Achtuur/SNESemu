use std::sync::{Mutex, Arc};

use crate::{arc_mut, memory::Memory};

/// Picture processing unit handles visual stuff
pub struct Ppu {
    memory: Arc<Mutex<Memory>>,

}


impl Ppu {
    pub fn new() -> Self {
        Ppu {
            memory: arc_mut!(Memory::new()),
        }
    }

    /// Give Ppu a reference to memory shared between CPU, PPU and APU
    pub fn set_memory(&mut self, memory: Arc<Mutex<Memory>>) {
        self.memory = memory;
    }
}