#[macro_export]
/// Get `nth` bit
macro_rules! nth_bit {
    ($num: expr, $n: literal) => {
        ($num >> $n) & 1
    };

    ($num: expr, $n: expr) => {{
        let m = $n as usize;
        ($num >> m) & 1
    }}
}

#[macro_export]
/// Get bits nth to mth bit of num, including mth bit
/// 
/// `bit_slice!(0b0110_1001, 0, 3)` returns `0b1001`
macro_rules! bit_slice {
    ($num: expr, $n: literal, $m: literal) => {{
        use crate::nth_bit;
        let mut b = 0;
        for i in $n..=$m {
            // get shift amount
            let shift = i - $n;
            b |= nth_bit!($num, i) << shift;
        }
        b
    }};

    ($num: expr, $n: expr, $m: expr) => {{
        use crate::nth_bit;
        let n = $n as usize;
        let m = $m as usize;
        let mut b = 0;
        for i in n..=m {
            // get shift amount
            let shift = i - n;
            b |= nth_bit!($num, i) << shift;
        }
        b
    }};
}

#[macro_export]
/// Check if nth bit is set
macro_rules! bit_set {
    ($num: expr, $n: literal) => {{
        use crate::nth_bit;
        nth_bit!($num, $n) == 1
    }};

    ($num: expr, $n: expr) => {{
        use crate::nth_bit;
        nth_bit!($num, $n) == 1
    }}
}

#[macro_export]
macro_rules! bank_byte {
    ($num: expr) => {
        ($num >> 16) as u8
    };
}

#[macro_export]
macro_rules! high_byte {
    ($num: expr) => {
        ($num >> 8) as u8
    }
}

#[macro_export]
macro_rules! low_byte {
    ($num: expr) => {
        $num as u8
    }
}