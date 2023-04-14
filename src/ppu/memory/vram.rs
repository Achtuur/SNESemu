use crate::{to_word, nth_bit, bit_slice, high_byte, low_byte};

const VRAM_SIZE: usize = 0x8000;

#[derive(Debug, Clone, Copy)]
/// Increment mode for VRAM based on byte written to `$2115`
enum IncrementMode {
    /// Increment when reading/writing to low VRAM byte
    /// 
    /// `$2118` for write and `$2139` for read respectively
    LowByte, 

    /// Increment when reading/writing to high VRAM byte
    /// 
    /// `$2119` for write and `$213A` for read respectively
    HighByte
}

#[derive(Debug, Clone, Copy)]
/// Address remapping mode based on bits 3 and 2 of `VMAIN` register
enum AddrRemap {
    /// No remapping
    NoRemap,
    /// Remap `rrrrrrrr YYYccccc -> rrrrrrrr cccccYYY` (2bpp)
    TwoBpp,
    /// Remap `rrrrrrrY YYcccccP -> rrrrrrrc ccccPYYY` (4bpp)
    FourBpp,
    /// Remap `rrrrrrYY YcccccPP -> rrrrrrcc cccPPYYY` (8bpp)
    EightBpp
}

pub struct Vram {
    /// Raw bytes representing all of VRAM
    bytes: [u16; VRAM_SIZE],
    /// Latch with read vram address, updates when VMADDL/H is updated
    latch: u16,
    /// Pointer to VRAM that represents vmaddl and vmaddh
    pointer: usize,
    // Vmain values ($2115)
    incr_mode: IncrementMode,
    incr_amount: u8,
    addr_remap: AddrRemap,


    vmain: u8, // $2115
    vmaddl: u8, // $2116
    vmaddh: u8, // $2117
    vmdatal: u8, // $2118
    vmdatah: u8, // $2119

    /// True if there is currently a V-blank or F-blank (NOT H-BLANK!)
    is_fvblanking: bool,
}

impl Vram {
    pub fn new() -> Vram {
        Vram {
            bytes: [0; VRAM_SIZE],
            vmain: 0,
            vmaddl: 0,
            vmaddh: 0,
            vmdatal: 0,
            vmdatah: 0,
            latch: 0,
            pointer: 0,
            incr_mode: IncrementMode::LowByte,
            incr_amount: 1,
            addr_remap: AddrRemap::NoRemap,
            is_fvblanking: false,
        }
    }

    pub fn start_fv_blank(&mut self) {
        self.is_fvblanking = true;
    }

    pub fn stop_fvh_blank(&mut self) {
        self.is_fvblanking = false;
    }

    /// Read from VRAM registers `$2139` and `$213A`
    pub fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            // VMDATALREAD
            0x2139 => {
                let val = low_byte!(self.latch);
                if matches!(self.incr_mode, IncrementMode::LowByte) {
                    self.latch = self.bytes[self.pointer];
                    self.increment_addr();
                    self.update_pointer();
                }
                Some(val)
            },
            // VMDATAHREAD
            0x213A => {
                let val = high_byte!(self.latch);
                if matches!(self.incr_mode, IncrementMode::HighByte) {
                    self.latch = self.bytes[self.pointer];
                    self.increment_addr();
                    self.update_pointer();
                }
                Some(val)
            },

