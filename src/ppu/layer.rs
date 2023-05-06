use super::rgb::Rgba;


pub enum BitDepth {
    TwoBpp,
    FourBpp,
    EightBpp,
    DirectColor,
}



#[derive(Debug, Copy, Clone)]
/// Layer that the SNES PPU uses when drawing on screen.
/// Inner RGBA value representing value for this scanline's x and y pixel can be obtained using `Layer::inner_rgba()`
pub enum Layer {
    /// Contains color for pixel and which palette is used, which is important for color math
    Sprite0(Rgba, usize),
    /// Contains color for pixel and which palette is used, which is important for color math
    Sprite1(Rgba, usize),
    /// Contains color for pixel and which palette is used, which is important for color math
    Sprite2(Rgba, usize),
    /// Contains color for pixel and which palette is used, which is important for color math
    Sprite3(Rgba, usize),
    Bg1Low(Rgba),
    Bg1High(Rgba),
    Bg2Low(Rgba),
    Bg2High(Rgba),
    Bg3Low(Rgba),
    Bg3High(Rgba),
    Bg4Low(Rgba),
    Bg4High(Rgba),
    /// This layer is only used when no other layer is non transparent for a pixel
    /// 
    /// Contains `#0000` in main screen and `colormath.fixed_color` for sub screen
    FallBack(Rgba),
}

