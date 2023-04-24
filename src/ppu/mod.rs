pub mod memory;
pub mod components;
pub mod rgb;
pub mod screenapp;
pub mod ppu;
pub mod sprite;
pub mod scanline;
pub mod tile;

use std::sync::{Mutex, Arc, RwLock};

use crate::{arc_mut};
use lazy_static::lazy_static;
use pix_engine::prelude::{Engine, PixResult};

use self::{memory::PpuMemory, screenapp::ScreenApp, scanline::Scanline};

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


const SCREEN_WIDTH: usize = 256;
const NTSC_SCREEN_HEIGHT: usize = 224;
const PAL_SCREEN_HEIGHT: usize = 239;
/// Picture processing unit handles visual stuff
pub struct Ppu {
    screen: ScreenApp,
    memory: Arc<Mutex<PpuMemory>>,
    scanline: Scanline,
}


impl Ppu {
    pub fn new() -> Self {
        Ppu {
            screen: ScreenApp::new(),
            scanline: Scanline::new(),
            memory: arc_mut!(PpuMemory::new()),
        }
    }

    pub fn set_ppumemory_ref(&mut self, memref: Arc<Mutex<PpuMemory>>) {
        self.memory = memref;
    }

    pub fn run(&mut self) -> PixResult<()> {
        let mut engine = Engine::builder()
        .dimensions(SCREEN_WIDTH as u32, NTSC_SCREEN_HEIGHT as u32)
        .title("MyApp")
        .show_frame_rate()
        .resizable()
        // .target_frame_rate(60)
        .build()?;

        engine.run(&mut self.screen)

    }
}