const VRAM_SIZE: usize = 0x8000;

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

enum AddrRemap {
    /// No remapping
    NoRemap,
    /// Remap `rrrrrrrr YYYccccc -> rrrrrrrr cccccYYY` (2bpp)
    TwoBpp,
    /// Remap `rrrrrrrY YYcccccP -> rrrrrrrc ccccPYYY` (4bpp)
    FourBpp,
    /// Remap `rrrrrrYY YcccccPP -> rrrrrrcc cccPPYYY` (8bpp)
    EighBpp
}

pub struct Vram {
    bytes: [u16; VRAM_SIZE],

    // Vmain values ($2115)
    incr_mode: IncrementMode,
    incr_amount: usize,
    addr_remap: AddrRemap,


    vmain: u8, // $2115
    vmaddl: u8, // $2116
    vmaddh: u8, // $2117
    vmdatal: u8, // $2118
    vmdatah: u8, // $2119

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
            
        }
    }

    pub fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            0x2115 => None,
            0x2116 => None,
            0x2117 => None,
            0x2118 => None,
            0x2139 => None,
            0x213A => None,
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0x2115 => self.write_vmain(byte),
            0x2116 => ,
            0x2117 => ,
            0x2118 => ,
            0x2139 => ,

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
        self.incr_amount = match (byte & 0x1, (byte >> 1) & 0x1) {
            (0, 0) => 1,
            (0, 1) => 32,
            (1, _) => 128,
        }

    }
}