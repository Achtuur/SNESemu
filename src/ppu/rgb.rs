use core::ops::{Add, Sub};

use crate::{bit_slice, nth_bit};

const MAX_RGB_VALUE: u8 = 0b11111;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba {r, g, b, a}
    }

    /// Return Rgba struct from word in SNES CGRAM
    pub fn from_snes_palette(word: u16) -> Rgba {
        Rgba {
            a: nth_bit!(word, 15) as u8,
            r: bit_slice!(word, 10, 14) as u8,
            g: bit_slice!(word, 5, 9) as u8,
            b: bit_slice!(word, 0, 4) as u8,
        }
    }

    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }
    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    /// Returns `self` as `(r, g, b)`
    pub fn as_rgb_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Returns `self` as `(r, g, b, a)`
    pub fn as_rgba_tuple(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn mul_rgb(&mut self, f: u8) {
        self.r = std::cmp::min(MAX_RGB_VALUE, self.r * f);
        self.g = std::cmp::min(MAX_RGB_VALUE, self.g * f);
        self.b = std::cmp::min(MAX_RGB_VALUE, self.b * f);
    }

    pub fn div_rgb(&mut self, d: u8) {
        if d == 0 {
            dbg!("Color::div was giving argument 0, skipping division");
        }
        self.r = std::cmp::min(MAX_RGB_VALUE, self.r / d);
        self.g = std::cmp::min(MAX_RGB_VALUE, self.g / d);
        self.b = std::cmp::min(MAX_RGB_VALUE, self.b / d);
    }
}


impl Default for Rgba {
    fn default() -> Rgba {
        Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}


impl Add for Rgba {
    type Output = Rgba;
    fn add(self, rhs: Self::Output) -> Self::Output {
        Rgba {
            r: std::cmp::min(MAX_RGB_VALUE, self.r + rhs.r),
            g: std::cmp::min(MAX_RGB_VALUE, self.g + rhs.g),
            b: std::cmp::min(MAX_RGB_VALUE, self.b + rhs.b),
            a: self.a,
        }
    }
}

impl Sub for Rgba {
    type Output = Rgba;
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Rgba {
            r: std::cmp::max(0, self.r - rhs.r),
            g: std::cmp::max(0, self.g - rhs.g),
            b: std::cmp::max(0, self.b - rhs.b),
            a: self.a
        }
    }
}