pub mod components;
pub mod layer;
pub mod memory;
pub mod ppu;
pub mod scanline;
pub mod sprite;
pub mod tile;

use std::sync::{Arc, Mutex, RwLock};

use crate::{arc_mut, snes::{NTSC_SCREEN_HEIGHT, SCREEN_WIDTH}};
use lazy_static::lazy_static;
use pix_engine::prelude::{Engine, PixResult};

use self::{memory::PpuMemory, scanline::Scanline};

lazy_static! {
    static ref V_BLANK: RwLock<bool> = RwLock::new(false);
    static ref H_BLANK: RwLock<bool> = RwLock::new(false);
    static ref F_BLANK: RwLock<bool> = RwLock::new(false);
}

#[macro_export]
macro_rules! fv_blanking {
    () => {{
        use $crate::ppu::{F_BLANK, V_BLANK};
        *V_BLANK.read().unwrap() || *F_BLANK.read().unwrap()
    }};
}

#[macro_export]
macro_rules! h_blanking {
    () => {{
        use $crate::ppu::H_BLANK;
        *H_BLANK.read().unwrap()
    }};
}

/// Picture processing unit handles visual stuff
pub struct Ppu {
    memory: Arc<Mutex<PpuMemory>>,
    scanline: Scanline,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            scanline: Scanline::new(),
            memory: arc_mut!(PpuMemory::new()),
        }
    }

    pub fn set_ppumemory_ref(&mut self, memref: Arc<Mutex<PpuMemory>>) {
        self.memory = memref;
    }
}
