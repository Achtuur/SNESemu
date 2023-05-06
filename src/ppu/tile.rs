use crate::{nth_bit, bit_slice, bit_set};

/// Representation for a single tile of bg, 
/// used as an abstraction layer for the data held in VRAM for a single tile
pub struct Tile {
    /// True if the character this tile represents should be flipped vertically
    pub flip_v: bool,
    /// True if the character this tile represents should be flipped horizontally
    pub flip_h: bool,
    /// Priority of this tile, either 0 or 1
    pub priority: usize,
    /// Palette this tile uses for use with 2BPP and 4BPP
    pub palette: u8,
    /// Tile number, index in Tilemap
    pub tile_num: u16,
}

impl Tile {
    /// Initialize with word from VRAM that represents a tile
    /// `vhopppcc cccccccc`
    /// * `v`: vertical flip
    /// * `h`: horizontal flip
    /// * `o`: tile priority
    /// * `ppp`: tile palette
    /// * `cc cccccccc`: tile number (address offset for character)
    pub fn new(word: u16) -> Tile {
        Tile {
            flip_v: bit_set!(word, 15),
            flip_h: bit_set!(word, 14),
            priority: nth_bit!(word, 13) as usize,
            palette: bit_slice!(word, 10, 12) as u8,
            tile_num: bit_slice!(word, 0, 9),
        }
    }
}
