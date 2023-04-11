use self::{cgram::CgRam, oam::Oam, vram::Vram};

use super::{background::Background, mode7::Mode7, window::Window};

mod cgram;
pub mod vram;
pub mod oam;

pub struct PpuMemory {
    pub vram: Vram,
    pub cgram: CgRam,
    pub bg1: Background,
    pub bg2: Background,
    pub bg3: Background,
    pub bg4: Background,
    pub w1: Window,
    pub w2: Window,
    pub mode7: Mode7,
    pub oam: Oam,
    
    pub inidisp: u8, //$2100
    pub objsel: u8, //$2101
    pub bgmode: u8,
    pub mosaic: u8,

    pub tm: u8,
    pub ts: u8,
    pub tmw: u8,
    pub tsw: u8,

    pub cgwsel: u8,
    pub cgadsub: u8,
    pub coldata: u8,
    pub setini: u8,

    /// Multplication result of mode 7 A * mode 7 B
    pub mpy_mulres: u32,


    pub bg_latch: u8,
}

impl PpuMemory {
    pub fn new() -> PpuMemory {
        PpuMemory {
            vram: Vram::new(),
            cgram: CgRam::new(),
            bg1: Background::new(),
            bg2: Background::new(),
            bg3: Background::new(),
            bg4: Background::new(),
            w1: Window::new(),
            w2: Window::new(),
            mode7: Mode7::new(),
            oam: Oam::new(),
            inidisp: 0,
            objsel: 0,
            bgmode: 0,
            mosaic: 0,
            tmw: 0,
            tsw: 0,
            bg_latch: 0,
            tm: 0,
            ts: 0,
            cgwsel: 0,
            cgadsub: 0,
            coldata: 0,
            setini: 0,
            mpy_mulres: 0,
        }
    }

    pub fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {

            // Lower bytes of mul register
            0x2134 => Some((self.mpy_mulres) as u8),
            // Middle bytes of mul register
            0x2135 => Some((self.mpy_mulres >> 8) as u8),
            // Upper bytes of mul register
            0x2136 => Some((self.mpy_mulres >> 16) as u8),

            // Software latch for H/V counter
            0x2137 => {
                // set current horizontal and vertical scanline
                // in $213C and $213D respectively
                todo!();
                None
            },

            // OAM
            0x2138 => self.oam.read(addr),

            // VRAM
            0x2139 | 0x213A => self.vram.read(addr),

            // CGRAM
            0x213B => self.cgram.read(addr),

            // horizontal scanline latch
            0x213C => todo!(),

            //vertical scanline latch
            0x213D => todo!(),

            // STAT77
            0x213E => todo!(),

            // STAT78
            // Also resets h/v scanline read var and counter_latch = 0
            0x213F => todo!(),


            _ => None,
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0x2100 => self.inidisp = byte,

            0x2101 => self.objsel = byte,

            // OAM
            0x2102 | 0x2103 | 0x2104 | 0x2138 => self.oam.write(addr, byte),

            0x2105 => self.set_bg_mode(byte),

            0x2106 => self.mosaic = byte,

            0x2107 => self.bg1.write_bgxsc_reg(byte),
            0x2108 => self.bg2.write_bgxsc_reg(byte),
            0x2109 => self.bg3.write_bgxsc_reg(byte),
            0x210A => self.bg4.write_bgxsc_reg(byte),

            0x210B => {
                self.bg1.write_base_address(byte & 0xF);
                self.bg2.write_base_address(byte >> 4);
            },

            0x210C => {
                self.bg3.write_base_address(byte & 0xF);
                self.bg4.write_base_address(byte >> 4);
            },

            0x210D..=0x2114 => self.set_bg_hvscroll(addr, byte),

            0x2115..=0x2119 => self.vram.write(addr, byte),


            // Mode 7 registers, writing to A and B also update multiplication
            0x211B | 0x211C => {
                self.mode7.write(addr, byte);
                self.set_mul_reg();
            }

            // Other mode 7 registers
            0x211A | 0x211D..=0x2120 => self.mode7.write(addr, byte),

            // CGRAM
            0x2121 | 0x2122 | 0x213B => self.cgram.write(addr, byte),

            0x2123 =>  {
                self.w1.write_12sel(byte);
                self.w2.write_12sel(byte >> 2);
            },

            0x2124 =>  {
                self.w1.write_34sel(byte);
                self.w2.write_34sel(byte >> 2);
            },

            0x2125 => {
                self.w1.write_objsel(byte);
                self.w2.write_objsel(byte);
            }

            0x2126 => self.w1.write_left_pos(byte),
            0x2127 => self.w1.write_right_pos(byte),
            0x2128 => self.w2.write_left_pos(byte),
            0x2129 => self.w2.write_right_pos(byte),

            0x212A => {
                self.w1.write_wbglog(byte);
                self.w2.write_wbglog(byte);
            }

            0x212B => {
                self.w1.write_wobjlog(byte);
                self.w2.write_wobjlog(byte);
            }

            // TM
            0x212C => self.tm = byte,

            // TS
            0x212D => self.ts = byte,

            // TMW
            0x212E => self.tmw = byte,

            // TSW
            0x212F => self.tsw = byte,

            // CGWSEL
            0x2130 => self.cgwsel = byte,

            // CGADSUB
            0x2131 => self.cgadsub = byte,

            // COLDATA
            0x2132 => self.coldata = byte,

            // SETINI
            0x2133 => self.setini = byte,

            _ => {},
        }
    }

    /// Set bg mode register, `byte` is byte written to `$2105`
    fn set_bg_mode(&mut self, byte: u8) {
        self.bgmode = byte
    }

    fn set_bg_hvscroll(&mut self, addr: u16, byte: u8) {
        match addr {
            0x210D => {
                self.bg1.write_hscroll(byte, self.bg_latch);
                self.mode7.write_hscroll(byte);
            }

            0x210E => {
                self.bg1.write_vscroll(byte, self.bg_latch);
                self.mode7.write_vscroll(byte); 
            }

            0x201F => self.bg2.write_hscroll(byte, self.bg_latch),
            0x2110 => self.bg2.write_vscroll(byte, self.bg_latch),

            0x2111 => self.bg3.write_hscroll(byte, self.bg_latch),
            0x2112 => self.bg3.write_vscroll(byte, self.bg_latch),

            0x2113 => self.bg4.write_hscroll(byte, self.bg_latch),
            0x2114 => self.bg4.write_vscroll(byte, self.bg_latch),
        }
        self.bg_latch = byte;
    }
    

    fn set_mul_reg(&mut self) {
        self.mpy_mulres = self.mode7.a as u32 * (self.mode7.b >> 8) as u32;
    }
}