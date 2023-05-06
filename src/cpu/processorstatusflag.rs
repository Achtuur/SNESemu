use bitflags::{bitflags, BitFlags};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    /// Status flags for SNES CPU
    ///
    /// Status flags can also be unions, 
    /// so checking for example the negative and overflow flags at the 
    /// same time can be done with `cpu.status.contains(ProcessorStatusFlags::Negative | ProcessorStatusFlags::Overflow)`
    pub struct ProcessorStatusFlags: u16 {
        /// Flag to check if WAI was called (not actually in snes!)
        const WaitForInterrupt = 0b100_0000_0000;
        /// Break (only in Emulation mode)
        const Break = 0b10_0000_0000;
        /// Emulation mode
        const Emulation = 0b01_0000_0000;
        /// Negative
        const Negative = 0b00_1000_0000;
        /// Overflow
        const Overflow = 0b00_0100_0000;
        /// Accumulator register size (native mode only) (0 = 16 bits, 1 = 8 bits)
        const Accumulator8bit = 0b00_0010_0000;
        /// X&Y register size (native mode only) (0 = 16 bits, 1 = 8 bits)
        const XYreg8bit = 0b00_0001_0000;
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

    pub fn startup_state() -> Self {
        let mut p = Self::new();
        p.set_flag(
            ProcessorStatusFlags::Emulation | 
            ProcessorStatusFlags::Accumulator8bit | 
            ProcessorStatusFlags::XYreg8bit
        );
        p
    }

    pub fn accflag_as_u8(&self) -> u8 {
        if self.contains(ProcessorStatusFlags::Accumulator8bit) {
           return 1;
        }
        0
    }

    pub fn xyflag_as_u8(&self) -> u8 {
        if self.contains(ProcessorStatusFlags::XYreg8bit) {
           return 1;
        }
        0
    }

    pub fn set_bits(&mut self, bits: u8) {
        let flag = ProcessorStatusFlags::from_bits(bits as u16).unwrap();
        self.set_flag(flag);
    }

    pub fn clear_bits(&mut self, bits: u8) {
        let flag = ProcessorStatusFlags::from_bits(bits as u16).unwrap();
        println!("bits: {0:02X?}", bits);
        println!("flag: {0:?}", flag);
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
