#[derive(Debug)]
pub enum CartridgeRegion {
    Japan,
    NorthAmerica,
    Europe,
    Sweden,
    Finland,
    Denmark,
    France,
    Netherlands,
    Spain,
    Germany,
    Italy,
    China,
    Indonesia,
    SouthKorea,
    CommonInternational,
    Canada,
    Brazil,
    Australia,
    Unidentified,
}

#[derive(Debug)]
pub enum CartridgeParseError {
    HeaderNotFound,
    WrongMapperMode,
    ChecksumMismatch,
    RomSizeMismatch,
    InvalidGameName,
}

#[derive(Debug)]
pub struct CartridgeMetadata {
    title: String,
    dev_id: String,
    region: CartridgeRegion,
    version: u8,
    maker_code: String,
    game_code: String,
    rom_size: usize,
    ram_size: usize,
    pub has_copier_bytes: bool,
}

impl CartridgeMetadata {
    pub fn new() -> Self {
        CartridgeMetadata {
            title: String::from(""),
            dev_id: String::from(""),
            region: CartridgeRegion::Unidentified,
            version: 0,
            maker_code: String::from(""),
            game_code: String::from(""),
            rom_size: 0,
            ram_size: 0,
            has_copier_bytes: false,
        }
    }
    
    pub fn set_dev_id(&mut self, dev_id_bytes: &[u8]) {
        if self.dev_id.is_empty() {
            self.dev_id = std::str::from_utf8(dev_id_bytes).unwrap().to_string();
        }
    }
    
    pub fn set_region(&mut self, region_id: u8) {
        use CartridgeRegion::*;
        if matches!(self.region, Unidentified) {       
            self.region = match region_id {
                0x00 => Japan,
                0x01 => NorthAmerica,
                0x02 => Europe,
                0x03 => Sweden,
                0x04 => Finland,
                0x05 => Denmark,
                0x06 => France,
                0x07 => Netherlands,
                0x08 => Spain,
                0x09 => Germany,
                0x0A => Italy,
                0x0B => China,
                0x0C => Indonesia,
                0x0D => SouthKorea,
                0x0E => CommonInternational,
                0x0F => Canada,
                0x10 => Brazil,
                0x11 => Australia,
                _ => Unidentified,
            };
        }
    }
    
    pub fn set_version(&mut self, version: u8) {
        if self.version == 0 {
            self.version = version;
        }
    }
    
    pub fn set_title(&mut self, title_bytes: &[u8]) {
        if self.title.is_empty() {
            self.title = std::str::from_utf8(title_bytes).unwrap().to_string();
        }
    }
    
    pub fn set_maker_code(&mut self, maker_code: String) {
        if self.maker_code.is_empty() {
            self.maker_code = maker_code;
        }
    }
    
    pub fn set_game_code(&mut self, game_code: String) {
        if self.game_code.is_empty() {
            self.game_code = game_code;
        }
    }

    /// Set rom size if unset, 'size' refers to the `1 << size` from the header
    pub fn set_rom_size(&mut self, size: u8) {
        if self.rom_size == 0 {
            self.rom_size = 1 << size;
        }
    }

    /// Set ram size if unset, 'size' refers to the `1 << size` from the header
    pub fn set_ram_size(&mut self, size: u8) {
        if self.ram_size == 0 {
            self.ram_size = 1 << size;
        }
    }
}

