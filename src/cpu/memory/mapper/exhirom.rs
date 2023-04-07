use super::Mappermode;

pub struct ExHiROM {
    /// Save RAM
    /// 
    /// Mapped in banks `0x70` to `0x7D` from `0x0000` to `0x7FFF`
    /// 
    /// Todo: change this to a hashmap/lis or something more memory efficient
    sram: Vec<u8>,

    /// ROM read from the cartridge
    /// 
    /// Mapped in banks `0x80 - 0xFF` from `0x8000 - 0xFFFF`, mirrored in `0x00 - 0x7D` from `0x8000 - 0xFFFF`
    rom: Vec<u8>,
}

impl ExHiROM {
    pub fn new() -> ExHiROM {
        ExHiROM {
            sram: Vec::new(),
            rom: Vec::new(),
        }
    }
}

impl Mappermode for ExHiROM {
    fn read(&self, long_addr: u32) -> Option<u8> {
        todo!()
    }

    fn write(&mut self, long_addr: u32, value: u8) {
        todo!()
    }

    fn copy_bytes_to_rom(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.rom.push(*byte);
        }
    }

    fn get_rom_size(&self) -> usize {
        self.rom.len()
    }

    fn set_ram_size(&mut self, size: u8) {
        self.sram = vec![0; (1 << size) * 1024];
    }

    fn get_sram_size(&self) -> usize {
        self.sram.len()
    }

    fn get_memory_map_mode(&self) -> u8 {
        0x05
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