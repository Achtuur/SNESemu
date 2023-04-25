use crate::{bit_set, bit_slice, ppu::{rgb::Rgba, layer::Layer}};

enum Region {
    Enabled,
    Window,
    WindowInverted,
    Disabled,
}

pub enum Addend {
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
    sub_transparent_region: Region,
    main_black_region: Region,
    operator_type: OperatorType,
    
    pub addend: Addend,
    pub direct_color_mode: bool,
    pub obj_math_enable: bool,
    pub bg_math_enable: [bool; 4],
    pub backdrop_enable: bool,
    pub fixed_color: Rgba,
}

impl ColorMath {
    pub fn new() -> ColorMath {
        ColorMath {
            direct_color_mode: false,
            addend: Addend::FixedColor,
            sub_transparent_region: Region::Enabled,
            main_black_region: Region::Enabled,
            obj_math_enable: false,
            bg_math_enable: [false; 4],
            backdrop_enable: false,
            operator_type: OperatorType::Add,
            fixed_color: Rgba::default(),
        }
    }

    /// Applies color math to input layer and outputs Rgba value for this pixel
    /// 
    /// If math is not enabled for a layer, it simply outputs the main screen layer's color
    /// 
    /// If one of the screen's data is transparent, the output color is equal to the non-transparent screen
    /// 
    /// If both are transparent, the output is black
    pub fn apply_math(&self, main_layer: Layer, sub_layer: Layer) -> Rgba {

        match (main_layer.inner_rgba() == Rgba::default(), sub_layer.inner_rgba() == Rgba::default()) {
            // Both are non transparent, move on to do math
            (false, false) => {},
            // Sub screen is transparent and main screen is non-transparent -> output main screen
            (false, true) => return main_layer.inner_rgba(),
            // Main screen is transparent and subscreen is non-transparent -> output sub screen
            (true, false) => return sub_layer.inner_rgba(),
            // Both screens transparent -> output black
            (true, true) => return Rgba::BLACK,
        };

        let math_enable = match main_layer {
            Layer::Bg1Low(_) | Layer::Bg1High(_) => self.bg_math_enable[0],

            Layer::Bg2Low(_) | Layer::Bg2High(_) => self.bg_math_enable[1],

            Layer::Bg3Low(_) | Layer::Bg3High(_) => self.bg_math_enable[2],

            Layer::Bg4Low(_) | Layer::Bg4High(_) => self.bg_math_enable[3],

            Layer::Sprite0(_, p) | Layer::Sprite1(_, p) | 
            Layer::Sprite2(_, p) | Layer::Sprite3(_, p) => {
                p >= 4 && p <= 7 && self.obj_math_enable
            },

            _ => unreachable!(),
        };

        if math_enable {
            self.add_colors(main_layer.inner_rgba(), sub_layer.inner_rgba())
        } else {
            main_layer.inner_rgba()
        }
    }

    fn add_colors(&self, c1: Rgba, c2: Rgba) -> Rgba {
        match self.operator_type {
            OperatorType::Add => c1 + c2,
            OperatorType::Sub => c1 - c2,
            OperatorType::Average => (c1 + c2).divide(2),
            OperatorType::SubHalf => (c1 - c2).divide(2),
        }
    }

    /// Apply `self.region` on color switch that is obtained from window logic for mainscreen
    /// 
    /// # Inputs
    /// 
    /// * `clr_w`: Whether the 'color' layer is inside the window
    /// 
    /// # Returns
    ///
    /// * `Region::Enabled => true`
    /// * `Region::Window => clr_window`
    /// * `Region::WindowInverted => !clr_window`
    /// * `Region::Disabled => false`
    pub fn apply_color_switch_main(&self, clr_w: bool) -> bool {
        match self.main_black_region {
            Region::Enabled => true,
            Region::Window => clr_w,
            Region::WindowInverted => !clr_w,
            Region::Disabled => false,
        }
    }

    /// Apply `self.region` on color switch that is obtained from window logic for subscreen
    /// 
    /// # Inputs
    /// 
    /// * `clr_w`: Whether the 'color' layer is inside the window
    /// 
    /// # Returns
    ///
    /// * `Region::Enabled => true`
    /// * `Region::Window => clr_window`
    /// * `Region::WindowInverted => !clr_window`
    /// * `Region::Disabled => false`
    pub fn apply_color_switch_sub(&self, arg: bool) -> bool {
        match self.sub_transparent_region {
            Region::Enabled => true,
            Region::Window => arg,
            Region::WindowInverted => !arg,
            Region::Disabled => false,
        }
    }

    pub fn write_register(&mut self, addr: u16, byte: u8) {
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
            self.bg_math_enable[i] = bit_set!(byte, i);
        }

        self.obj_math_enable = bit_set!(byte, 4);
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