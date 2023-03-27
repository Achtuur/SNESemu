use crate::memory::separate_bank_hhll_addr;

use super::Mappermode;

/// Size of one bank of ROM memory using HiROM mapping
const HIROM_BANK_SIZE: u16 = 0xFFFF - 0x8000;

const ROM_SIZE: usize = 0x7FFF * 0x7F;

/// Size of one SRAM bank
const SRAM_BANK_SIZE: u16 = 0x7FFF;

/// Number of bytes in SRAM
const SRAM_SIZE: usize = 0x7FFF * 0xE;

/// HiROM mapper, more information can be found on [the snesdev wiki](https://snes.nesdev.org/wiki/Memory_map#HiROM)
pub struct HiROM {
    /// Save RAM
    /// 
    /// Mapped in banks `0x70` to `0x7D` from `0x0000` to `0x7FFF`
    /// 
    /// Todo: change this to a hashmap/lis or something more memory efficient
    sram: [u8; SRAM_SIZE],

    /// ROM read from the cartridge
    /// 
    /// Mapped in banks `0x80 - 0xFF` from `0x8000 - 0xFFFF`, mirrored in `0x00 - 0x7D` from `0x8000 - 0xFFFF`
    rom: [u8; ROM_SIZE],
}

impl HiROM {
    pub fn new() -> Self {
        HiROM { 
            sram: [0_u8; SRAM_SIZE],
            rom: [0_u8; ROM_SIZE],
        }
    }

    pub fn read_sram(&self, long_addr: u32) -> u8 {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        let i = bank as u16 * SRAM_BANK_SIZE + hhll;
        self.sram[i as usize]
    }

    pub fn write_sram(&mut self, long_addr: u32, value: u8) {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        let i = bank as u16 * SRAM_BANK_SIZE + hhll;
        self.sram[i as usize] = value;
    }

    pub fn read_rom(&self, long_addr: u32) -> u8 {
        // get $BB and $HHLL as separate numbers, to make range checking a bit easier
        let (bank, hi_lo_byte) = separate_bank_hhll_addr(long_addr);

        // get indices for bank and address
        let bank_i = match bank {
            0x00..=0x7E => bank,
            0x80..=0xFF => bank - 0x80,
            _ => panic!("Bank out of range when reading memory with HiROM")
        };

        let hi_lo_byte_i = match hi_lo_byte {
            0x8000..=0xFFFF => hi_lo_byte - 0x8000,
            _ => panic!("Address out of range when reading memory with HiROM")
        };

        // final index is bank_i * 7FFF + addr_i since every ROM bank in HiROM has a size of 0xFFFF - 0x8000
        let i = bank_i as u16 * HIROM_BANK_SIZE + hi_lo_byte_i;

        self.rom[i as usize]
    }
}

impl Mappermode for HiROM {

    fn read(&self, long_addr: u32) -> u8 {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        match (bank, hhll) {

            // SRAM
            (0x30..=0x3F | 0xB0..=0xBF, 0x6000..=0x7FFF) => self.read_sram(long_addr),

            // ROM
            (0x00..=0x7D | 0x80..=0xFF, 0x8000..=0xFFFF) => self.read_rom(long_addr),

            _ => panic!("Invalid memory address read in HiROM")
        }
    }

    fn write(&mut self, long_addr: u32, value: u8) {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        match (bank, hhll) {

            // SRAM
            (0x70..=0x7D, 0x0000..=0x7FFF) => self.write_sram(long_addr, value),

            // ROM
            (0x00..=0x7D | 0x80..=0xFF, 0x8000..=0xFFFF) => panic!("Attempting to write to ROM (HiROM)"),

            _ => panic!("Invalid memory address read in HiROM")
        }
    }
}