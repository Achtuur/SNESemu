pub struct Oam {
    oamaddl: u8, //$2102
    oamaddh: u8, //$2103
}

impl Oam {
    pub fn new() -> Oam {
        Oam {
            oamaddh: 0,
            oamaddl: 0,
        }
    }

    pub fn read(&mut self, addr: u16) -> Option<u8> {
        todo!()
    }


    pub fn write(&mut self, addr: u16, byte: u8) {
        todo!()
    }
}