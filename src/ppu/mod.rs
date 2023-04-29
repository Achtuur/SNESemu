pub mod memory;
pub mod components;
pub mod rgb;
pub mod ppu;
pub mod sprite;
pub mod scanline;
pub mod tile;
pub mod layer;

use std::{sync::{Mutex, Arc, RwLock}, thread, time::Instant};
use std::time::Duration;

use crate::{arc_mut};
use lazy_static::lazy_static;
use pix_engine::prelude::{Engine, PixResult};

use self::{memory::PpuMemory, scanline::Scanline, rgb::Rgba};

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


pub const SCREEN_WIDTH: usize = 256;
pub const NTSC_SCREEN_HEIGHT: usize = 224;
pub const PAL_SCREEN_HEIGHT: usize = 239;
/// Picture processing unit handles visual stuff
pub struct SPpu {
    pub pixels: Vec<Rgba>,
    memory: Arc<Mutex<PpuMemory>>,
    scanline: Scanline,
}


impl SPpu {
    pub fn new() -> Self {
        SPpu {
            pixels: Vec::<Rgba>::new(),
            scanline: Scanline::new(),
            memory: arc_mut!(PpuMemory::new()),
        }
    }

    pub fn set_ppumemory_ref(&mut self, memref: Arc<Mutex<PpuMemory>>) {
        self.memory = memref;
    }

    /// Set pixel for a single position
    pub fn set_pixel(&mut self, x: usize, y: usize, pix: Rgba) {
        let i = y * SCREEN_WIDTH + x;
        self.pixels[i] = pix;
    }

}