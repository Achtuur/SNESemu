use std::sync::{Arc, Mutex};

use mapper::CpuMemoryMapper;

use crate::{apu::memory::ApuMemory, ppu::memory::PpuMemory, separate_bank_hhll_addr};

use self::{
    cartridge::{CartridgeMetadata},
    mapper::{exhirom::ExHiROM, hirom::HiROM, lorom::LoROM, Mappermode},
    ram::Ram,
};

use super::{CartridgeParseError, CpuError, CpuResult};

pub(crate) mod cartridge;
pub(crate) mod mapper;
pub(crate) mod ram;

/// Struct that represents all memory in the SNES, this is shared between CPU, PPU and APU as they all need to read/write to it
///
/// Addresses in the SNES are represented by `$BBHHLL`, where `BB` is the bank byte, `HH` is the high byte and `LL` is the low byte.
///
/// Quadrants are also specified in the same way [this video](https://www.youtube.com/watch?v=-U76YvWdnZM) explaning mapping does.
///
/// Since rust does not support `u24` natively, `u32` will be used instead and the upper 8 bits are unused and ignored
///
/// Source used: [snes wiki page](https://snes.nesdev.org/wiki/Memory_map)
pub struct CpuMemory<'m> {
    mapper: CpuMemoryMapper<'m>,
    ram: Ram,
    ppu_memory: Arc<Mutex<PpuMemory>>,
    apu_memory: Arc<Mutex<ApuMemory>>,
}

impl<'m> CpuMemory<'m> {
    pub fn new() -> Self {
        CpuMemory {
            mapper: CpuMemoryMapper::default(),
            ram: Ram::new(),
            ppu_memory: Arc::new(Mutex::new(PpuMemory::new())),
            apu_memory: Arc::new(Mutex::new(ApuMemory::new())),
        }
    }

    pub fn reset(&mut self) {
        self.mapper.reset();
        self.ram.reset();
        // self.ppu_memory.lock().unwrap().reset();
        // self.apu_memory.lock().unwrap().reset();
    }


    pub fn from_cartridge(cartridge_bytes: &'m [u8]) -> CpuResult<Self> {
        let mapper = Self::parse_cartridge(cartridge_bytes)?;
        tracing::info!("Using mapper: {}", mapper);
        Ok(Self {
            mapper,
            ram: Ram::new(),
            ppu_memory: Arc::new(Mutex::new(PpuMemory::new())),
            apu_memory: Arc::new(Mutex::new(ApuMemory::new())),
        })
    }

    fn parse_cartridge(cartridge_bytes: &'m [u8]) -> CpuResult<CpuMemoryMapper<'m>> {
        tracing::debug!("Reading cartridge without copier bytes...");
        match CartridgeMetadata::try_parse(cartridge_bytes) {
            Ok(meta) => {
                tracing::info!("Loaded cartridge: {} ({})", meta.title, meta.rom_size);
                return CpuMemoryMapper::from_cartridge_data(&meta)
            },
            Err(e) => {
                tracing::debug!("Failed to parse cartridge: {}", e);
            }
        }

        // copier bytes
        tracing::debug!("Reading cartridge with copier bytes...");
        match CartridgeMetadata::try_parse(&cartridge_bytes[512..]) {
            Ok(meta) => {
                tracing::info!("Loaded cartridge: {} ({})", meta.title, meta.rom_size);
                return CpuMemoryMapper::from_cartridge_data(&meta)
            },
            Err(e) => {
                tracing::debug!("Failed to parse cartridge: {}", e);
            }
        }

        Err(CartridgeParseError::HeaderNotFound.into())
    }

    pub fn set_ppumemory_ref(&mut self, memref: Arc<Mutex<PpuMemory>>) {
        self.ppu_memory = memref;
    }

    pub fn set_apumemory_ref(&mut self, memref: Arc<Mutex<ApuMemory>>) {
        self.apu_memory = memref;
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


            // Controller
            (0x00..=0x3F, 0x4000..=0x41FF) |
            (0x80..=0xBF, 0x4000..=0x41FF) => {
                tracing::trace!("Reading unimplemented controller registers, returning None");
                None
            },

            // CPU, DMA
            (0x00..=0x3F, 0x4200..=0x5FFF) |
            (0x80..=0xBF, 0x4200..=0x5FFF) => {
                tracing::trace!("Reading unimplemented DMA registers, returning None");
                None
            },


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

            // Controller
            (0x00..=0x3F, 0x4000..=0x41FF) |
            (0x80..=0xBF, 0x4000..=0x41FF) => {
                tracing::trace!("Writing unimplemented controller registers, ignoring");
            },

            // CPU, DMA
            (0x00..=0x3F, 0x4200..=0x5FFF) |
            (0x80..=0xBF, 0x4200..=0x5FFF) => {
                tracing::trace!("Writing unimplemented DMA registers, ignoring");
            },


            // Rest of space is dependant on mapper, so mapper will deal with it
            _ => self.mapper.write(long_addr, byte),

        }
    }
}
