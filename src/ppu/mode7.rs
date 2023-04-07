pub struct Mode7 {
    /// RF.. ..YX, tilemap repeat (R), fill (F), flip vertical (Y), flip horizontal (X)
    m7sel: u8,
    /// Matrix A
    a: u16,
    b: u16,
    c: u16,
    d: u16,
    x: u16,
    y: u16,
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
        }
    }
}