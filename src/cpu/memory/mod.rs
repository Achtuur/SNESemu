use std::sync::{Mutex, Arc};

use crate::{ppu::memory::PpuMemory, apu::memory::ApuMemory, separate_bank_hhll_addr};

use self::{mapper::{Mappermode, lorom::LoROM, hirom::HiROM, exhirom::ExHiROM}, cartridge::{CartridgeParseError, CartridgeMetadata}, ram::Ram};


pub mod mapper;
pub mod ram;
pub mod cartridge;

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

    pub fn set_ppumemory_ref(&mut self, memref: Arc<Mutex<PpuMemory>>) {
        self.ppu_memory = memref;
    }

    pub fn set_apumemory_ref(&mut self, memref: Arc<Mutex<ApuMemory>>) {
        self.apu_memory = memref;
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
    pub fn read(&mut self, long_addr: u32) -> Option<u8> {
        let (bank, hhll) = separate_bank_hhll_addr!(long_addr);
        match (bank, hhll) {
            // RAM
            (0x00..=0x3F, 0x0000..=0x1FFF) | // Work ram mirror in first quadrant
            (0x80..=0xBF, 0x0000..=0x1FFF) | // Work ram mirror in third quadrant
            (0x7E..=0x7F, 0x0000..=0xFFFF) | // Main region of WRAM
            (0x00..=0x3F, 0x2180..=0x2183) | // WM registers
            (0x80..=0xBF, 0x2180..=0x2183) => self.ram.read(long_addr),
            
            // PPU, APU registers
            (0x00..=0x3F, 0x2000..=0x213F) |
            (0x80..=0xBF, 0x2000..=0x213F) => self.ppu_memory.lock().unwrap().read(hhll),


            // Controller todo: implement this!
            (0x00..=0x3F, 0x4000..=0x41FF) |
            (0x80..=0xBF, 0x4000..=0x41FF) => Some(0),

            // CPU, DMA todo: implement this!
            (0x00..=0x3F, 0x4200..=0x5FFF) |
            (0x80..=0xBF, 0x4200..=0x5FFF) => Some(0),


            // Rest of space is dependant on mapper, so mapper will deal with it
            _ => self.mapper.read(long_addr),
        }
    }

    pub fn write(&mut self, long_addr: u32, byte: u8) {
        let (bank, hhll) = separate_bank_hhll_addr!(long_addr);

        match (bank, hhll) {
            // RAM
            (0x00..=0x3F, 0x0000..=0x1FFF) | // Work ram mirror in first quadrant
            (0x80..=0xBF, 0x0000..=0x1FFF) | // Work ram mirror in third quadrant
            (0x7E..=0x7F, 0x0000..=0xFFFF) | // Main region of WRAM
            (0x00..=0x3F, 0x2180..=0x2183) | // WM registers
            (0x80..=0xBF, 0x2180..=0x2183) => self.ram.write(long_addr, byte),
            
            // PPU
            (0x00..=0x3F, 0x2000..=0x213F) |
            (0x80..=0xBF, 0x2000..=0x213F) => self.ppu_memory.lock().unwrap().write(hhll, byte),

            // APU
            // (0x00..=0x3F, 0x2140..=0x2143) => self.apu_memory.lock().unwrap().write(hhll, value),

            // Controller todo: implement this!
            (0x00..=0x3F, 0x4000..=0x41FF) |
            (0x80..=0xBF, 0x4000..=0x41FF) => {},

            // CPU, DMA todo: implement this!
            (0x00..=0x3F, 0x4200..=0x5FFF) |
            (0x80..=0xBF, 0x4200..=0x5FFF) => {},


            // Rest of space is dependant on mapper, so mapper will deal with it
            _ => self.mapper.write(long_addr, byte),

        }
    }

    /// Get all bytes from sram
    /// 
    /// Can be used to perform save games
    pub fn get_sram_bytes(&self) -> Vec<u8> {
        self.mapper.get_sram_bytes()
    }
}
