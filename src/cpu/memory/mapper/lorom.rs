use crate::cpu::memory::separate_bank_hhll_addr;

use super::{Mappermode};

/// Size of one bank of ROM memory using LoROM mapping
const ROM_BANK_SIZE: usize = 0x8000;

/// Size of one SRAM bank
const SRAM_BANK_SIZE: usize = 0x8000;

/// LoROM mapper, more information can be found on [the snesdev wiki](https://snes.nesdev.org/wiki/Memory_map#LoROM)
pub struct LoROM {
    /// Save RAM
    /// 
    /// Mapped in banks `0x70` to `0x7D` from `0x0000` to `0x7FFF`
    /// 
    /// From some basic research, SRAM is either 2kB, 8kB, 20kB or 40kB
    sram: Vec<u8>,

    /// Number of SRAM banks, depends on sram size and mapper
    /// 
    /// In LoROM, `sram_banks = ram_size_bytes / 0x8000`
    sram_banks: usize,
    
    /// ROM read from the cartridge
    /// 
    /// Mapped in banks `0x80 - 0xFF` from `0x8000 - 0xFFFF`, mirrored in `0x00 - 0x7D` from `0x8000 - 0xFFFF`
    rom: Vec<u8>,

    /// Number of ROM banks, depends on rom size and mapper
    /// 
    /// In LoROM, `rom_banks = rom_size_bytes / 0x8000`
    rom_banks: usize,
    
}

impl LoROM {
    pub fn new() -> Self {
        LoROM { 
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
            0x70..=0x7D => bank - 0x70,
            0xF0..=0xFF => bank - 0xF0,
            _ => return None,
        } as usize;
        
        // Mirror bank index if needed
        // With size of 2kB and 8 kB bank_i is always 0, meaning every bank is a mirror of the first one
        let bank_i = bank_i % self.sram_banks;
        
        // Mirror hhll if needed (only applies to sram size of 2kB)
        let hhll_i = hhll as usize % self.get_sram_size();
        
        let i = bank_i * SRAM_BANK_SIZE + hhll_i;
        Some(i)
    }
    
    pub fn read_rom(&self, long_addr: u32) -> Option<u8> {
        // get $BB and $HHLL as separate numbers, to make range checking a bit easier
        let (bank, hi_lo_byte) = separate_bank_hhll_addr(long_addr);
        
        // get indices for bank and address
        let bank_i = match bank {
            0x00..=0x7E => bank,
            0x80..=0xFF => bank - 0x80,
            _ => return None,
        } as usize;
        
        let bank_i = bank_i % self.rom_banks;
        
        let hi_lo_byte_i = match hi_lo_byte {
            0x0000..=0x7FFF => hi_lo_byte,
            0x8000..=0xFFFF => hi_lo_byte - 0x8000,
        } as usize;
        
        // calculate final index using bank index and hhll index
        let i = bank_i * ROM_BANK_SIZE + hi_lo_byte_i;
        
        // println!("${:#06X?} = rom[{:?}] = #{:#04X?}", long_addr, i, self.rom[i as usize]);
        Some(self.rom[i as usize])
    }
}

impl Mappermode for LoROM {
    
    fn read(&self, long_addr: u32) -> Option<u8> {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        match (bank, hhll) {
            // SRAM
            (0x70..=0x7D | 0xF0..=0xFF, 0x0000..=0x7FFF) => self.read_sram(long_addr),
            
            // ROM
            (0x00..=0x7D, 0x8000..=0xFFFF) | // upper Q1, Q2 mirrors
            (0x80..=0xFF, 0x8000..=0xFFFF) | // Actual rom in upper Q3, Q4
            (0xC0..=0xF0, 0x0000..=0x7FFF) => self.read_rom(long_addr),
            
            _ => None
        }
    }
    
    fn write(&mut self, long_addr: u32, value: u8) {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);
        match (bank, hhll) {
            // SRAM
            (0x70..=0x7D | 0xF0..=0xFF, 0x0000..=0x7FFF) => self.write_sram(long_addr, value),
            
            // ROM
            (0x00..=0x7D, 0x8000..=0xFFFF) | // upper Q1, Q2 mirrors
            (0x80..=0xFF, 0x8000..=0xFFFF) | // Actual rom in upper Q3, Q4
            (0xC0..=0xF0, 0x0000..=0x7FFF) =>  {}
            
            _ => {}
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
        0x00
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