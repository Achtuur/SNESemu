
use std::sync::{Mutex, Arc};

use crate::{ppu::memory::PpuMemory, apu::memory::ApuMemory};

use self::{mapper::{Mappermode, lorom::LoROM, hirom::HiROM, exhirom::ExHiROM}, cartridge::{CartridgeParseError, CartridgeMetadata}, ram::Ram};


mod mapper;
mod ram;
mod cartridge;

/// Struct that represents all memory in the SNES, this is shared between CPU, PPU and APU as they all need to read/write to it
/// 
/// Addresses in the SNES are represented by `$BBHHLL`, where `BB` is the bank byte, `HH` is the high byte and `LL` is the low byte.
/// 
/// Quadrants are also specified in the same way [this video](https://www.youtube.com/watch?v=-U76YvWdnZM) explaning mapping does.
/// 
/// Since rust does not support `u24` natively, `u32` will be used instead and the upper 8 bits are unused and ignored
/// 
/// Source used: [snes wiki page](https://snes.nesdev.org/wiki/Memory_map)
pub struct CpuMemory {
    mapper: Box<dyn Mappermode>,
    pub cartridge_metadata: CartridgeMetadata,
    ram: Ram,
    ppu_memory: Arc<Mutex<PpuMemory>>,
    apu_memory: Arc<Mutex<ApuMemory>>,
}

impl CpuMemory {
    pub fn new() -> Self {
        CpuMemory {
            // default to lorom mapper, this should probably be changed to be an Option<Box<dyn Mappermode>>
            mapper: Box::new(LoROM::new()),
            cartridge_metadata: CartridgeMetadata::new(),
            ram: Ram::new(),
            ppu_memory: Arc::new(Mutex::new(PpuMemory::new())),
            apu_memory: Arc::new(Mutex::new(ApuMemory::new())),
        }
    }

    /// Insert cartridge means copying raw bytes to mapper and parsing cartridge info
    pub fn insert_cartridge(&mut self, raw_bytes: &[u8]) -> Result<(), CartridgeParseError> {

        // try parse with lorom
        let mut mapper = LoROM::new();
        if let Ok(()) = self.try_parse(&mut mapper, raw_bytes) { 
            self.mapper = Box::new(mapper);
            return Ok(()) 
        }
        // try parse without first 512 bytes
        if let Ok(()) = self.try_parse(&mut mapper, &raw_bytes[512..]) { 
            self.mapper = Box::new(mapper);
            self.cartridge_metadata.has_copier_bytes = true;
            return Ok(()) 
        }
        
        // try hirom next
        let mut mapper = HiROM::new();
        if let Ok(()) = self.try_parse(&mut mapper, raw_bytes) { 
            self.mapper = Box::new(mapper);
            return Ok(()) 
        }
        // try parse without first 512 bytes
        if let Ok(()) = self.try_parse(&mut mapper, &raw_bytes[512..]) { 
            self.mapper = Box::new(mapper);
            self.cartridge_metadata.has_copier_bytes = true;
            return Ok(()) 
        }

        // try exhirom next
        let mut mapper = ExHiROM::new();
        if let Ok(()) = self.try_parse(&mut mapper, raw_bytes) { 
            self.mapper = Box::new(mapper);
            return Ok(()) 
        }
        // try parse without first 512 bytes
        if let Ok(()) = self.try_parse(&mut mapper, &raw_bytes[512..]) { 
            self.mapper = Box::new(mapper);
            self.cartridge_metadata.has_copier_bytes = true;
            return Ok(()) 
        }
        Err(CartridgeParseError::HeaderNotFound)
    }

    fn try_parse(&mut self, mapper: &mut impl Mappermode, raw_bytes: &[u8]) -> Result<(), CartridgeParseError> {
        match mapper.try_parse(raw_bytes) {
            Ok(metadata) => {
                self.cartridge_metadata = metadata;
                Ok(())
            },
            Err(e) => {
                mapper.reset();
                Err(e) 
            },
        }
    }


    /// Read a byte from memory using a 24 bit address, 
    pub fn read(&self, long_addr: u32) -> Option<u8> {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);

        match (bank, hhll) {
            // RAM
            (0x00..=0x3F, 0x0000..=0x1FFF) | // Work ram mirror in first quadrant
            (0x80..=0xBF, 0x0000..=0x1FFF) | // Work ram mirror in third quadrant
            (0x7E..=0x7F, 0x0000..=0xFFFF) => self.ram.read(long_addr),
            
            // PPU, APU registers
            (0x00..=0x3F, 0x2000..=0x3FFF) |
            (0x80..=0xBF, 0x2000..=0x3FFF) => self.ppu_memory.lock().unwrap().read(hhll),

            // Controller
            (0x00..=0x3F, 0x4000..=0x41FF) |
            (0x80..=0xBF, 0x4000..=0x41FF) => todo!(),

            // CPU, DMA
            (0x00..=0x3F, 0x4200..=0x5FFF) |
            (0x80..=0xBF, 0x4200..=0x5FFF) => todo!(),


            // Rest of space is dependant on mapper, so mapper will deal with it
            _ => self.mapper.read(long_addr),

        }
    }

    pub fn write(&mut self, long_addr: u32, value: u8) {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);

        match (bank, hhll) {
            // RAM
            (0x00..=0x3F, 0x0000..=0x1FFF) | // Work ram mirror in first quadrant
            (0x80..=0xBF, 0x0000..=0x1FFF) | // Work ram mirror in third quadrant
            (0x7E..=0x7F, 0x0000..=0xFFFF) => self.ram.write(long_addr, value),
            
            // PPU
            (0x00..=0x3F, 0x2000..=0x3FFF) |
            (0x80..=0xBF, 0x2000..=0x3FFF) => self.ppu_memory.lock().unwrap().write(hhll, value),

            // APU
            // (0x00..=0x3F, 0x2140..=0x2143) => self.apu_memory.lock().unwrap().write(hhll, value),

            // Controller
            (0x00..=0x3F, 0x4000..=0x41FF) |
            (0x80..=0xBF, 0x4000..=0x41FF) => todo!(),

            // CPU, DMA
            (0x00..=0x3F, 0x4200..=0x5FFF) |
            (0x80..=0xBF, 0x4200..=0x5FFF) => todo!(),


            // Rest of space is dependant on mapper, so mapper will deal with it
            _ => self.mapper.write(long_addr, value),

        }
    }

    /// Get all bytes from sram
    /// 
    /// Can be used to perform save games
    pub fn get_sram_bytes(&self) -> Vec<u8> {
        self.mapper.get_sram_bytes()
    }
}


/// Separates long address `$BBHHLL` as tuple `($BB, $HHLL)`
/// 
/// Any bit above the 24th in `long_addr` is ignored
fn separate_bank_hhll_addr(long_addr: u32) -> (u8, u16) {
    let bank: u8 = ((long_addr & 0xFF0000) >> 16) as u8;
    let hi_lo_byte: u16 = (long_addr & 0x00FFFF) as u16;
    (bank, hi_lo_byte)
}