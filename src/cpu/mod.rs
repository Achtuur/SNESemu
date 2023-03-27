pub mod instructions;
pub mod processorstatusflag;
mod cpu;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    /// Status flags for SNES CPU
    ///
    /// Use `cpu.status.set(ProccerStatusFlags::<variant>, true|false)` to set/clear a flag and `cpu.status.contains(ProcessorStatusFlags::<variant>)` to check if flag is set
    ///
    /// Status flags can also be unions, so checking for example the negative and overflow flags at the same time can be done with `cpu.status.contains(ProcessorStatusFlags::Negative | ProcessorStatusFlags::Overflow)`
    pub struct ProcessorStatusFlags: u16 {
        /// Emulation mode
        const Emulation = 0b10_0000_0000;
        /// Break (only in Emulation mode)
        const Break = 0b01_0000_0000;
        /// Negative
        const Negative = 0b00_1000_0000;
        /// Overflow
        const Overflow = 0b00_0100_0000;
        /// Accumulator register size (native mode only) (0 = 16 bits, 1 = 8 bits)
        const Accumulatorsize = 0b00_0010_0000;
        /// X register size (native mode only) (0 = 16 bits, 1 = 8 bits)
        const Xregistersize = 0b00_0001_0000;
        /// Decimal
        const Decimal = 0b00_0000_1000;
        /// IRQ disable
        const IRQdisable = 0b00_0000_0100;
        /// Zero
        const Zero = 0b00_0000_0010;
        /// Carry
        const Carry = 0b00_0000_0001;
    }
}

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
