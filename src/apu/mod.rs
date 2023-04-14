use std::sync::{Arc, Mutex};

use crate::arc_mut;

use self::memory::ApuMemory;

pub mod memory;


/// Audio processing unit
pub struct Apu {
    memory: Arc<Mutex<ApuMemory>>,
}


impl Apu {
    pub fn new() -> Self {
        Apu {
            memory: arc_mut!(ApuMemory::new()),
        }
    }

    pub fn set_apumemory_ref(&mut self, memref: Arc<Mutex<ApuMemory>>) {
        self.memory = memref;
    }

}