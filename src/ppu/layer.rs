use super::rgb::Rgba;

/// Layer that the SNES PPU uses when drawing on screen.
/// Inner RGBA value representing value for this scanline's x and y pixel can be obtained using `Layer::inner_rgba()`
pub enum Layer {
    Sprite0(Rgba),
    Sprite1(Rgba),
    Sprite2(Rgba),
    Sprite3(Rgba),
    Bg1Low(Rgba),
    Bg1High(Rgba),
    Bg2Low(Rgba),
    Bg2High(Rgba),
    Bg3Low(Rgba),
    Bg3High(Rgba),
    Bg4Low(Rgba),
    Bg4High(Rgba),
}

impl Layer {
    /// Get rgba value that is contained in this Layer variant
    pub fn inner_rgba(&self) -> Rgba {
        match self {
            Layer::Sprite0(c) => *c,
            Layer::Sprite1(c) => *c,
            Layer::Sprite2(c) => *c,
            Layer::Sprite3(c) => *c,
            Layer::Bg1Low(c) => *c,
            Layer::Bg1High(c) => *c,
            Layer::Bg2Low(c) => *c,
            Layer::Bg2High(c) => *c,
            Layer::Bg3Low(c) => *c,
            Layer::Bg3High(c) => *c,
            Layer::Bg4Low(c) => *c,
            Layer::Bg4High(c) => *c,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
/// Struct containing RGB values for all the layers, meant for temporary construction to create a Vec<Layer>
pub struct LayerStruct {
    pub sprite0: Rgba,
    pub sprite1: Rgba,
    pub sprite2: Rgba,
    pub sprite3: Rgba,
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
        }
    } 

    /// Returns vector where layers are in order from front to back
    pub fn as_ordered_vec(&self, bgmode: usize, bg3prio: bool) -> Vec<Layer> {
        use Layer::*;
        match bgmode {
            0 => vec![
            Sprite3(self.sprite3), 
            Bg1High(self.bg1high),
            Bg2High(self.bg2high),
            Sprite2(self.sprite2),
            Bg1Low(self.bg1low),
            Bg2Low(self.bg2low),
            Sprite1(self.sprite1),
            Bg3High(self.bg3high),
            Bg4High(self.bg4high),
            Sprite0(self.sprite0),
            Bg3Low(self.bg3low),
            Bg4Low(self.bg4low),
            ],
            1 => {
                if bg3prio {
                    vec![
                    Bg3High(self.bg3high),
                    Sprite3(self.sprite3), 
                    Bg1High(self.bg1high),
                    Bg2High(self.bg2high),
                    Sprite2(self.sprite2),
                    Bg1Low(self.bg1low),
                    Bg2Low(self.bg2low),
                    Sprite1(self.sprite1),
                    Sprite0(self.sprite0),
                    Bg3Low(self.bg3low),
                    ]
                } else {
                    vec![
                    Sprite3(self.sprite3), 
                    Bg1High(self.bg1high),
                    Bg2High(self.bg2high),
                    Sprite2(self.sprite2),
                    Bg1Low(self.bg1low),
                    Bg2Low(self.bg2low),
                    Sprite1(self.sprite1),
                    Bg3High(self.bg3high),
                    Sprite0(self.sprite0),
                    Bg3Low(self.bg3low),
                    ]
                }
            },
            
            2 | 3 | 4 | 5 => vec![
            Sprite3(self.sprite3), 
            Bg1High(self.bg1high),
            Sprite2(self.sprite2),
            Bg2High(self.bg2high),
            Sprite1(self.sprite1),
            Bg1Low(self.bg1low),
            Sprite0(self.sprite0),
            Bg2Low(self.bg2low),
            ],
            
            6 => vec![
            Sprite3(self.sprite3), 
            Bg1High(self.bg1high),
            Sprite2(self.sprite2),
            Sprite1(self.sprite1),
            Bg1Low(self.bg1low),
            Sprite0(self.sprite0),
            ],
            
            7 => vec![
            Sprite3(self.sprite3),
            Sprite2(self.sprite2),
            Sprite1(self.sprite1),
            Bg1Low(self.bg1low),
            Sprite0(self.sprite0),
            ],
            _ => unreachable!()
        } 
    }
}