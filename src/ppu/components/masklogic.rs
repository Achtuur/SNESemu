pub enum MaskLogic {
    Or,
    And,
    Xor,
    Xnor,
}

impl MaskLogic {
    // Return mask logic based on 2 bits that is in WLOG registers
    pub fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::Or,
            0b01 => Self::And,
            0b10 => Self::Xor,
            0b11 => Self::Xnor,
            _ => unreachable!(),
        }
    }

    /// Perform masking logic on 2 booleans (act as a 2 input logic gate)
    pub fn mask(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Self::Or => lhs || rhs,
            Self::And => lhs && rhs,
            Self::Xor => lhs ^ rhs,
            Self::Xnor => !(lhs ^ rhs),
        }
    }
}