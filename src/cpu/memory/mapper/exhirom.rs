use super::Mappermode;

#[derive(Debug)]
pub struct ExHiROM<'m> {
    /// Save RAM
    ///
    /// Mapped in banks `0x70` to `0x7D` from `0x0000` to `0x7FFF`
    ///
    /// Todo: change this to a hashmap/lis or something more memory efficient
    sram: Vec<u8>,

    /// ROM read from the cartridge
    ///
    /// Mapped in banks `0x80 - 0xFF` from `0x8000 - 0xFFFF`, mirrored in `0x00 - 0x7D` from `0x8000 - 0xFFFF`
    rom: &'m [u8],
}

impl<'m> ExHiROM<'m> {
    pub fn new(rom: &'m [u8], sram_size: usize) -> Self {
        let sram = vec![0; sram_size * 1024];
        Self {
            rom,
            sram,
        }
    }
}

impl Mappermode for ExHiROM<'_> {
    fn read(&self, long_addr: u32) -> Option<u8> {
        todo!()
    }

    fn write(&mut self, long_addr: u32, value: u8) {
        todo!()
    }

    fn rom_size(&self) -> usize {
        self.rom.len()
    }

    fn sram_size(&self) -> usize {
        self.sram.len()
    }

    fn reset(&mut self) {
        // don't want to reallocate
        for x in self.sram.iter_mut() {
            *x = 0;
        }
    }

    fn sram_bytes(&self) -> &[u8] {
        &self.sram
    }

    fn copy_bytes_to_sram(&mut self, bytes: &[u8]) {
        self.sram = Vec::new();
        for byte in bytes {
            self.sram.push(*byte);
        }
    }
}
