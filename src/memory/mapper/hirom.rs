use crate::memory::{separate_bank_hhll_addr, cartridge::CartridgeParseError};

use super::Mappermode;

/// Size of one bank of ROM memory using HiROM mapping
const ROM_BANK_SIZE: usize = 0xFFFF;

/// Size of one SRAM bank
const SRAM_BANK_SIZE: usize = 0x2000;

/// HiROM mapper, more information can be found on [the snesdev wiki](https://snes.nesdev.org/wiki/Memory_map#HiROM)
pub struct HiROM {
    /// Save RAM
    /// 
    /// Mapped in banks `0x70` to `0x7D` from `0x0000` to `0x7FFF`
    /// 
    /// Todo: change this to a hashmap/lis or something more memory efficient
    sram: Vec<u8>,

    /// Number of SRAM banks, depends on sram size and mapper
    /// 
    /// In HiROM, `sram_banks = ram_size_bytes / 0x2000`
    sram_banks: usize,

    /// ROM read from the cartridge
    /// 
    /// Mapped in banks `0x80 - 0xFF` from `0x8000 - 0xFFFF`, mirrored in `0x00 - 0x7D` from `0x8000 - 0xFFFF`
    rom: Vec<u8>,

    /// Number of ROM banks, depends on rom size and mapper
    /// 
    /// In LoROM, `rom_banks = rom_size_bytes / 0xFFFF`
    rom_banks: usize,
}

impl HiROM {
    pub fn new() -> Self {
        HiROM { 
            sram: Vec::new(),
            rom: Vec::new(),
            sram_banks: 0,
            rom_banks: 0,
            
        }
    }

    /// Read a byte from SRAM addressing range
    pub fn read_sram(&self, long_addr: u32) -> Option<u8> {
        if let Some(i) = self.sram_index_from_long_addr(long_addr) {
            return Some(self.sram[i as usize]);
        }
        None
    }
    
    /// Write a byte to SRAM addressing range
    pub fn write_sram(&mut self, long_addr: u32, value: u8) {
        if let Some(i) = self.sram_index_from_long_addr(long_addr) {
            self.sram[i as usize] = value;
        }
    }
    
    /// Returns index for internal sram vector based on `long_addr`
    fn sram_index_from_long_addr(&self, long_addr: u32) -> Option<usize> {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        
        let bank_i = match bank {
            0x30..=0x3F => bank - 0x30,
            0xB0..=0xBF => bank - 0xB0,
            _ => unreachable!()
        } as usize;
        
        // Mirror bank index if needed
        // With size of 2kB and 8 kB bank_i is always 0, meaning every bank is a mirror of the first one
        let bank_i = bank_i % self.sram_banks;
        
        // Mirror hhll if needed (only applies to sram size of 2kB)
        // With HiROM, hhll cannot be mirrored, but it's kept in just in case :)
        let hhll_i = hhll as usize % self.get_sram_size();
        
        let i = bank_i * SRAM_BANK_SIZE + hhll_i;
        Some(i)
    }

    pub fn read_rom(&self, long_addr: u32) -> Option<u8> {
        // get $BB and $HHLL as separate numbers, to make range checking a bit easier
        let (bank, hi_lo_byte) = separate_bank_hhll_addr(long_addr);

        // get indices for bank and address
        let bank_i = match bank {
            0x00..=0x3F => bank,
            0x80..=0xBF => bank - 0x80,
            0xC0..=0xFF => bank - 0xC0,
            _ => return None,
        };

        let bank_i = bank_i as usize % self.rom_banks;

        // Assuming that ROM will always be at least 1 full bank, meaning no mirroring is needed for hhll bytes
        let hi_lo_byte_i = hi_lo_byte as usize;

        // calculate final index using bank index and hhll index
        let i = bank_i * ROM_BANK_SIZE + hi_lo_byte_i;

        Some(self.rom[i as usize])
    }
}

impl Mappermode for HiROM {

    fn read(&self, long_addr: u32) -> Option<u8> {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        match (bank, hhll) {

            // SRAM
            (0x30..=0x3F | 0xB0..=0xBF, 0x6000..=0x7FFF) => self.read_sram(long_addr),

            // ROM
            (0xC0..=0xFF, 0x0000..=0xFFFF) | // main rom (banks from $C0 to $FF)
            (0x00..=0x3F | 0x80..=0xBF, 0x8000..=0xFFFF) // mirrors
            => self.read_rom(long_addr),

            _ => None
        }
    }

    fn write(&mut self, long_addr: u32, value: u8) {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        match (bank, hhll) {

            // SRAM
            (0x70..=0x7D, 0x0000..=0x7FFF) => self.write_sram(long_addr, value),

            // ROM
            (0xC0..=0xFF, 0x0000..=0xFFFF) | // main rom (banks from $C0 to $FF)
            (0x00..=0x3F | 0x80..=0xBF, 0x8000..=0xFFFF) // mirrors
            => {},

            _ => {},
        }
    }

    fn copy_bytes_to_rom(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.rom.push(*byte);
        }
        self.rom_banks = std::cmp::max(1, self.get_rom_size() / ROM_BANK_SIZE);
    }

    fn get_rom_size(&self) -> usize {
        self.rom.len()
    }
    
    fn set_ram_size(&mut self, size: u8) {
        self.sram = vec![0; (1 << size) * 1024];
        self.sram_banks = std::cmp::max(1, self.get_sram_size() / SRAM_BANK_SIZE);
    }

    fn get_sram_size(&self) -> usize {
        self.sram.len()
    }

    fn get_memory_map_mode(&self) -> u8 {
        0x01
    }

    fn reset(&mut self) {
        self.rom = Vec::new();
        self.sram = Vec::new();
    }

    fn get_sram_bytes(&self) -> Vec<u8> {
        self.sram.clone()
    }

    fn copy_bytes_to_sram(&mut self, bytes: &[u8]) {
        self.sram = Vec::new();
        for byte in bytes {
            self.sram.push(*byte);
        }
    }
}