impl Layer {
    /// Get rgba value that is contained in this Layer variant
    pub fn inner_rgba(&self) -> Rgba {
        match self {
            Layer::Sprite0(c, _) => *c,
            Layer::Sprite1(c, _) => *c,
            Layer::Sprite2(c, _) => *c,
            Layer::Sprite3(c, _) => *c,
            Layer::Bg1Low(c) => *c,
            Layer::Bg1High(c) => *c,
            Layer::Bg2Low(c) => *c,
            Layer::Bg2High(c) => *c,
            Layer::Bg3Low(c) => *c,
            Layer::Bg3High(c) => *c,
            Layer::Bg4Low(c) => *c,
            Layer::Bg4High(c) => *c,
            Layer::FallBack(c) => *c,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
/// Struct containing RGB values for all the layers, meant for temporary construction to create a Vec<Layer>
pub struct LayerStruct {
    pub sprite0: Rgba,
    pub sprite0_palette: usize,
    pub sprite1: Rgba,
    pub sprite1_palette: usize,
    pub sprite2: Rgba,
    pub sprite2_palette: usize,
    pub sprite3: Rgba,
    pub sprite3_palette: usize,
    pub bg1low: Rgba,
    pub bg2low: Rgba,
    pub bg3low: Rgba,
    pub bg4low: Rgba,
    pub bg1high: Rgba,
    pub bg2high: Rgba,
    pub bg3high: Rgba,
    pub bg4high: Rgba,
}


impl LayerStruct {
    pub fn new() -> LayerStruct {
        LayerStruct {
            sprite0: Rgba::default(),
            sprite1: Rgba::default(),
            sprite2: Rgba::default(),
            sprite3: Rgba::default(),
            bg1low: Rgba::default(),
            bg2low: Rgba::default(),
            bg3low: Rgba::default(),
            bg4low: Rgba::default(),
            bg1high: Rgba::default(),
            bg2high: Rgba::default(),
            bg3high: Rgba::default(),
            bg4high: Rgba::default(),
            sprite0_palette: 0,
            sprite1_palette: 0,
            sprite2_palette: 0,
            sprite3_palette: 0,
        }
    }
    
    pub fn get_highest_priority_layer(&self, bgmode: usize, bg3prio: bool, fallbackcolor: Rgba) -> Layer {
        let prio_vec = self.as_ordered_vec(bgmode, bg3prio);
        let layer = prio_vec.iter().find_map(|layer| {
            if layer.inner_rgba() != Rgba::default() {
                return Some(layer);
            }
            None
        });

        match layer {
            Some(l) => *l,
            None => Layer::FallBack(fallbackcolor),
        }
    }

    /// Returns vector where layers are in order from front to back
    fn as_ordered_vec(&self, bgmode: usize, bg3prio: bool) -> Vec<Layer> {
        use Layer::*;
        match bgmode {
            0 => vec![
            Sprite3(self.sprite3, self.sprite3_palette), 
            Bg1High(self.bg1high),
            Bg2High(self.bg2high),
            Sprite2(self.sprite2, self.sprite2_palette),
            Bg1Low(self.bg1low),
            Bg2Low(self.bg2low),
            Sprite1(self.sprite1, self.sprite1_palette),
            Bg3High(self.bg3high),
            Bg4High(self.bg4high),
            Sprite0(self.sprite0, self.sprite0_palette),
            Bg3Low(self.bg3low),
            Bg4Low(self.bg4low),
            ],
            1 => {
                if bg3prio {
                    vec![
                    Bg3High(self.bg3high),
                    Sprite3(self.sprite3, self.sprite3_palette), 
                    Bg1High(self.bg1high),
                    Bg2High(self.bg2high),
                    Sprite2(self.sprite2, self.sprite2_palette),
                    Bg1Low(self.bg1low),
                    Bg2Low(self.bg2low),
                    Sprite1(self.sprite1, self.sprite1_palette),
                    Sprite0(self.sprite0, self.sprite0_palette),
                    Bg3Low(self.bg3low),
                    ]
                } else {
                    vec![
                    Sprite3(self.sprite3, self.sprite3_palette), 
                    Bg1High(self.bg1high),
                    Bg2High(self.bg2high),
                    Sprite2(self.sprite2, self.sprite2_palette),
                    Bg1Low(self.bg1low),
                    Bg2Low(self.bg2low),
                    Sprite1(self.sprite1, self.sprite1_palette),
                    Bg3High(self.bg3high),
                    Sprite0(self.sprite0, self.sprite0_palette),
                    Bg3Low(self.bg3low),
                    ]
                }
            },
            
            2 | 3 | 4 | 5 => vec![
            Sprite3(self.sprite3, self.sprite3_palette), 
            Bg1High(self.bg1high),
            Sprite2(self.sprite2, self.sprite2_palette),
            Bg2High(self.bg2high),
            Sprite1(self.sprite1, self.sprite1_palette),
            Bg1Low(self.bg1low),
            Sprite0(self.sprite0, self.sprite0_palette),
            Bg2Low(self.bg2low),
            ],
            
            6 => vec![
            Sprite3(self.sprite3, self.sprite3_palette), 
            Bg1High(self.bg1high),
            Sprite2(self.sprite2, self.sprite2_palette),
            Sprite1(self.sprite1, self.sprite1_palette),
            Bg1Low(self.bg1low),
            Sprite0(self.sprite0, self.sprite0_palette),
            ],
            
            7 => vec![
            Sprite3(self.sprite3, self.sprite3_palette),
            Sprite2(self.sprite2, self.sprite2_palette),
            Sprite1(self.sprite1, self.sprite1_palette),
            Bg1Low(self.bg1low),
            Sprite0(self.sprite0, self.sprite0_palette),
            ],
            _ => unreachable!()
        } 
    }

    /// Enables/disables bg1 color based on `bg1_enabled` and whether this layer is inside the window
    /// 
    /// Disabling bg1 means setting `self.bg1high = self.bg1low = Rgba::default()`
    pub fn set_bg1_enabled(&mut self, bg1_enabled: bool, bg1_window_enabled: bool, inside_window: bool) {
        if !bg1_enabled || (bg1_window_enabled && inside_window) {
            self.bg1high = Rgba::default();
            self.bg1low = Rgba::default();
        }
    }

    pub fn set_bg2_enabled(&mut self, bg2_enabled: bool, bg1_window_enabled: bool, inside_window: bool) {
        if !bg2_enabled || (bg1_window_enabled && inside_window) {
            self.bg2high = Rgba::default();
            self.bg2low = Rgba::default();
        }
    }

    pub fn set_bg3_enabled(&mut self, bg3_enabled: bool, bg1_window_enabled: bool, inside_window: bool) {
        if !bg3_enabled || (bg1_window_enabled && inside_window) {
            self.bg3high = Rgba::default();
            self.bg3low = Rgba::default();
        }
    }

    pub fn set_bg4_enabled(&mut self, bg4_enabled: bool, bg1_window_enabled: bool, inside_window: bool) {
        if !bg4_enabled || (bg1_window_enabled && inside_window) {
            self.bg4high = Rgba::default();
            self.bg4low = Rgba::default();
        }
    }

    pub fn set_obj_enabled(&mut self, obj_enabled: bool, bg1_window_enabled: bool, inside_window: bool) {
        if !obj_enabled || (bg1_window_enabled && inside_window) {
            self.sprite0 = Rgba::default();
            self.sprite1 = Rgba::default();
            self.sprite2 = Rgba::default();
            self.sprite3 = Rgba::default();
        }
    }

}