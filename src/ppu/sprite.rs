#[derive(Debug, Clone)]
pub struct Sprite {
    /// X position of sprite
    pub x: usize,
    /// Y position of sprite
    pub y: usize,
    /// Priority of sprite
    pub priority: usize,
    /// Index of tile in VRAM
    pub tile_index: usize,
    /// Palette used by this sprite
    pub palette: usize,
    /// If true, flips sprite vertically
    pub flip_v: bool,
    /// If true, flips sprite horizontally
    pub flip_h: bool,
    /// If true, sprite uses big size from OBJSEL, else uses small size
    pub big_size: bool,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            x: 0,
            y: 0,
            priority: 0,
            tile_index: 0,
            palette: 0,
            flip_v: false,
            flip_h: false,
            big_size: false,
        }
    }
}
