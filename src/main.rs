pub mod addr_macros;
mod apu;
pub mod bit_macros;
mod cpu;
mod ppu;
mod snes;
pub mod rgb;

use apu::memory::ApuMemory;

use pix_engine::engine::Engine;
use ppu::memory::PpuMemory;
use snes::{SnesApp, NTSC_SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::{apu::Apu, cpu::Cpu, ppu::Ppu};

#[macro_export]
/// `arc_mut!(x) = Arc::new(Mutex::new(x))`
macro_rules! arc_mut {
    ($obj:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($obj))
    };
}

fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    .init();

    let mut snes = SnesApp::new();
    snes.run().unwrap()
}
