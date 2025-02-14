use pix_engine::{color, error::Result, image::PixelFormat, rgb, shape::Point, state::PixState, texture::TextureId};
use pix_engine::prelude::{Color, PixEngine, PixResult};

use crate::rgb::Rgba;

use super::{NTSC_SCREEN_HEIGHT, SCREEN_WIDTH};

pub(crate) struct Screen {
    scanline: usize,
    pixels: Vec<Rgba>,
    texture: TextureId,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            scanline: 0,
            pixels: vec![Rgba::default(); NTSC_SCREEN_HEIGHT * SCREEN_WIDTH],
            texture: TextureId::default(),
        }
    }

    pub fn init(&mut self, s: &mut PixState) -> PixResult<()> {
        self.texture = s.create_texture(SCREEN_WIDTH as u32, NTSC_SCREEN_HEIGHT as u32, PixelFormat::Rgba)?;
        Ok(())
    }

    pub fn inc_scanline(&mut self) {
        self.scanline += 1;
        self.scanline %= NTSC_SCREEN_HEIGHT;
    }


    /// Set pixel for a single position
    pub fn set_pixel(&mut self, x: usize, y: usize, pix: Rgba) {
        let i = y * SCREEN_WIDTH + x;
        self.pixels[i] = pix;
    }

    /// Set pixels for a single scanline
    pub fn set_scanline(&mut self, scanline: usize, pix: &[Rgba]) {
        let i = scanline * SCREEN_WIDTH;
        self.pixels[i..i + SCREEN_WIDTH].clone_from_slice(pix);
    }

    /// Set all pixels at once
    pub fn set_pixels(&mut self, pix: &[Rgba]) {
        self.pixels.clone_from_slice(pix);
    }

    pub fn render_screen(&mut self, s: &mut PixState) -> PixResult<()> {
        let pixels = self.pixels.iter().flat_map(|rgb| {
            rgb.into_inner_highrange()
        })
        .collect::<Vec<_>>();

        s.set_texture_target(self.texture)?;
        s.background(color!(0, 0, 0, 255));
        s.clear_texture_target();

        s.update_texture(self.texture, None, &pixels, 4)?;
        s.texture(self.texture, None, None)?;
        Ok(())
        // self.pixels.iter().enumerate().for_each(|(i, rgb)| {
        //     // Calculate x and y values and construct a Point
        //     let (x, y) = (i % SCREEN_WIDTH, i / SCREEN_WIDTH);
        //     let p = Point::new([x as i32, y as i32]);

        //     // Set color of pixel to be drawn
        //     let (r, g, b, a) = rgb.as_highrange_rgba_tuple();
        //     let c = color!(r, g, b, a);
        //     s.stroke(c);
        //     s.point(p);
        // });
    }
}
