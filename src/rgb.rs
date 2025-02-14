use core::ops::{Add, Sub};

use crate::{bit_slice, nth_bit};

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rgba {
    // r, g, b, a
    clr: [u8; 4],
    // r: u8,
    // g: u8,
    // b: u8,
    // a: u8,
}

impl Rgba {
    pub const MAX_RGB_VALUE: u8 = 0b11111;
    pub const BLACK: Rgba = Rgba::new(0, 0, 0, Self::MAX_RGB_VALUE);
    pub const RED: Rgba = Rgba::new(Self::MAX_RGB_VALUE, 0, 0, Self::MAX_RGB_VALUE);
    pub const GREEN: Rgba = Rgba::new(0, Self::MAX_RGB_VALUE, 0, Self::MAX_RGB_VALUE);
    pub const BLUE: Rgba = Rgba::new(0, 0, Self::MAX_RGB_VALUE, Self::MAX_RGB_VALUE);
    pub const WHITE: Rgba = Rgba::new(Self::MAX_RGB_VALUE, Self::MAX_RGB_VALUE, Self::MAX_RGB_VALUE, Self::MAX_RGB_VALUE);

    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba {
            clr: [r, g, b, a],
        }
    }

    /// Return Rgba struct from word in SNES CGRAM
    pub fn from_snes_palette(word: u16) -> Rgba {
        Self::new(
            bit_slice!(word, 10, 14) as u8, // r
            bit_slice!(word, 5, 9) as u8, // g
            bit_slice!(word, 0, 4) as u8, // b
            nth_bit!(word, 15) as u8, // a
        )
    }

    #[inline]
    pub const fn r(&self) -> u8 {
        self.clr[0]
    }

    #[inline]
    pub const fn g(&self) -> u8 {
        self.clr[1]
    }

    #[inline]
    pub const fn b(&self) -> u8 {
        self.clr[2]
    }

    #[inline]
    pub const fn a(&self) -> u8 {
        self.clr[3]
    }

    #[inline]
    pub fn set_r(&mut self, r: u8) {
        self.clr[0] = r;
    }

    #[inline]
    pub fn set_g(&mut self, g: u8) {
        self.clr[1] = g;
    }

    #[inline]
    pub fn set_b(&mut self, b: u8) {
        self.clr[2] = b;
    }
    #[inline]
    pub fn set_a(&mut self, a: u8) {
        self.clr[3] = a;
    }

    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        &self.clr
    }

    #[inline]
    pub const fn into_inner(self) -> [u8; 4] {
        self.clr
    }

    #[inline]
    pub const fn into_inner_highrange(self) -> [u8; 4] {
        [
            (255 * (self.r() as u16) / Self::MAX_RGB_VALUE as u16) as u8,
            (255 * (self.g() as u16) / Self::MAX_RGB_VALUE as u16) as u8,
            (255 * (self.b() as u16) / Self::MAX_RGB_VALUE as u16) as u8,
            (255 * (self.a() as u16) / Self::MAX_RGB_VALUE as u16) as u8,
        ]
    }

    /// Returns `self` as `(r, g, b)`
    #[inline]
    pub const fn as_rgb_tuple(&self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }

    /// Returns `self` as `(r, g, b, a)`
    #[inline]
    pub const fn as_rgba_tuple(&self) -> (u8, u8, u8, u8) {
        (self.r(), self.g(), self.b(), self.a())
    }

    /// Returns `self` as `(r, g, b, a)` using 8 bits per color instead of 5
    #[inline]
    pub fn as_highrange_rgba_tuple(&self) -> (u8, u8, u8, u8) {
        (
            (255 * (self.r() as u16) / Self::MAX_RGB_VALUE as u16) as u8,
            (255 * (self.g() as u16) / Self::MAX_RGB_VALUE as u16) as u8,
            (255 * (self.b() as u16) / Self::MAX_RGB_VALUE as u16) as u8,
            (255 * (self.a() as u16) / Self::MAX_RGB_VALUE as u16) as u8,
        )
    }

    pub fn multiply(&self, f: u8) -> Rgba {
        let r = std::cmp::min(Self::MAX_RGB_VALUE, self.r() * f);
        let g = std::cmp::min(Self::MAX_RGB_VALUE, self.g() * f);
        let b = std::cmp::min(Self::MAX_RGB_VALUE, self.b() * f);
        Rgba::new(r, g, b, self.a())
    }

    pub fn divide(&mut self, d: u8) -> Rgba {
        if d == 0 {
            dbg!("Color::div was giving argument 0, skipping division");
        }
        let r = std::cmp::min(Self::MAX_RGB_VALUE, self.r() / d);
        let g = std::cmp::min(Self::MAX_RGB_VALUE, self.g() / d);
        let b = std::cmp::min(Self::MAX_RGB_VALUE, self.b() / d);
        Rgba::new(r, g, b, self.a())
    }
}

impl Add for Rgba {
    type Output = Rgba;
    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::new(
            std::cmp::min(Self::MAX_RGB_VALUE, self.r() + rhs.r()),
            std::cmp::min(Self::MAX_RGB_VALUE, self.g() + rhs.g()),
            std::cmp::min(Self::MAX_RGB_VALUE, self.b() + rhs.b()),
            self.a(),
        )
    }
}

impl Sub for Rgba {
    type Output = Rgba;
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::new(
            std::cmp::max(0, self.r() - rhs.r()),
            std::cmp::max(0, self.g() - rhs.g()),
            std::cmp::max(0, self.b() - rhs.b()),
            self.a(),
        )
    }
}
