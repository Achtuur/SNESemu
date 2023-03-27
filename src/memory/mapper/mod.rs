

mod lorom;
mod hirom;

pub struct Mapper {

}

impl Mapper {
    pub fn new(rom: [u8; 0xFF]) -> Self {
        Mapper {

        }
    }
}

pub trait Mappermode {
    /// Handles reading memory in the following regions:
    /// * Q1 upper half
    /// * Q2
    /// * Q3 upper half
    /// * Q4
    fn read(&self, long_addr: u32) -> u8;

    /// Handles writing to memory in the following regions:
    /// * Q1 upper half
    /// * Q2
    /// * Q3 upper half
    /// * Q4
    fn write(&mut self, long_addr: u32, value: u8);
}