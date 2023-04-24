use crate::to_word;

use super::masklogic::MaskLogic;

const SEVEN: u8 = 7;

pub struct Background {
    /// Base address of tilemap in VRAM
    pub tilemap_vram_addr: u16,
    /// Amount of tilemaps in "vertical" direction, is either `0` or `1` for 1 and 2 tilemaps resp.
    pub vertical_tilemap_count: u8,
    /// Amount of tilemaps in "horizontal" direction, is either `0` or `1` for 1 and 2 tilemaps resp.
    pub horizontal_tilemap_count: u8,
    /// Base address of character data (the actual sprites of the tiles)
    pub chr_base_addr: u16,
    /// Size of characters for this background, either `8` or `16`
    pub char_size: u16,
    /// Horizontal scroll
    pub scroll_x: u16,
    /// Vertical scroll
    pub scroll_y: u16,
    /// True if this background is enabled on the main screen
    pub enable_main: bool,
    /// True if this background is enabled on the sub screen
    pub enable_sub: bool,
    /// True is mosaic is enabled for this background
    pub mosaic: bool,
    /// Which mask logic to windows that apply to this background
    pub mask_logic: MaskLogic,
}

impl Background {
    pub fn new() -> Background {
        Background {
            tilemap_vram_addr: 0,
            vertical_tilemap_count: 0,
            horizontal_tilemap_count: 0,
            chr_base_addr: 0,
            char_size: 8,
            scroll_x: 0,
            scroll_y: 0,
            enable_main: false,
            enable_sub: false,
            mosaic: false,
            mask_logic: MaskLogic::from_bits(0),
        }
    }


    /// Write to BGxSC register ($2107 to 210A for BG 1-4)
    /// 
    /// `AAAA AAYX`
    /// 
    /// * Tilemap VRAM address (A) (highest 5 bits)
    /// * vertical tilemap count (Y) 
    /// * horizontal tilemap count (X). 
    pub fn write_bgxsc_reg(&mut self, byte: u8) {
        self.tilemap_vram_addr = (byte as u16) << 10;
        self.vertical_tilemap_count = (byte >> 1) & 1;
        self.horizontal_tilemap_count = byte & 1;
    }

    /// Write to BGxyNBA register
    /// 
    /// Only lower bytes will be read so for BG2 and BG4
    /// this will have to be done outside this function
    pub fn write_base_address(&mut self, byte: u8) {
        self.chr_base_addr = ((byte & 0xF) as u16) << 12;
    }

    pub fn write_hscroll(&mut self, byte: u8, bglatch: u8) {
        let ll = (bglatch & !SEVEN) as u16 | 
        (self.scroll_x >> 8) & SEVEN as u16;

        let val = to_word!(byte, ll);
        self.scroll_x = val & 0x3FF;
    }

    pub fn write_vscroll(&mut self, byte: u8, bglatch: u8) {
        let val = ((byte as u16) << 8) | bglatch as u16;

        self.scroll_y = val & 0x3FF;
    }

    /// Bits have to be the 2 bits from $212A that apply to this BG layer
    pub fn write_masklogic(&mut self, bits: u8) {
        self.mask_logic = MaskLogic::from_bits(bits);
    }

    pub fn set_mosaic(&mut self, mosaic: bool) {
        self.mosaic = mosaic;
    }

    pub fn set_enable_main(&mut self, enabled: bool) {
        self.enable_main = enabled;
    }

    pub fn set_enable_sub(&mut self, enabled: bool) {
        self.enable_sub = enabled;
    }

    /// Sets character size based on `size`
    /// 
    /// `size == 0` sets `self.char_size` to `8`
    /// 
    /// `size == 1` sets `self.char_size` to `16`
    pub fn set_char_size(&mut self, size: bool) {
        self.char_size = match size {
            false => 8,
            true => 16,
        }
    }
}
