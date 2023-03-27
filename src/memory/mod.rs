use self::mapper::Mappermode;


mod mapper;
mod ram;

/// Struct that represents all memory in the SNES, this is shared between CPU, PPU and APU as they all need to read/write to it
/// 
/// Addresses in the SNES are represented by `$BBHHLL`, where `BB` is the bank byte, `HH` is the high byte and `LL` is the low byte.
/// 
/// Quadrants are also specified in the same way [this video](https://www.youtube.com/watch?v=-U76YvWdnZM) explaning mapping does.
/// 
/// Since rust does not support `u24` natively, `u32` will be used instead and the upper 8 bits are unused and ignored
/// 
/// Source used: [snes wiki page](https://snes.nesdev.org/wiki/Memory_map)
/// 
pub struct Memory {
    mapper: Box<dyn Mappermode>,

}

impl Memory {
    pub fn new() -> Self {
        Memory { 
            mapper: todo!(),
        }
    }


    pub fn read(&self, long_addr: u32) -> u8 {
        let (bank, hhll) = separate_bank_hhll_addr(long_addr);

        match (bank, hhll) {
            // RAM
            (0x00..=0x3F, 0x0000..=0x1FFF) | // Work ram mirror in first quadrant
            (0x80..=0xBF, 0x0000..=0x1FFF) | // Work ram mirror in third quadrant
            (0x7E..=0x7F, 0x0000..=0xFFFF) => todo!(),
            
            // PPU, APU registers
            (0x00..=0x3F, 0x2000..=0x3FFF) |
            (0x80..=0xBF, 0x2000..=0x3FFF) => todo!(),

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

    pub fn write(&mut self, long_addr: u32) {

    }
}


/// Separates long address `$BBHHLL` as tuple `($BB, $HHLL)`
fn separate_bank_hhll_addr(long_addr: u32) -> (u8, u16) {
    let bank: u8 = ((long_addr & 0xFF0000) >> 16) as u8;
    let hi_lo_byte: u16 = (long_addr & 0x00FFFF) as u16;
    (bank, hi_lo_byte)
}