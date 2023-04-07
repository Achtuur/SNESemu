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
}