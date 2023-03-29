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
        const Accumulator8bit = 0b00_0010_0000;
        /// X register size (native mode only) (0 = 16 bits, 1 = 8 bits)
        const Xreg8bit = 0b00_0001_0000;
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


impl ProcessorStatusFlags {
    pub fn new() -> Self {
        ProcessorStatusFlags::from_bits(0).unwrap()
    }

    pub fn clear_all(&mut self) {
        *self.0.bits_mut() = 0;
    }

    pub fn set_flag(&mut self, flags: Self) {
        self.set(flags, true);
    } 

    pub fn clear_flag(&mut self, flags: Self) {
        self.set(flags, false);
    }
}
