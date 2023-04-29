pub mod snes;

use std::time::{Duration, Instant};

use pix_engine::prelude::Engine;

use crate::{cpu::{SCpu, memory::cartridge::CartridgeParseError}, ppu::{SPpu, memory::PpuMemory, SCREEN_WIDTH, NTSC_SCREEN_HEIGHT}, apu::{SApu, memory::ApuMemory}, arc_mut};

#[derive(Debug)]
pub enum SnesRunError {
    NoCartridgeInserted,
}

pub struct Snes {
    cpu: SCpu,
    ppu: SPpu,
    apu: SApu,
    last_tick: Instant,
    cycles: u128,
    time_spent: Duration,
}

impl Snes {
    pub fn new() -> Snes {
        let mut snes = Snes {
            cpu: SCpu::new(),
            ppu: SPpu::new(),
            apu: SApu::new(),
            last_tick: Instant::now(),
            cycles: 0,
            time_spent: Duration::from_millis(0),
        };

        snes.create_ppu_memory();
        snes.create_apu_memory();

        snes
    }

    fn create_ppu_memory(&mut self) {
        let ppumem = arc_mut!(PpuMemory::new());
        self.cpu.memory.set_ppumemory_ref(ppumem.clone());
        self.ppu.set_ppumemory_ref(ppumem);
    }

    fn create_apu_memory(&mut self) {
        let apumem = arc_mut!(ApuMemory::new());
        self.cpu.memory.set_apumemory_ref(apumem.clone());
        self.apu.set_apumemory_ref(apumem);
    }

    pub fn insert_cartridge(&mut self, rom: &[u8]) -> Result<(), CartridgeParseError> {
        self.cpu.memory.insert_cartridge(rom)
    }

    /// Start main loop of SNES, which starts the pixengine app and plays the game currently loaded
    pub fn run(&mut self) -> Result<(), SnesRunError> {
        if self.cpu.memory.cartridge_metadata.title.is_none() {
            return Err(SnesRunError::NoCartridgeInserted);
        }

        let mut engine = Engine::builder()
        .dimensions(SCREEN_WIDTH as u32, NTSC_SCREEN_HEIGHT as u32)
        .title("MyApp")
        .show_frame_rate()
        .resizable()
        // .target_frame_rate(60)
        .build();

        engine.unwrap().run(self);

        Ok(())
    }
}