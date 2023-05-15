use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    /// Status flags for SMP700
    ///
    /// Status flags can also be unions, 
    /// so checking for example the negative and overflow flags at the 
    /// same time can be done with `cpu.status.contains(StatusFlags::Negative | StatusFlags::Overflow)`
    pub struct StatusWord: u16 {
        /// N - Negative
        const Negative = 0b00_1000_0000;
        /// V - Overflow
        const Overflow = 0b00_0100_0000;
        /// P - Direct page flag (moves the direct page to $0100 if set)
        const DPFlag = 0b00_0010_0000;
        /// B - Break (?) (unused, documentation doesnt say anything?)
        const Break = 0b00_0001_0000;
        ///  H - Half-carry flag (carry between low and high nibble)
        const HalfCarry = 0b00_0000_1000;
        /// Interrupt Enable (unused)
        const InterruptEnable = 0b00_0000_0100;
        /// Zero
        const Zero = 0b00_0000_0010;
        /// Carry
        const Carry = 0b00_0000_0001;
    }
}


impl StatusWord {
    pub fn new() -> Self {
        StatusWord::from_bits(0).unwrap()
    }

    pub fn startup_state() -> Self {
        let mut p = Self::new();
        p
    }

    pub fn set_bits(&mut self, bits: u8) {
        let flag = StatusWord::from_bits(bits as u16).unwrap();
        self.set_flag(flag);
    }

    pub fn clear_bits(&mut self, bits: u8) {
        let flag = StatusWord::from_bits(bits as u16).unwrap();
        self.clear_flag(flag);
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

    /// Return lower 8 bits of status flag (everything except emulation flag and break flag)
    pub fn get_bits(&self) -> u8 {
        self.bits() as u8
    }
}
