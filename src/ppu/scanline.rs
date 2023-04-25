use super::{SCREEN_WIDTH, H_BLANK, NTSC_SCREEN_HEIGHT, V_BLANK};


const HOR_SCANLINES: usize = 338;
const NTSC_VER_SCANLINES: usize = 261;
const PAL_VER_SCANLINES: usize = 311;


/// This struct emulates the electron beam in a CRT tv,
/// keeping track of current x and y position of the beam and setting V and H blank accordingly
pub struct Scanline {
    pub x: usize,
    pub y: usize,
    pub scanline_sprites: usize,

}

impl Scanline {
    pub fn new() -> Scanline {
        Scanline {
            x: 0,
            y: 0,
            scanline_sprites: 0,
        }
    }

    /// Moves this scanline to next position
    pub fn goto_next(&mut self) {
        self.x += 1;

        if self.x >= SCREEN_WIDTH {
            // x > screen width means H_blank is on
            *H_BLANK.write().unwrap() = true;
        } else if self.x >= HOR_SCANLINES {
            // x > scanlines means h_blank turns off and x is at left of screen
            *H_BLANK.write().unwrap() = false;
            self.x = 0;

            // move to next scanline
            self.y += 1;
            self.scanline_sprites = 0;
        }

        // Same logic as x, but with Vblank instead
        if self.y >= NTSC_SCREEN_HEIGHT {
            *V_BLANK.write().unwrap() = true;
        } else if self.y >= NTSC_VER_SCANLINES {
            *V_BLANK.write().unwrap() = false;
            self.y = 0;
        }

    }
}