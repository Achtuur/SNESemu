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

use self::{memory::PpuMemory, scanline::Scanline, rgb::Rgba, components::background::BackgroundLayer};

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
/// Total number of pixels on screen with NTSC
pub const NTSC_SCREEN_PIXELS: usize = SCREEN_WIDTH * NTSC_SCREEN_HEIGHT;

pub const PAL_SCREEN_HEIGHT: usize = 239;
/// Total number of pixels on screen with PAL
pub const PAL_SCREEN_PIXELS: usize = SCREEN_WIDTH * PAL_SCREEN_HEIGHT;
/// Picture processing unit handles visual stuff
pub struct SPpu {
    /// Pixels that should be drawn to the screen
    pub pixels: Vec<Rgba>,

    /// Pixels for background 1, without any color effects applied. 
    /// This vector should only contain the tiles read from tilemap in VRAM
    /// 
    /// Stored as `(pixel, priority)` tuple
    pub bg1_pixels: Vec<(Rgba, usize)>,

    /// Pixels for background 2, without any color effects applied. 
    /// This vector should only contain the tiles read from tilemap in VRAM
    /// 
    /// Stored as `(pixel, priority)` tuple
    pub bg2_pixels: Vec<(Rgba, usize)>,

    /// Pixels for background 3, without any color effects applied. 
    /// This vector should only contain the tiles read from tilemap in VRAM
    /// 
    /// Stored as `(pixel, priority)` tuple
    pub bg3_pixels: Vec<(Rgba, usize)>,

    /// Pixels for background 4, without any color effects applied. 
    /// This vector should only contain the tiles read from tilemap in VRAM
    /// 
    /// Stored as `(pixel, priority)` tuple
    pub bg4_pixels: Vec<(Rgba, usize)>,

    /// Pixels for object layer, without any color effects applied. 
    /// This vector should only contain the characters read from OAM
    /// 
    /// Stored as `(pixel, priority, palette)` tuple
    pub obj_pixels: Vec<(Rgba, usize, usize)>,

    memory: Arc<Mutex<PpuMemory>>,
    scanline: Scanline,
}


impl SPpu {
    pub fn new() -> Self {
        SPpu {
            pixels: Vec::<Rgba>::new(),
            scanline: Scanline::new(),
            memory: arc_mut!(PpuMemory::new()),
            bg1_pixels: Vec::new(),
            bg2_pixels: Vec::new(),
            bg3_pixels: Vec::new(),
            bg4_pixels: Vec::new(),
            obj_pixels: Vec::new(),
            
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