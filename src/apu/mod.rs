use std::sync::{Arc, Mutex};

use crate::arc_mut;

use self::memory::ApuMemory;

pub mod memory;


/// Audio processing unit
pub struct SApu {
    memory: Arc<Mutex<ApuMemory>>,
}


impl SApu {
    pub fn new() -> Self {
        SApu {
            memory: arc_mut!(ApuMemory::new()),
        }
    }

    pub fn set_apumemory_ref(&mut self, memref: Arc<Mutex<ApuMemory>>) {
        self.memory = memref;
    }

}