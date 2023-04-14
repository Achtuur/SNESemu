use core::ops::{Add, Sub};

const MAX_RGB_VALUE: u8 = 0b11111;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {r, g, b}
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

    pub fn mul(&mut self, f: u8) {
        self.r = std::cmp::min(MAX_RGB_VALUE, self.r * f);
        self.g = std::cmp::min(MAX_RGB_VALUE, self.g * f);
        self.b = std::cmp::min(MAX_RGB_VALUE, self.b * f);
    }

    pub fn div(&mut self, d: u8) {
        if d == 0 {
            dbg!("Color::div was giving argument 0, skipping division");
        }
        self.r = std::cmp::min(MAX_RGB_VALUE, self.r / d);
        self.g = std::cmp::min(MAX_RGB_VALUE, self.g / d);
        self.b = std::cmp::min(MAX_RGB_VALUE, self.b / d);
    }
}


impl Default for Color {
    fn default() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
        }
    }
}


impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self::Output) -> Self::Output {
        Color {
            r: std::cmp::min(MAX_RGB_VALUE, self.r + rhs.r),
            g: std::cmp::min(MAX_RGB_VALUE, self.g + rhs.g),
            b: std::cmp::min(MAX_RGB_VALUE, self.b + rhs.b),
        }
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Color {
            r: std::cmp::max(0, self.r - rhs.r),
            g: std::cmp::max(0, self.g - rhs.g),
            b: std::cmp::max(0, self.b - rhs.b),
        }
    }
}