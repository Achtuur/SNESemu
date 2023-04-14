use crate::{bit_set, bit_slice, fv_blanking};

pub struct PpuState {
    pub force_blank: bool,
    pub brightness: u8,
    pub mosaic_size: u8,

    pub enable_obj_main: bool,
    pub enable_obj_sub: bool,

    pub enable_window_bg_main: [bool; 4],
    pub enable_window_bg_sub: [bool; 4],

    pub enabled_window_obj_main: bool,
    pub enabled_window_obj_sub: bool,

    pub overscan_enabled: bool,
    pub ph512_mode: bool,
    pub interlace: bool,
    pub obj_vertical_mode: bool,

    /// Background modes 0 to 7
    pub background_mode: u8,

    pub bg3_prio: bool,

    /// Background size per background.
    /// 
    /// False: 8x8, true: 16x16
    pub bg_size: [bool; 4],

}

impl PpuState {
    pub fn new() -> PpuState {
        PpuState {
            force_blank: false,
            brightness: 0,
            mosaic_size: 0,
            enable_window_bg_main: [false; 4],
            enable_window_bg_sub: [false; 4],
            enabled_window_obj_main: false,
            enabled_window_obj_sub: false,
            overscan_enabled: false,
            ph512_mode: false,
            interlace: false,
            obj_vertical_mode: false,
            enable_obj_main: false,
            enable_obj_sub: false,
            background_mode: 0,
            bg3_prio: false,
            bg_size: [false; 4],
        }
    }

    /// Write to `$2100`
    pub fn write_inidisp(&mut self, byte: u8) {
        if fv_blanking!() {
            self.force_blank = bit_set!(byte, 7);
            self.brightness = bit_slice!(byte, 0, 3);
        }
    }

    /// Write to `$2105`
    pub fn write_bgmode(&mut self, byte: u8) {
        if fv_blanking!() {
            self.background_mode = bit_slice!(byte, 0, 2);
            self.bg3_prio = bit_set!(byte, 3);
            for i in 0..4 {
                self.bg_size[i] = bit_set!(byte, i + 4);
            }
        }
        
    }

    /// Write to `$2106`
    pub fn write_mosaic(&mut self, byte: u8) {
        if fv_blanking!() {
            self.mosaic_size = bit_slice!(byte, 4, 7);
        }
    }

    /// Write to `$212C`
    pub fn write_tm(&mut self, byte: u8) {
        if fv_blanking!() {
            self.enable_obj_main = bit_set!(byte, 5);
        }
    }
    
    /// Write to `$212D`
    pub fn write_ts(&mut self, byte: u8) {
        if fv_blanking!() {
            self.enable_obj_sub = bit_set!(byte, 5);
        }
    }

    /// Write to `$2133`
    pub fn write_inisel(&mut self, byte: u8) {
        if fv_blanking!() {

        }
    }

    /// Write to `$212E`
    pub fn write_tmw(&mut self, byte: u8) {
        if fv_blanking!() {
            for i in 0..4 {
                self.enable_window_bg_main[i] = bit_set!(byte, i);
            }
            self.enabled_window_obj_main = bit_set!(byte, 5);
        }
    }
    
    /// Write to `$212F`
    pub fn write_tsw(&mut self, byte: u8) {
        if fv_blanking!() {
            for i in 0..4 {
                self.enable_window_bg_sub[i] = bit_set!(byte, i);
            }
            self.enabled_window_obj_sub = bit_set!(byte, 5)
        }
    }
}