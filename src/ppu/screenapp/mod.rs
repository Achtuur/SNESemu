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
            let (r, g, b) = rgb.as_rgb_tuple();
            let c = color!(r, g, b);
            s.stroke(c);
            // 
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

        let v = (0..SCREEN_WIDTH)
        .map(|_| Rgba::new(random!(255), random!(255), random!(255), 255))
        .collect::<Vec<Rgba>>();

        self.set_scanline(self.scanline, &v);
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