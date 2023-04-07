pub struct Background {
    pub tilemap_vram_addr: u8,
    pub vertical_tilemap_count: u8,
    pub horizontal_tilemap_count: u8,
    pub chr_base_addr: u8,
    pub horizontal_scroll: u8,
    pub vertical_scroll: u8,
    pub enable_main: bool,
    pub enable_sub: bool,
    pub window_enable_main: bool,
    pub window_enable_sub: bool,
    
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
            window_enable_main: false,
            window_enable_sub: false,
        }
    }
}
