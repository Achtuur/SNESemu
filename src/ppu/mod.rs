pub mod memory;
pub mod background;
pub mod mode7;
pub mod window;
pub mod ppustate;
pub mod colormath;
pub mod color;

use std::sync::{Mutex, Arc, RwLock};

use crate::{arc_mut};
use lazy_static::lazy_static;

use self::memory::PpuMemory;


lazy_static! {
    static ref V_BLANK: RwLock<bool> = RwLock::new(false);
    static ref H_BLANK: RwLock<bool> = RwLock::new(false);
    static ref F_BLANK: RwLock<bool> = RwLock::new(false);
}

#[macro_export]
macro_rules! fv_blanking {
    () => {{
        use crate::ppu::{F_BLANK, V_BLANK};
        *V_BLANK.read().unwrap() || *F_BLANK.read().unwrap()
    }};
}

#[macro_export]
macro_rules! h_blanking {
    () => {{
        use crate::ppu::H_BLANK;
        *H_BLANK.read().unwrap()
    }};
}


/// Picture processing unit handles visual stuff
pub struct Ppu {
    memory: Arc<Mutex<PpuMemory>>,
}


impl Ppu {
    pub fn new() -> Self {
        Ppu {
            memory: arc_mut!(PpuMemory::new()),
        }
    }

    pub fn set_ppumemory_ref(&mut self, memref: Arc<Mutex<PpuMemory>>) {
        self.memory = memref;
    }
}