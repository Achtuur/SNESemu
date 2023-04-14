#[macro_export]
/// Separates long address `$BBHHLL` as tuple `($BB, $HHLL)`
/// 
/// Any bit above the 24th in `long_addr` is ignored
macro_rules! separate_bank_hhll_addr {
    ($long_addr: expr) => {{
        let bank: u8 = (($long_addr & 0xFF0000) >> 16) as u8;
        let hi_lo_byte: u16 = ($long_addr & 0x00FFFF) as u16;
        (bank, hi_lo_byte)
    }};
}

#[macro_export]
/// Creates a word from two bytes, first pass the high byte, then the low byte
macro_rules! to_word {
    ($hh: expr, $ll: expr) => {
        (($hh as u16) << 8) | ($ll as u16)
    };
}


#[macro_export]
/// Combines `pointer_bank ($BB)`, `pointer_high ($HH)`, `pointer_low ($LL)` into `$BBHHLL`
/// 
/// Or combines `$bank ($BB)` and `$hhll ($HHLL)` into `$BBHHLL`
macro_rules! to_long {
    ($bank: expr, $hh: expr, $ll: expr) => {{
        (($bank as u32) << 16) | (($hh as u32) << 8) | ($ll as u32)
    }};

    ($bank: expr, $hhll: expr) => {
        to_long!($bank, ($hhll >> 8) as u8, $hhll as u8)
    }
}

#[macro_export]
/// Set bank of address specified by `$addr`
/// 
/// The lower 8 bits of `$bank` are used as the new bank, 
/// and the lower 16 bits of `$addr` makes up the rest of the address
macro_rules! set_bb {
    ($addr: expr, $bank: expr) => {
        ($addr as u32) & 0x00FFFF | ((($bank as u32) & 0xFF) << 16)
    }
}

#[macro_export]
macro_rules! set_hh {
    ($addr: expr, $hh: expr) => {
        ($addr as u32) & 0xFF00FF | (($hh as u32) & 0xFF) << 8
    };
}


#[macro_export]
macro_rules! set_ll {
    ($addr: expr, $ll: expr) => {
        ($addr as u32) & 0xFFFF00 | ($ll as u32) & 0xFF
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_to_word() {
        let x = 0x12;
        let y = 0x34;
        let z = to_word!(x, y);
        assert_eq!(z, 0x1234);
    }

    #[test]
    fn test_to_long() {
        let x = 0x12;
        let y = 0x34;
        let z = 0x56;
        let w = to_long!(x, y, z);
        assert_eq!(w, 0x123456);
    }

    #[test]
    fn test_separate_hhll_bank_address() {
        let addr = 0x123456;
        let (bb, hhll) = separate_bank_hhll_addr!(addr);
        
        assert_eq!(bb, 0x12);
        assert_eq!(hhll, 0x3456);
    }

    #[test]
    fn test_set_ll() {
        let x: u16 = 0xABCD;
        let y = 0x12;
        let z = set_ll!(x, y);
        assert_eq!(z, 0xAB12);
    }

    #[test]
    fn test_set_hh() {
        let x: u16 = 0xABCD;
        let y = 0x12;
        let z = set_hh!(x, y);
        assert_eq!(z, 0x12CD);
    }

    #[test]
    fn test_set_bb() {
        let x: u16 = 0xABCD;
        let y = 0x12;
        let z = set_bb!(x, y);
        assert_eq!(z, 0x12ABCD);
    }
}