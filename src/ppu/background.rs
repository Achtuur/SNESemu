const SEVEN: u8 = 7;

pub struct Background {
    pub tilemap_vram_addr: u8,
    pub vertical_tilemap_count: u8,
    pub horizontal_tilemap_count: u8,
    pub chr_base_addr: u8,
    pub horizontal_scroll: u16,
    pub vertical_scroll: u16,
    pub enable_main: bool,
    pub enable_sub: bool,
}

impl Background {
    pub fn new() -> Background {
        Background {
            tilemap_vram_addr: 0,
            vertical_tilemap_count: 0,
            horizontal_tilemap_count: 0,
            chr_base_addr: 0,
            horizontal_scroll: 0,
            vertical_scroll: 0,
            enable_main: false,
            enable_sub: false,
        }
    }


    /// Write to BGxSC register ($2107 to 210A for BG 1-4)
    /// 
    /// `AAAA AAYX`
    /// 
    /// * Tilemap VRAM address (A)
    /// * vertical tilemap count (Y) 
    /// * horizontal tilemap count (X). 
    pub fn write_bgxsc_reg(&mut self, byte: u8) {
        self.tilemap_vram_addr = byte >> 2;
        self.vertical_tilemap_count = (byte >> 1) & 1;
        self.horizontal_tilemap_count = byte & 1;
    }

    /// Write to BGxyNBA register
    /// 
    /// Only lower bytes will be read so for BG2 and BG4
    /// this will have to be done outside this function
    pub fn write_base_address(&mut self, byte: u8) {
        self.chr_base_addr = byte & 0xF;
    }

    pub fn write_hscroll(&mut self, byte: u8, bglatch: u8) {
        let val = ((byte as u16) << 8) | 
        (bglatch & !SEVEN) as u16 | 
        (self.horizontal_scroll >> 8) & SEVEN as u16;
        self.horizontal_scroll = val & 0x3FF;
    }

    pub fn write_vscroll(&mut self, byte: u8, bglatch: u8) {
        let val = ((byte as u16) << 8) | bglatch as u16;

        self.vertical_scroll = val & 0x3FF;
    }
}
