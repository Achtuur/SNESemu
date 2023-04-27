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
        // use crate::nth_bit;
        let n_bits = $m - $n + 1;
        let mask = (1 << n_bits) - 1; // get mask 0b..111 where number of 1's is n_bits

        let mask = mask << $n; // shift to left by $n bits to place it correctly
        let b = ($num & mask) >> $n; // mask $num and shift it to the right
        b
    }};

    ($num: expr, $n: expr, $m: expr) => {{
        let n = $n as usize;
        let m = $m as usize;
        // use crate::nth_bit;
        let n_bits = $m - $n + 1;
        let mask = (1 << n_bits) - 1; // get mask 0b..111 where number of 1's is n_bits

        let mask = mask << $n; // shift to left by $n bits to place it correctly
        let b = ($num & mask) >> $n; // mask $num and shift it to the right
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

#[cfg(test)]
mod tests {
    #[test]
    fn bit_slice() {
        let x = 0b0110_1001;
        let y = bit_slice!(x, 0, 3);
        assert_eq!(y, 0b1001);
    }
}