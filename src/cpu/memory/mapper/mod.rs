pub mod exhirom;
pub mod hirom;
pub mod lorom;

use crate::cpu::{CartridgeParseError, CpuResult};

use super::cartridge::{CartridgeMapperType, CartridgeMetadata};

use crate::cpu::CpuError;


pub trait Mappermode {
    /// Handles reading memory in the following regions:
    /// * Q1 upper half
    /// * Q2
    /// * Q3 upper half
    /// * Q4
    fn read(&self, long_addr: u32) -> Option<u8>;

    /// Handles writing to memory in the following regions:
    /// * Q1 upper half
    /// * Q2
    /// * Q3 upper half
    /// * Q4
    fn write(&mut self, long_addr: u32, value: u8);

    // /// Copy raw bytes to internal rom vector
    // fn copy_bytes_to_rom(&mut self, bytes: &[u8]);

    fn rom_size(&self) -> usize;

    /// Returns size of sram in bytes
    fn sram_size(&self) -> usize;

    /// Returns all bytes currently in SRAM, ordered from lowest to highest address
    fn sram_bytes(&self) -> &[u8];

    /// Copy raw bytes to internal sram vector, overwrites any previously existing value!
    ///
    /// Only for use when loading a save
    fn copy_bytes_to_sram(&mut self, bytes: &[u8]);

    /// Reset mapper to initial state
    ///
    /// clears ram and rom completely
    fn reset(&mut self);
}


#[derive(Debug)]
pub enum CpuMemoryMapper<'m> {
    LoROM(lorom::LoROM<'m>),
    HiROM(hirom::HiROM<'m>),
    ExHiROM(exhirom::ExHiROM<'m>),
}


impl std::default::Default for CpuMemoryMapper<'_> {
    fn default() -> Self {
        Self::LoROM(lorom::LoROM::new(&[], 0))
    }
}

impl<'m> CpuMemoryMapper<'m> {
    pub fn from_cartridge_data(cart_data: &CartridgeMetadata<'m>) -> CpuResult<Self> {
        let rom = cart_data.rom_bytes;
        let sram_size = cart_data.ram_size;
        let mapper = match cart_data.mapper {
            Some(CartridgeMapperType::LoROM) => Self::LoROM(lorom::LoROM::new(rom, sram_size)),
            Some(CartridgeMapperType::HiROM) => Self::HiROM(hirom::HiROM::new(rom, sram_size)),
            Some(CartridgeMapperType::ExHiROM) => {
                Self::ExHiROM(exhirom::ExHiROM::new(rom, sram_size))
            }
            _ => return Err(CartridgeParseError::WrongMapperMode(0).into()),
        };
        Ok(mapper)
    }
}

/// Implements the Peripheral trait for `$enum`.
///
/// This is equivalent to writing out a match statement for each variant of `$enum`.
/// It basically saves a lot of writing when adding drivers
macro_rules! impl_trait {
    ($enum: ident, [$($variants: ident),+]) => {
        // use $crate::cpu::memory::Mappermode;
        #[allow(unused)] // some methods could be useful later on
        impl $enum<'_> {
            pub(crate) fn read(&self, long_addr: u32) -> Option<u8> {
                match self {
                    $(Self::$variants(x) => Mappermode::read(x, long_addr)),+
                }
            }

            pub(crate) fn write(&mut self, long_addr: u32, value: u8) {
                match self {
                    $(Self::$variants(x) => Mappermode::write(x, long_addr, value)),+
                }
            }

            pub(crate) fn rom_size(&self) -> usize {
                match self {
                    $(Self::$variants(x) => Mappermode::rom_size(x)),+
                }
            }

            pub(crate) fn sram_size(&self) -> usize {
                match self {
                    $(Self::$variants(x) => Mappermode::sram_size(x)),+
                }
            }

            pub(crate) fn sram_bytes(&self) -> &[u8] {
                match self {
                    $(Self::$variants(x) => Mappermode::sram_bytes(x)),+
                }
            }

            pub(crate) fn copy_bytes_to_sram(&mut self, bytes: &[u8]) {
                match self {
                    $(Self::$variants(x) => Mappermode::copy_bytes_to_sram(x, bytes)),+
                }
            }

            pub(crate) fn reset(&mut self) {
                match self {
                    $(Self::$variants(x) => Mappermode::reset(x)),+
                }
            }
        }

        impl std::fmt::Display for CpuMemoryMapper<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variants(_) => write!(f,
                            "{}: rom[{:#06X}], sram[{:#04X}]",
                            stringify!($variants), self.rom_size(), self.sram_size()
                        )
                    ),+
                }
            }
        }
    }
}

impl_trait!(CpuMemoryMapper, [
    LoROM,
    HiROM,
    ExHiROM
]);
