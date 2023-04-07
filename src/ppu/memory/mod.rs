use self::{cgram::CgRam, oam::Oam};

use super::{background::Background, mode7::Mode7};

mod cgram;
pub mod vram;
pub mod oam;

pub struct PpuMemory {
    cgram: CgRam,
    bg1: Background,
    bg2: Background,
    bg3: Background,
    bg4: Background,
    mode7: Mode7,
    oam: Oam,
    
    inidisp: u8, //$2100
    objsel: u8, //$2101
    bgmode: u8,
    mosaic: u8,
}

impl PpuMemory {
    pub fn new() -> PpuMemory {
        PpuMemory {
            cgram: CgRam::new(),
            bg1: Background::new(),
            bg2: Background::new(),
            bg3: Background::new(),
            bg4: Background::new(),
            mode7: Mode7::new(),
            oam: Oam::new(),
            inidisp: 0,
            objsel: 0,
            bgmode: 0,
            mosaic: 0,
        }
    }

    pub fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            // OAM
            0x2102 | 0x2103 | 0x2104 | 0x2138 => self.oam.read(addr),

            // CGRAM
            0x2121 | 0x2122 | 0x213B => self.cgram.read(addr),

            // work RAM
            0x2180 | 0x2181 | 0x2182 | 0x2183 => ,

            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0x2121 | 0x2122 | 0x213B => self.cgram.write(addr, byte),

            _ => unreachable!(),
        }
    }
}