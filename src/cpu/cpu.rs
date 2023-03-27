use super::{Cpu, ProcessorStatusFlags};

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            sp: 0,
            pc: 0, //todo -> set to correct init value
            acc: 0,
            status: ProcessorStatusFlags::from_bits(0).unwrap(),
            ram: [0_u8; 0x1FFF],
            x: 0,
            y: 0,
            dp: 0,
            dbr: 0,
            pbr: 0,
        }
    }

    pub fn init(&mut self) {

    }

    // This function is called every 'clock cycle'
    pub fn tick(&mut self) {
        // read instruction

        // read data needed by instruction

        // execute instruction
    }
}