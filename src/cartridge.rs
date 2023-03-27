/// Cartridge struct handles reading cartridges and read/write to cartridge ROM/RAM
pub struct Cartridge {
    raw_bytes: Vec<u8>,
}

impl Cartridge {
    pub fn new(raw_bytes: &[u8]) -> Cartridge {
        Cartridge {
            raw_bytes: raw_bytes.to_vec(),
        }
    }

    /// Parse cartridge and obtain information from ROM headers for use in memory structs
    pub fn parse() {
        
    }
}