use self::processorstatusflag::ProcessorStatusFlags;

mod cpu;
pub mod processorstatusflag;

pub struct Cpu {
    // Registers
    /// Stack pointer
    /// (Points to the next available(unused) location on the stack.)
    sp: u16,
    /// Program counter
    /// (Holds the address of the current instruction to execute.)
    pc: u16,
    /// Accumulator
    /// (This is the math register. It stores one of two operands or the result of most arithmetic and logical operations.)
    acc: u16,
    /// Processor status flags
    status: ProcessorStatusFlags,
    /// Index register X
    /// (Can be used to reference memory, to pass data to memory, or as counters for loops.)
    x: u16,
    /// Index register Y
    /// (Can be used to reference memory, to pass data to memory, or as counters for loops.)        
    y: u16,
    /// Direct page register
    /// (Used for direct page addressing modes.)
    dp: u16,
    /// Data bank register
    /// (Holds the default bank for memory transfers.)
    dbr: u8,
    /// Program bank register
    /// (Holds the bank address of all instruction fetches.)
    pbr: u8,

    /// 128 KB RAM, addresses $7E0000-$7FFFFF
    ram: [u8; 0x1FFF],
}

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
