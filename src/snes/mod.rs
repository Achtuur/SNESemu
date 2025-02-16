mod screen;

use pix_engine::color;
use pix_engine::engine::Engine;
use pix_engine::rgb;
use pix_engine::{
    prelude::{Color, PixEngine, PixResult},
    random,
    shape::Point,
    state::PixState,
};
use screen::Screen;

use crate::apu::memory::ApuMemory;
use crate::apu::Apu;
use crate::arc_mut;
use crate::cpu::memory::CpuMemory;
use crate::cpu::Cpu;
use crate::ppu::memory::PpuMemory;
use crate::ppu::Ppu;

use super::{rgb::Rgba};

pub(crate) const SCREEN_WIDTH: usize = 256;
pub(crate) const NTSC_SCREEN_HEIGHT: usize = 224;
pub(crate) const PAL_SCREEN_HEIGHT: usize = 239;
pub struct SnesApp<'mem> {
    cpu: Cpu<'mem>,
    ppu: Ppu,
    apu: Apu,
    screen: Screen,
}

impl<'mem> SnesApp<'mem> {
    pub fn new() -> Self {
        SnesApp {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            apu: Apu::new(),
            screen: Screen::new(),
        }
    }

    pub fn run(&mut self) -> PixResult<()> {
        let mut engine = Engine::builder()
        .dimensions(SCREEN_WIDTH as u32, NTSC_SCREEN_HEIGHT as u32)
        .title("MyApp")
        .show_frame_rate()
        .resizable()
        // .target_frame_rate(60)
        .build()?;

        engine.run(self)
    }

    fn render_screen(&mut self, s: &mut PixState) -> PixResult<()> {
        self.screen.render_screen(s)
    }
}

impl PixEngine for SnesApp<'_> {
    // Set up application state and initial settings. `PixState` contains
    // engine specific state and utility methods for actions like getting mouse
    // coordinates, drawing shapes, etc. (Optional)
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        self.screen.init(s)?;

        let rom = include_bytes!("../../resources/rom/Super Mario World.smc");
        self.cpu.memory = CpuMemory::from_cartridge(rom)?;

        let ppumem = arc_mut!(PpuMemory::new());
        self.cpu.memory.set_ppumemory_ref(ppumem.clone());
        self.ppu.set_ppumemory_ref(ppumem);

        let apumem = arc_mut!(ApuMemory::new());
        self.cpu.memory.set_apumemory_ref(apumem.clone());
        self.apu.set_apumemory_ref(apumem);

        // go to reset state
        self.cpu.reset();
        // self.ppu.reset();
        // self.apu.reset();


        // Set the background to GRAY and clear the screen.
        s.background(Color::BLACK);

        // Returning `Err` instead of `Ok` would indicate initialization failed,
        // and that the application should terminate immediately.
        Ok(())
    }

    // Main update/render loop. Called as often as possible unless
    // `target_frame_rate` was set with a value. (Required)
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        self.cpu.tick(false, false);
        if let Some((x, y, clr)) = self.ppu.tick() {
            self.screen.set_pixel(x, y, clr);
        }

        self.render_screen(s)?;

        // let mut i: u16 = 0;
        // let mut j: u16 = 0;
        // let mut k: u16 = 0;
        // let mut l: u16 = 0;

        // let v = (0..SCREEN_WIDTH * NTSC_SCREEN_HEIGHT)
        //     .map(|_| {
        //         i = i.wrapping_add(1) % Rgba::MAX_RGB_VALUE as u16;
        //         j = (random!(10) + i * 10) % Rgba::MAX_RGB_VALUE as u16;
        //         k = (random!(1) + j * 6) % Rgba::MAX_RGB_VALUE as u16;
        //         l = (random!(8) + k * 7) % Rgba::MAX_RGB_VALUE as u16;
        //         Rgba::new(i as u8, j as u8, k as u8, 255)
        //     })
        //     .collect::<Vec<Rgba>>();

        // self.screen.set_pixels(&v);
        // self.screen.inc_scanline();
        Ok(())
    }

    // Clean up any state or resources before exiting such as deleting temporary
    // files or saving game state. (Optional)
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        // save sram data to file
        Ok(())
    }
}
