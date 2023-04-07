pub mod lorom;
pub mod hirom;
pub mod exhirom;

use super::cartridge::{CartridgeMetadata, CartridgeParseError};


pub trait Mappermode {

    /// Handles reading memory in the following regions:
    /// * Q1 upper half
    /// * Q2
    /// * Q3 upper half
    /// * Q4
    fn read(&self, long_addr: u32) -> Option<u8>;

    /// Handles writing to memory in the following regions:
    /// * Q1 upper half
    /// * Q2
    /// * Q3 upper half
    /// * Q4
    fn write(&mut self, long_addr: u32, value: u8);

    /// Copy raw bytes to internal rom vector
    fn copy_bytes_to_rom(&mut self, bytes: &[u8]);

    fn get_rom_size(&self) -> usize;

    /// Set size of ram where size indicates `1 << size` kB of ram
    fn set_ram_size(&mut self, size: u8);

    /// Returns size of sram in bytes
    fn get_sram_size(&self) -> usize;

    /// Returns all bytes currently in SRAM, ordered from lowest to highest address
    fn get_sram_bytes(&self) -> Vec<u8>;

    /// Copy raw bytes to internal sram vector, overwrites any previously existing value!
    /// 
    /// Only for use when loading a save
    fn copy_bytes_to_sram(&mut self, bytes: &[u8]);

    /// Returns memory map mode number that is expected to be found in header
    fn get_memory_map_mode(&self) -> u8;


    /// Reset mapper to initial state
    /// 
    /// clears ram and rom completely
    fn reset(&mut self);
    
    /// Try parse cartridge using this mapper mode
    /// 
    /// # Inputs
    /// 
    /// * `raw_bytes` - raw bytes from the cartridge
    fn try_parse(&mut self, raw_bytes: &[u8]) -> Result<CartridgeMetadata, CartridgeParseError> {

        /// Macro that does self.read(addr).unwrap()
        macro_rules! readf {
            ($addr: expr) => {
                self.read($addr).unwrap()
            }
        }
        
        self.copy_bytes_to_rom(raw_bytes);

        let checksum = (readf!(0xFFDC) as u16) << 8 | readf!(0xFFDD) as u16;
        let checksum_compl = (readf!(0xFFDE) as u16) << 8 | readf!(0xFFDF) as u16;

        if checksum + checksum_compl != 0xFFFF {
            return Err(CartridgeParseError::ChecksumMismatch)
        }

        let mut metadata = CartridgeMetadata::new();

        // header version 3 
        if readf!(0xFFDA) == 0x33 {
            // First two bytes are developer ID
            let dev_id = &[readf!(0x00FFB0), readf!(0x00FFB1)]; // raw bytes
            metadata.set_dev_id(dev_id);
            
            // Next four bytes are game code
            let game_code = (0..4).map(|offset| readf!(0xFFB2 + offset)).collect::<Vec<u8>>();
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

        let game_name = (0..21).map(|offset| readf!(0xFFC0 + offset)).collect::<Vec<u8>>();

        // check if every character except the last is either a space or a capital letter
        if game_name[0..20].iter().any(|byte| (*byte != 0x20 && *byte < 0x41) || *byte > 0x5A) {
            return Err(CartridgeParseError::InvalidGameName);
        }

        metadata.set_title(&game_name);

        // map mode: 001smmmm
        //              |++++ - Map mode (0: lorom, 1: hirom, 2: exhirom)
        //              +------ Speed: (0: slow, 1: fast)
        let map_mode = readf!(0xFFD5);

        if map_mode & 0x0F != self.get_memory_map_mode() {
            return Err(CartridgeParseError::WrongMapperMode)
        }

        let cart_type = readf!(0xFFD6);
        
        let rom_size = readf!(0xFFD7); // 1 << rom_size kB

        if (1 << rom_size) >= raw_bytes.len() {
            return Err(CartridgeParseError::RomSizeMismatch)
        } 

        metadata.set_rom_size(rom_size);

        let ram_size = readf!(0xFFD8); // 1 << ram_size kB
        self.set_ram_size(ram_size);

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