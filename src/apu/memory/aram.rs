const ARAM_SIZE_BYTES: usize = 0xFFFF;

pub struct Aram {
    bytes: [u8; ARAM_SIZE_BYTES],
}

impl Aram {
    pub fn new() -> Aram {
        Aram {
            bytes: [0; ARAM_SIZE_BYTES],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.bytes[addr as usize] = val;
    }
}