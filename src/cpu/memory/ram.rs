use crate::{separate_bank_hhll_addr, set_ll, set_bb, set_hh};

/// Total bytes in RAM
const RAM_SIZE: usize = 2 * 0xFFFF;

const RAM_BANK_SIZE: u16 = 0xFFFF;

/// CPU Ram
pub struct Ram {
    /// Raw ram bytes
    bytes: [u8; RAM_SIZE],
    /// pointer for use with WRAM access registers ($2180 to $2183)
    pointer: usize,
}

impl Ram {
    pub fn new() -> Self {
        Ram { 
            bytes: [0_u8; RAM_SIZE],
            pointer: 0,
        }
    }

    /// Read a byte from RAM
    /// 
    /// # Important
    /// 
    /// It is assumed this function is only called from `Memory::read()` meaning `long_addr` is assumed to always be within valid range with mirrors.
    /// 
    /// Ie, if `hhll > $2000` then `bank == $7E | $7F` and the other way around with the mirrored sections
    pub fn read(&mut self, long_addr: u32) -> Option<u8> {
        match long_addr {
            0x2180 => self.read_from_pointer(),
            0x2181..=0x2183 => None,
            _ => self.read_direct(long_addr),
        }
    }

    /// Write a byte to RAM
    /// 
    /// # Important
    /// 
    /// It is assumed this function is only called from `Memory::read()` meaning `long_addr` is assumed to always be within valid range with mirrors.
    /// 
    /// Ie, if `hhll > $2000` then `bank == $7E | $7F` and the other way around with the mirrored sections
    pub fn write(&mut self, long_addr: u32, byte: u8) {
        match long_addr {
            //WMDATA
            0x2180 => self.write_with_pointer(byte),
            //WMADDL
            0x2181 => self.pointer = set_ll!(self.pointer, byte) as usize,
            //WMADDM
            0x2182 => self.pointer = set_hh!(self.pointer, byte) as usize,
            //WMADDL
            0x2183 => self.pointer = set_bb!(self.pointer, byte) as usize,

            _ => self.write_direct(long_addr, byte),
        }
        
    }

    /// Returns index for internal bytes array from given long_addr
    fn index_from_long_addr(long_addr: u32) -> Option<usize> {
        let (bank, hhll) = separate_bank_hhll_addr!(long_addr);
        let bank_i = match bank {
            0x00..=0x3F | 0x7E | 0x80..=0xBF => 0_u16,
            0x7F => 1_u16,
            _ => return None,
        };

        let hhll_i = hhll;
        let i = (bank_i * RAM_BANK_SIZE + hhll_i) as usize;
        Some(i)
    }

    /// Read from pointer, which is a 17 bit number
    fn read_from_pointer(&mut self) -> Option<u8> {
        let val = self.bytes[self.pointer];
        self.pointer = self.pointer.wrapping_add(1);
        Some(val)
    }

    fn write_with_pointer(&mut self, byte: u8) {
        self.bytes[self.pointer] = byte;
        self.pointer = self.pointer.wrapping_add(1);
    }

    fn read_direct(&self, long_addr: u32) -> Option<u8> {
        let i = Self::index_from_long_addr(long_addr);
        match Self::index_from_long_addr(long_addr) {
            Some(i) => Some(self.bytes[i]),
            None => None,
        }
    }

    fn write_direct(&mut self, long_addr: u32, byte: u8) {
        match Self::index_from_long_addr(long_addr) {
            Some(i) => self.bytes[i] = byte,
            None => {},
        }
    }
}