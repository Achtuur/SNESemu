use derive_more::{Display, From};

pub type CpuResult<T> = core::result::Result<T, CpuError>;

// uncomment this for test code
// pub type Error = Box<dyn std::error::Error>;

#[derive(Display, Debug, From)]
pub enum CpuError {
    #[display("Cartidge parse error: {}", _0)]
    CartridgeParseError(CartridgeParseError),
}

impl std::error::Error for CpuError {}



#[derive(Display, Debug)]
pub enum CartridgeParseError {
    #[display("Header not found at expected location")]
    HeaderNotFound,
    #[display("Invalid mapper mode found in header: {:#02X}", _0)]
    WrongMapperMode(u8),
    #[display("Checksum mismatch")]
    ChecksumMismatch,
    #[display("ROM size mismatch")]
    RomSizeMismatch,
    #[display("Invalid Game name")]
    InvalidGameName,
}

impl std::error::Error for CartridgeParseError {}
