use super::separate_bank_hhll_addr;

/// Total bytes in RAM
const RAM_SIZE: usize = 2 * 0xFFFF;

const RAM_BANK_SIZE: u16 = 0xFFFF;

/// CPU Ram
pub struct Ram {
    bytes: [u8; RAM_SIZE]
}

impl Ram {
    pub fn new() -> Self {
        Ram { 
            bytes: [0_u8; RAM_SIZE],
        }
    }

    /// Read a byte from RAM
    /// 
    /// # Important
    /// 
    /// It is assumed this function is only called from `Memory::read()` meaning `long_addr` is assumed to always be within valid range with mirrors.
    /// 
    /// Ie, if `hhll > $2000` then `bank == $7E | $7F` and the other way around with the mirrored sections
    pub fn read(&self, long_addr: u32) -> Option<u8> {
        let i = Self::index_from_long_addr(long_addr);
        match Self::index_from_long_addr(long_addr) {
            Some(i) => Some(self.bytes[i]),
            None => None,
        }
    }

    /// Write a byte to RAM
    /// 
    /// # Important
    /// 
    /// It is assumed this function is only called from `Memory::read()` meaning `long_addr` is assumed to always be within valid range with mirrors.
    /// 
    /// Ie, if `hhll > $2000` then `bank == $7E | $7F` and the other way around with the mirrored sections
    pub fn write(&mut self, long_addr: u32, value: u8) {
        match Self::index_from_long_addr(long_addr) {
            Some(i) => self.bytes[i] = value,
            None => {},
        }
    }

    /// Returns index for internal bytes array from given long_addr
    fn index_from_long_addr(long_addr: u32) -> Option<usize> {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        let bank_i = match bank {
            0x00..=0x3F | 0x7E | 0x80..=0xBF => 0_u16,
            0x7F => 1_u16,
            _ => return None,
        };

        let hhll_i = hhll;
        let i = (bank_i * RAM_BANK_SIZE + hhll_i) as usize;
        Some(i)
    }
}