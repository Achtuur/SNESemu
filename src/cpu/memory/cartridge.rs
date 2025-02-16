use crate::cpu::CartridgeParseError;

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
pub enum CartridgeMapperType {
    LoROM,
    HiROM,
    ExHiROM,
}

#[derive(Debug, Default)]
pub struct CartridgeMetadata<'a> {
    pub rom_bytes: &'a [u8],
    pub title: String,
    pub dev_id: Option<String>,
    pub region: Option<CartridgeRegion>,
    pub version: Option<u8>,
    pub maker_code: Option<String>,
    pub game_code: Option<String>,
    pub rom_size: usize,
    pub ram_size: usize,
    pub has_copier_bytes: bool,
    pub mapper: Option<CartridgeMapperType>,
}

impl CartridgeMetadata<'_> {
    pub fn new() -> Self {
        CartridgeMetadata {
            ..Default::default()
        }
    }

    pub fn set_dev_id(&mut self, dev_id_bytes: &[u8]) {
        if self.dev_id.is_none() {
            self.dev_id = Some(std::str::from_utf8(dev_id_bytes).unwrap().to_string());
        }
    }

    pub fn set_region(&mut self, region_id: u8) {
        use CartridgeRegion as R;
        if self.region.is_none() {
            self.region = match region_id {
                0x00 => Some(R::Japan),
                0x01 => Some(R::NorthAmerica),
                0x02 => Some(R::Europe),
                0x03 => Some(R::Sweden),
                0x04 => Some(R::Finland),
                0x05 => Some(R::Denmark),
                0x06 => Some(R::France),
                0x07 => Some(R::Netherlands),
                0x08 => Some(R::Spain),
                0x09 => Some(R::Germany),
                0x0A => Some(R::Italy),
                0x0B => Some(R::China),
                0x0C => Some(R::Indonesia),
                0x0D => Some(R::SouthKorea),
                0x0E => Some(R::CommonInternational),
                0x0F => Some(R::Canada),
                0x10 => Some(R::Brazil),
                0x11 => Some(R::Australia),
                _ => Some(R::Unidentified),
            };
        }
    }

    pub fn set_version(&mut self, version: u8) {
        if self.version.is_none() {
            self.version = Some(version);
        }
    }

    pub fn set_title(&mut self, title_bytes: &[u8]) {
        self.title = std::str::from_utf8(title_bytes).unwrap().to_string();
    }

    pub fn set_maker_code(&mut self, maker_code: String) {
        if self.maker_code.is_none() {
            self.maker_code = Some(maker_code);
        }
    }

    pub fn set_game_code(&mut self, game_code: String) {
        if self.game_code.is_none() {
            self.game_code = Some(game_code);
        }
    }

    /// Set rom size if unset, 'size' refers to the `1 << size` from the header
    pub fn set_rom_size(&mut self, size: u8) {
        self.rom_size = 1 << size;
    }

    /// Set ram size if unset, 'size' refers to the `1 << size` from the header
    pub fn set_ram_size(&mut self, size: u8) {
        self.ram_size = 1 << size;
    }

    pub fn set_mapper(&mut self, mapper_byte: u8) -> Result<(), CartridgeParseError> {
        let mapper = match mapper_byte {
            0x00 => CartridgeMapperType::LoROM,
            0x01 => CartridgeMapperType::HiROM,
            0x05 => CartridgeMapperType::ExHiROM,
            _ => return Err(CartridgeParseError::WrongMapperMode(mapper_byte)),
        };
        self.mapper = Some(mapper);
        Ok(())
    }

    /// Try parse cartridge using this mapper mode
    ///
    /// # Inputs
    ///
    /// * `raw_bytes` - raw bytes from the cartridge
    pub(crate) fn try_parse(raw_bytes: &[u8]) -> Result<CartridgeMetadata, CartridgeParseError> {
        /// The address is based on CPU memory map
        ///
        /// Cartridge header is always at the same place, regardless of mapper
        /// So we can subtract the address by 0x8000 to get the correct address
        macro_rules! readf {
            ($addr: expr) => {{
                let addr = $addr as usize - 0x8000;
                let x = raw_bytes[$addr as usize - 0x8000];
                tracing::trace!("Cartridge init: rom[{:#04X}] = {:#04X}", addr, x);
                x
            }};
        }

        let mut metadata = CartridgeMetadata::new();
        metadata.rom_bytes = raw_bytes;

        let checksum = (readf!(0xFFDC) as u16) << 8 | readf!(0xFFDD) as u16;
        let checksum_compl = (readf!(0xFFDE) as u16) << 8 | readf!(0xFFDF) as u16;

        if checksum + checksum_compl != 0xFFFF {
            return Err(CartridgeParseError::ChecksumMismatch);
        }

        // header version 3
        if readf!(0xFFDA) == 0x33 {
            // First two bytes are developer ID
            let dev_id = &[readf!(0x00FFB0), readf!(0x00FFB1)]; // raw bytes
            metadata.set_dev_id(dev_id);

            // Next four bytes are game code
            let game_code = (0..4)
                .map(|offset| readf!(0xFFB2 + offset))
                .collect::<Vec<u8>>();
            let game_code = std::str::from_utf8(&game_code).unwrap().to_string();

            let ex_flash_size = readf!(0xFFBC); // 1 << exflash_size kB
            let ex_ram_size = readf!(0xFFBD); // 1 << ex_ram_size kB

            let chipset_subtype = readf!(0xFFBF);

            metadata.set_game_code(game_code);
            // metadata.set_chipset_subtype(chipset_subtype);
        }
        // header version 2
        else if readf!(0xFFD4) == 0 && (0..15).all(|offset| readf!(0xFFB0 + offset) == 0x00) {
            let chipset_subtype = readf!(0xFFBF);
        }

        // Normal header - starts at $FFC0

        let game_name = (0..21)
            .map(|offset| readf!(0xFFC0 + offset))
            .collect::<Vec<u8>>();

        // check if every character except the last is either a space or a capital letter
        // if game_name[0..20]
        //     .iter()
        //     .any(|byte| !matches!(byte, 0x20 | 0x41..=0x5A))
        // {
        //     return Err(CartridgeParseError::InvalidGameName);
        // }

        metadata.set_title(&game_name);

        // map mode: 001smmmm
        //              |++++ - Map mode (0: lorom, 1: hirom, 2: exhirom)
        //              +------ Speed: (0: slow, 1: fast)
        let map_mode_byte = readf!(0xFFD5);
        // only take lower nybble
        metadata.set_mapper(map_mode_byte & 0xF)?;

        let cart_type = readf!(0xFFD6);

        let rom_size = readf!(0xFFD7); // 1 << rom_size kB

        if (1 << rom_size) >= raw_bytes.len() {
            return Err(CartridgeParseError::RomSizeMismatch);
        }

        metadata.set_rom_size(rom_size);

        let ram_size = readf!(0xFFD8); // 1 << ram_size kB
        metadata.set_ram_size(ram_size);

        let country = readf!(0xFFD9);
        metadata.set_region(country);

        let dev_id = readf!(0xFFDA);
        metadata.set_dev_id(&[dev_id]);

        let version = readf!(0xFFDB);
        metadata.set_version(version);

        Ok(metadata)
    }
}
