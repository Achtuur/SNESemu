use pix_engine::{prelude::{Engine, PixResult, PixEngine, Color, Font}, state::PixState, shape::Point, random, line_};
use pix_engine::color;
use pix_engine::rgb;

use super::{SCREEN_WIDTH, NTSC_SCREEN_HEIGHT, rgb::Rgba};
pub struct ScreenApp {
    scanline: usize,
    pixels: Vec<Rgba>,
}

impl ScreenApp {
    pub fn new() -> Self {
        ScreenApp {
            scanline: 0,
            pixels: vec![Rgba::default(); (NTSC_SCREEN_HEIGHT * SCREEN_WIDTH) as usize],
        }
    }

    /// Set pixel for a single position
    pub fn set_pixel(&mut self, x: usize, y: usize, pix: Rgba) {
        let i = y * SCREEN_WIDTH + x;
        self.pixels[i] = pix;
    }

    /// Set pixels for a single scanline
    pub fn set_scanline(&mut self, scanline: usize, pix: &[Rgba]) {
        let i = scanline * SCREEN_WIDTH as usize;
        self.pixels[i..i+SCREEN_WIDTH as usize].clone_from_slice(pix);
    }

    /// Set all pixels at once
    pub fn set_pixels(&mut self, pix: &[Rgba]) {
        self.pixels.clone_from_slice(pix);
    }

    fn render_screen(&mut self, s: &mut PixState) {
        self.pixels.iter().enumerate().for_each(|(i, rgb)| {
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

impl PixEngine for ScreenApp {
    // Set up application state and initial settings. `PixState` contains
    // engine specific state and utility methods for actions like getting mouse
    // coordinates, drawing shapes, etc. (Optional)
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Set the background to GRAY and clear the screen.
        s.background(Color::BLACK);

        // Returning `Err` instead of `Ok` would indicate initialization failed,
        // and that the application should terminate immediately.
        Ok(())
    }

    // Main update/render loop. Called as often as possible unless
    // `target_frame_rate` was set with a value. (Required)
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {

        self.render_screen(s);

        let mut i: u16 = 0;
        let mut j: u16 = 0;
        let mut k: u16 = 0;
        let mut l: u16 = 0;

        let v = (0..SCREEN_WIDTH * NTSC_SCREEN_HEIGHT)
        .map(|_| {
            i = i.wrapping_add(1) % Rgba::MAX_RGB_VALUE as u16;
            j = (random!(10) + i * 10) % Rgba::MAX_RGB_VALUE as u16;
            k = (random!(1) + j * 6) % Rgba::MAX_RGB_VALUE as u16;
            l = (random!(8) + k * 7) % Rgba::MAX_RGB_VALUE as u16;
            Rgba::new(i as u8, j as u8, k as u8, l as u8)
        })
        .collect::<Vec<Rgba>>();

        // self.set_scanline(self.scanline, &v);
        self.set_pixels(&v);
        self.scanline += 1;
        self.scanline %= NTSC_SCREEN_HEIGHT as usize;
        Ok(())
    }

    // Clean up any state or resources before exiting such as deleting temporary
    // files or saving game state. (Optional)
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}