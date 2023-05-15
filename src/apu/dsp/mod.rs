pub struct DSP {
    write_addr: u8,
    
    /// Voices
    voice1: u8,
    /// More registers and stuff
    reg0: u8,
}

impl DSP {
    pub fn new() -> DSP {
        DSP {
            write_addr: 0,
            voice1: 0,
            reg0: 0,
        }
    }

    /// Set write address for DSP, set by writing to `$00F2` in SMP700
    pub fn set_write_addr(&mut self, byte: u8) {
        self.write_addr = byte;
    }

    /// Write a byte to a DSP register, done by writing to `$00F3` in SMP700
    pub fn write_byte(&mut self, byte: u8) {
        match self.write_addr {
            // Voices
            0x00 | 0x01 => {},

            0x02 => {},
            0x04 => {},
            0x05 => {},
            0x07 => {},
            0x08 => {},
            0x09 => {},

            // Master
            0x0C => {},
            0x6C => {},
            0x2D => {},
            0x3D => {},
            0x5D => {},

            // Echo
            0x2C => {},
            0x4D => {},
            0x0D => {},
            0x6D => {},
            0x7D => {},

            // Control
            0x4C => {},
            0x5C => {},
            0x7C => {},

            // FIR
            0x0F => {},
            0x1F => {},
            0x2F => {},
            0x3F => {},
            0x4F => {},
            0x5F => {},
            0x6F => {},
            0x7F => {},

            _ => unreachable!(),
        }
    }
}