use crate::{low_byte, high_byte, bank_byte, bit_set, bit_slice};

use self::{cgram::CgRam, oam::Oam, vram::Vram};

use super::components::{background::{Background, BackgroundLayer}, window::Window, mode7::Mode7, ppustate::PpuState, colormath::ColorMath};



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
    pub ppustate: PpuState,
    pub colormath: ColorMath,

    /// Multplication result of mode 7 A * mode 7 B
    pub mpy_mulres: u32,


    pub bg_latch: u8,
}

impl PpuMemory {
    pub fn new() -> PpuMemory {
        PpuMemory {
            vram: Vram::new(),
            cgram: CgRam::new(),
            bg1: Background::new(BackgroundLayer::Background1),
            bg2: Background::new(BackgroundLayer::Background2),
            bg3: Background::new(BackgroundLayer::Background3),
            bg4: Background::new(BackgroundLayer::Background4),
            w1: Window::new(),
            w2: Window::new(),
            mode7: Mode7::new(),
            oam: Oam::new(),
            ppustate: PpuState::new(),
            colormath: ColorMath::new(),
            bg_latch: 0,
            mpy_mulres: 0,
        }
    }

    pub fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {

            // Lower bytes of mul register
            0x2134 => Some(low_byte!(self.mpy_mulres)),
            // Middle bytes of mul register
            0x2135 => Some(high_byte!(self.mpy_mulres)),
            // Upper bytes of mul register
            0x2136 => Some(bank_byte!(self.mpy_mulres)),

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
            0x2139 | 0x213A => self.vram.read_register(addr),

            // CGRAM
            0x213B => self.cgram.read_register(addr),

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
            0x2100 => self.ppustate.write_inidisp(byte),

            // OAM
            0x2101..=0x2104 => self.oam.write(addr, byte),

            0x2105 => {
                self.ppustate.write_bgmode(byte);
                self.bg1.set_char_size(bit_set!(byte, 4));
                self.bg2.set_char_size(bit_set!(byte, 5));
                self.bg3.set_char_size(bit_set!(byte, 6));
                self.bg4.set_char_size(bit_set!(byte, 7));
            },

            // Mosaic
            0x2106 => {
                self.ppustate.write_mosaic(byte);
                self.bg1.set_mosaic(bit_set!(byte, 0));
                self.bg2.set_mosaic(bit_set!(byte, 1));
                self.bg3.set_mosaic(bit_set!(byte, 2));
                self.bg4.set_mosaic(bit_set!(byte, 3));
            }

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

            0x2115..=0x2119 => self.vram.write_register(addr, byte),


            // Mode 7 registers, writing to A and B also update multiplication
            0x211B | 0x211C => {
                self.mode7.write(addr, byte);
                self.set_mul_reg();
            }

            // Other mode 7 registers
            0x211A | 0x211D..=0x2120 => self.mode7.write(addr, byte),

            // CGRAM
            0x2121 | 0x2122 | 0x213B => self.cgram.write_register(addr, byte),

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
                self.bg1.write_masklogic(bit_slice!(byte, 0, 1));
                self.bg2.write_masklogic(bit_slice!(byte, 2, 3));
                self.bg3.write_masklogic(bit_slice!(byte, 4, 5));
                self.bg4.write_masklogic(bit_slice!(byte, 6, 7));
            }

            0x212B => self.ppustate.write_wobjlog(byte),

            // TM
            0x212C => {
                self.ppustate.write_tm(byte);
                self.bg1.set_enable_main(bit_set!(byte, 0));
                self.bg2.set_enable_main(bit_set!(byte, 1));
                self.bg3.set_enable_main(bit_set!(byte, 2));
                self.bg4.set_enable_main(bit_set!(byte, 3));
            },

            // TS
            0x212D => {
                self.ppustate.write_ts(byte);
                self.bg1.set_enable_sub(bit_set!(byte, 0));
                self.bg2.set_enable_sub(bit_set!(byte, 1));
                self.bg3.set_enable_sub(bit_set!(byte, 2));
                self.bg4.set_enable_sub(bit_set!(byte, 3));
            },

            // TMW
            0x212E => self.ppustate.write_tmw(byte),

            // TSW
            0x212F => self.ppustate.write_tsw(byte),

            // CGWSEL
            0x2130..=0x2132 => self.colormath.write_register(addr, byte),
            
            // SETINI / INISEL
            0x2133 => self.ppustate.write_inisel(byte),

            _ => {},
        }
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

            _ => unreachable!(),
        }
        self.bg_latch = byte;
    }
    

    fn set_mul_reg(&mut self) {
        self.mpy_mulres = self.mode7.a as u32 * high_byte!(self.mode7.b) as u32;
    }

}