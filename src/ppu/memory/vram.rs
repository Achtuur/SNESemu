const VRAM_SIZE: usize = 0x8000;
pub struct Vram {
    bytes: [u16; VRAM_SIZE],
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
            0x2139 => None,
            0x213A => None,
        }
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            
        }
    }
}