            _ => None,
        }
    }

    /// Write to VRAM registers `$2115` to and including `$2119`
    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            // VMAIN
            0x2115 => self.write_vmain(byte),
            // VMADDL
            0x2116 => {
                self.vmaddl = byte;
                self.update_pointer();
                self.latch = self.bytes[self.pointer];
            },
            // VMADDH
            0x2117 => {
                self.vmaddh = byte & 0x7F; //ignore high bit
                self.update_pointer();
                self.latch = self.bytes[self.pointer];
            },
            // VMDATAL
            0x2118 => {
                self.vmdatal = byte;
                self.write_data();
                if matches!(self.incr_mode, IncrementMode::LowByte) {
                    self.increment_addr();
                    self.update_pointer();
                }
            },
            // VMDATAH
            0x2119 => {
                self.vmdatah = byte;
                self.write_data();
                if matches!(self.incr_mode, IncrementMode::HighByte) {
                    self.increment_addr();
                    self.update_pointer();
                }
            },

            _ => {},
        }
    }

    /// 7  bit  0
    /// ---- ----
    /// M... RRII
    /// |    ||||
    /// |    ||++- Address increment amount:
    /// |    ||     0: Increment by 1 word
    /// |    ||     1: Increment by 32 words
    /// |    ||     2: Increment by 128 words
    /// |    ||     3: Increment by 128 words
    /// |    ++--- Address remapping: (VMADD -> Internal)
    /// |           0: None
    /// |           1: Remap rrrrrrrr YYYccccc -> rrrrrrrr cccccYYY (2bpp)
    /// |           2: Remap rrrrrrrY YYcccccP -> rrrrrrrc ccccPYYY (4bpp)
    /// |           3: Remap rrrrrrYY YcccccPP -> rrrrrrcc cccPPYYY (8bpp)
    /// +--------- Address increment mode:
    /// 0: Increment after writing $2118 or reading $2139
    /// 1: Increment after writing $2119 or reading $213A
    fn write_vmain(&mut self, byte: u8) {
        if !self.is_fvblanking {
            return;
        }

        self.incr_amount = match byte & 0b11 {
            0b00 => 1,
            0b01 => 32,
            0b10 | 0b11 => 128,
            _ => unreachable!(),
        };

        self.addr_remap = match (byte >> 2) * 0b11 {
            0b00 => AddrRemap::NoRemap,
            0b01 => AddrRemap::TwoBpp,
            0b10 => AddrRemap::FourBpp,
            0b11 => AddrRemap::EightBpp,
            _ => unreachable!(),
        };
        
        self.incr_mode = match byte >> 7 {
            0b0 => IncrementMode::LowByte,
            0b1 => IncrementMode::HighByte,
            _ => unreachable!(),
        };
    }
    
    /// Helper function to write `VMDATAH << 8 | VMDATAL` to `self.bytes[pointer]`
    fn write_data(&mut self) {
        if !self.is_fvblanking {
            self.bytes[self.pointer] = to_word!(self.vmdatah, self.vmdatal);
        }
    }

    /// Update pointer to VRAM after `vmaddh` or `vmaddl` are updated
    fn update_pointer(&mut self) {
        if !self.is_fvblanking {
            let pointer = get_remapped_address(&self.addr_remap, self.vmaddl, self.vmaddh);
            self.pointer = pointer as usize;
        }
    }

    /// Increment VMADD based on increment mode and increment amount
    fn increment_addr(&mut self) {
        match self.incr_mode {
            IncrementMode::LowByte => self.vmaddl = self.vmaddl.wrapping_add(self.incr_amount),
            IncrementMode::HighByte => self.vmaddh = self.vmaddh.wrapping_add(self.incr_amount),
        }
    }
}

/// Returns remapped address based on `remap`
/// 
/// * `AddrRemap::NoRemap` -- None
/// * `AddrRemap::TwoBpp` -- Remap rrrrrrrr YYYccccc -> rrrrrrrr cccccYYY
/// * `AddrRemap::FourBpp` -- Remap rrrrrrrY YYcccccP -> rrrrrrrc ccccPYYY
/// * `AddrRemap::EightBpp` -- Remap rrrrrrYY YcccccPP -> rrrrrrcc cccPPYYY
fn get_remapped_address(remap: &AddrRemap, addrl: u8, addrh: u8) -> u16 {
    match remap {
        AddrRemap::NoRemap => to_word!(addrh, addrl),
        AddrRemap::TwoBpp => {
            let y = bit_slice!(addrl, 5, 7);
            let ll = addrl << 3 | y;
            to_word!(addrh, ll)
        },
        AddrRemap::FourBpp => {
            let p = nth_bit!(addrl, 0);
            let y = nth_bit!(addrh, 0) << 2 | bit_slice!(addrl, 6, 7);
            let c = nth_bit!(addrl, 5);
            let ll = ((addrl << 2) & 0xF0) | p << 3 | y;
            let hh = (addrh & 0xFE) | c;
            to_word!(hh, ll)
        },
        AddrRemap::EightBpp => {
            let p = bit_slice!(addrl, 0, 1);
            let y = bit_slice!(addrh, 0, 1) << 1 | nth_bit!(addrl, 7);
            let c = bit_slice!(addrl, 5, 6);
            let ll = ((addrl << 3) & 0xD0) | p << 3 | y;
            let hh = (addrh & 0xFC) | c;
            to_word!(hh, ll)
        },
    }
}