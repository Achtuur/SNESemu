use crate::{bit_set, bit_slice};

use super::color::Color;

enum Region {
    Enabled,
    Window,
    WindowInverted,
    Disabled,
}

enum Addend {
    FixedColor,
    Subscreen,
}

enum OperatorType {
    /// Main + Sub
    Add,
    /// Main - Sub
    Sub,
    /// (Main + Sub) / 2
    Average,
    /// (Main - Sub) / 2
    SubHalf,
}

pub struct ColorMath {
    direct_color_mode: bool,
    addend: Addend,
    sub_transparent_region: Region,
    main_black_region: Region,

    obj_color_enable: bool,
    bg_color_enable: [bool; 4],
    backdrop_enable: bool,
    operator_type: OperatorType,

    fixed_color: Color,
}

impl ColorMath {
    pub fn new() -> ColorMath {
        ColorMath {
            direct_color_mode: false,
            addend: Addend::FixedColor,
            sub_transparent_region: Region::Enabled,
            main_black_region: Region::Enabled,
            obj_color_enable: false,
            bg_color_enable: [false; 4],
            backdrop_enable: false,
            operator_type: OperatorType::Add,
            fixed_color: Color::default(),
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            //CGWSEL
            0x2130 => self.write_cgwsel(byte),
            //CGADSUB
            0x2131 => self.write_cgadsub(byte),
            //COLDATA
            0x2132 => self.write_coldata(byte),
            _ => {},
        }
    }

    fn write_cgwsel(&mut self, byte: u8) {
        self.direct_color_mode = bit_set!(byte, 0);

        self.addend = match bit_set!(byte, 1) {
            false => Addend::FixedColor,
            true => Addend::Subscreen
        };

        self.sub_transparent_region = match bit_slice!(byte, 4, 5) {
            0b00 => Region::Enabled,
            0b01 => Region::Window,
            0b10 => Region::WindowInverted,
            0b11 => Region::Disabled,
            _ => unreachable!(),
        };

        self.main_black_region = match bit_slice!(byte, 4, 5) {
            0b00 => Region::Enabled,
            0b01 => Region::Window,
            0b10 => Region::WindowInverted,
            0b11 => Region::Disabled,
            _ => unreachable!(),
        };
    }

    fn write_cgadsub(&mut self, byte: u8) {
        for i in 0..4 {
            self.bg_color_enable[i] = bit_set!(byte, i);
        }

        self.obj_color_enable = bit_set!(byte, 4);
        self.backdrop_enable = bit_set!(byte, 5);
        self.operator_type = match bit_slice!(byte, 6, 7) {
            0b00 => OperatorType::Add,
            0b01 => OperatorType::Sub,
            0b10 => OperatorType::Average,
            0b11 => OperatorType::SubHalf,
            _ => unreachable!(),
        }
    }

    fn write_coldata(&mut self, byte: u8) {
        let c = bit_slice!(byte, 0, 4);
        if bit_set!(byte, 5) {
            self.fixed_color.set_r(c);
        }
        if bit_set!(byte, 6) {
            self.fixed_color.set_g(c);
        }
        if bit_set!(byte, 7) {
            self.fixed_color.set_b(c);
        }
    }

}