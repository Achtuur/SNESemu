use std::{time::{Duration, Instant}, thread, sync::Mutex};
use lazy_static::lazy_static;
use pix_engine::{prelude::{Engine, PixResult, PixEngine, Color, Font}, state::PixState, shape::Point, random, line_};
use pix_engine::color;
use pix_engine::rgb;

use super::{SCREEN_WIDTH, NTSC_SCREEN_HEIGHT, Snes};

use crate::{ppu::rgb::Rgba, cpu::{NMI_PENDING, IRQ_PENDING}};

/// NTSC Clock frequency in MHz
const NTSC_CLOCK_FREQ: f64 = 21.447;
/// NTSC Clock preiod in microseconds
const NTSC_CLOCK_PER_US: f64 = 1.0 / 21.447;
/// PAL Clock frequency in MHz
const PAL_CLOCK_FREQ: f64 = 21.281;

impl Snes {

    /// Set pixels for a single scanline
    pub fn set_scanline(&mut self, scanline: usize, pix: &[Rgba]) {
        let i = scanline * SCREEN_WIDTH as usize;
        self.ppu.pixels[i..i+SCREEN_WIDTH as usize].clone_from_slice(pix);
    }

    /// Set all pixels at once
    pub fn set_pixels(&mut self, pix: &[Rgba]) {
        self.ppu.pixels.clone_from_slice(pix);
    }

    fn render_screen(&mut self, s: &mut PixState) {
        self.ppu.pixels.iter().enumerate().for_each(|(i, rgb)| {
            // Calculate x and y values and construct a Point
            let (x, y) = (i % SCREEN_WIDTH, i / SCREEN_WIDTH);
            let p = Point::new([x as i32, y as i32]);

            // Set color of pixel to be drawn
            let (r, g, b, a) = rgb.as_highrange_rgba_tuple();
            let c = color!(r, g, b, a);
            s.stroke(c);
            s.point(p);
        });
    }
}

impl PixEngine for Snes {
    // Set up application state and initial settings. `PixState` contains
    // engine specific state and utility methods for actions like getting mouse
    // coordinates, drawing shapes, etc. (Optional)
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {

        // reset components
        self.cpu.reset();
        self.ppu.reset();
        // self.apu.reset();

        // Set the background to GRAY and clear the screen.
        s.background(Color::BLACK);

        self.last_tick = Instant::now();

        // Returning `Err` instead of `Ok` would indicate initialization failed,
        // and that the application should terminate immediately.
        Ok(())
    }

    // Main update/render loop. Called as often as possible unless
    // `target_frame_rate` was set with a value. (Required)
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {

    
        // execute correct number of cycles

        let _ = self.cpu.tick();

        for _ in 0..3 {
            self.ppu.tick();
        }

        self.cycles += 1;

        // get time difference with previous loop

        let dt = self.last_tick - Instant::now();
        
        self.time_spent += (Instant::now()).duration_since(self.last_tick);

        let expected_time = Duration::from_secs_f64(NTSC_CLOCK_PER_US * self.cycles as f64);
        
        if self.time_spent > expected_time {
            // do extra cycles to catch up
            println!("Emulation behind by {:?}", self.time_spent - expected_time);
            
        } else if self.time_spent < expected_time {
            // sleep for a while
            thread::sleep(expected_time - self.time_spent);
        }
        
        self.last_tick = Instant::now();

        self.render_screen(s);

        Ok(())
    }

    // Clean up any state or resources before exiting such as deleting temporary
    // files or saving game state. (Optional)
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        // save sram data to file
        Ok(())
    }
}