pub struct Mode7 {
    /// RF.. ..YX, tilemap repeat (R), fill (F), flip vertical (Y), flip horizontal (X)
    pub m7sel: u8,
    /// Matrix A
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
    pub x: u16,
    pub y: u16,
    pub hscroll: u16,
    pub vscroll: u16,

    latch: u8,
}

impl Mode7 {
    pub fn new() -> Mode7 {
        Mode7 {
            m7sel: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            x: 0,
            y: 0,
            latch: 0,
            hscroll: 0,
            vscroll: 0,
            
        }
    }

    pub fn read(&mut self, addr: u16) -> Option<u8> {
        todo!()
    }

    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            0x211A => self.m7sel = byte,
            0x211B => self.a = self.write_latch(byte),
            0x211C => self.b = self.write_latch(byte),
            0x211D => self.c = self.write_latch(byte),
            0x211E => self.d = self.write_latch(byte),
            0x211F => self.x = self.write_latch(byte),
            0x2120 => self.y = self.write_latch(byte),
        }
    }

    /// Returns `byte << 8 | latch` and then sets `latch` to `byte`
    fn write_latch(&mut self, byte: u8) -> u16 {
        let val = ((byte as u16) << 8) | self.latch as u16;
        self.latch = byte;
        val
    }

    pub fn write_hscroll(&mut self, byte: u8) {
        self.hscroll = self.write_latch(byte);
    }

    pub fn write_vscroll(&mut self, byte: u8) {
        self.vscroll = self.write_latch(byte);
    }

}