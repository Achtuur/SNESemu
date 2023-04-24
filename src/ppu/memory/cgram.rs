use crate::{fv_blanking, h_blanking};
const CGRAM_SIZE: usize = 0x100;
pub struct CgRam {
    bytes: [u16; CGRAM_SIZE],
    rw_count: bool,
    latch: u8,
    word_address: u8,
    is_fvhblanking: bool,
}

impl CgRam {
    pub fn new() -> CgRam {
        CgRam {
            bytes: [0; CGRAM_SIZE],
            rw_count: false,
            latch: 0,
            word_address: 0, // Emulates register $2121 (CGADD)
            is_fvhblanking: false,
        }
    }
    
    pub fn reset_rwcount(&mut self) {
        self.rw_count = false;
    }

    /// Read CGRAM from inside PPU
    pub fn read(&self, addr: u16) -> u16 {
        self.bytes[(addr as usize) % CGRAM_SIZE as usize]
    }
        
    /// Read CGRAM using registers in cpu address space
    pub fn read_register(&mut self, addr: u16) -> Option<u8> {
        
        match addr {
            // CGADD
            0x2121 => None,
            // CGDATA
            0x2122 => None,
            // CGDATAREAD
            0x213B => {
                match self.word_address {
                    0 => Some(0),
                    _ => self.read_data(),
                }
            },
            _ => unreachable!(),
        }
    }
    
    pub fn write_register(&mut self, addr: u16, byte: u8) {
        match addr {
            // CGADD
            0x2121 => self.write_word_address(byte),
            // CGDATA
            0x2122 => {
                match self.word_address {
                    0 => {},
                    _ => self.write_data(byte),
                }
            }
            // CGDATAREAD
            0x213B => {},
            _ => unreachable!(),
        }
    }

    fn write_word_address(&mut self, addr: u8) {
        if fv_blanking!() | h_blanking!() {
            self.word_address = addr;
            self.rw_count = false;
        }
    }


    fn read_data(&mut self) -> Option<u8> {
        if !fv_blanking!() | h_blanking!() {
            return None;
        }

        let read_byte = if self.rw_count {
            // if count == 1, return upper byte
            (self.bytes[self.word_address as usize] >> 8) as u8
        } else {
            // count == 0, return lower byte
            self.bytes[self.word_address as usize] as u8
        };
        self.rw_count = !self.rw_count;
        Some(read_byte)
    }

    fn write_data(&mut self, byte: u8) {
        if !fv_blanking!() | h_blanking!() {
            return;
        }

        if self.rw_count {
            // if count == 1, write to cgram using input byte and old byte stored in latch
            self.bytes[self.word_address as usize] = ((byte as u16) << 8) | self.latch as u16;
            self.word_address = self.word_address.wrapping_add(1);
        } else {
            // count == 0, set latch (lower byte) to input byte
            self.latch = byte;
        };
        self.rw_count = !self.rw_count;
    }
}