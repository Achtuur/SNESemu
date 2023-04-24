use crate::{nth_bit, bit_slice, bit_set};

/// Representation for a single tile of bg, 
/// used as an abstraction layer for the data held in VRAM for a single tile
pub struct Tile {
    pub v_flip: bool,
    pub h_flip: bool,
    pub prio: bool,
    pub palette: u8,
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
            v_flip: bit_set!(word, 15),
            h_flip: bit_set!(word, 14),
            prio: bit_set!(word, 13),
            palette: bit_slice!(word, 10, 12) as u8,
            tile_num: bit_slice!(word, 0, 9),
        }
    }
}
