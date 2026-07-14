pub fn set_bit(value: &mut u8, bit: u8, high: bool) {
    if high {
        *value |= 1 << bit;
    } else {
        *value &= !(1 << bit);
    }
}

pub fn get_bit(value: u8, bit: u8) -> bool {
    (value & (1 << bit)) != 0
}

pub fn u16_to_u8s(v: u16) -> (u8, u8) {
    ((v >> 8) as u8, v as u8)
}

pub fn u8s_to_u16(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) | (lo as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_bit_high() {
        let mut v = 0b0000_0000;
        set_bit(&mut v, 3, true);
        assert_eq!(v, 0b0000_1000);
    }

    #[test]
    fn set_bit_low() {
        let mut v = 0b1111_1111;
        set_bit(&mut v, 3, false);
        assert_eq!(v, 0b1111_0111);
    }

    #[test]
    fn get_bit_true() {
        assert!(get_bit(0b0000_1000, 3));
    }

    #[test]
    fn get_bit_false() {
        assert!(!get_bit(0b0000_1000, 2));
    }

    #[test]
    fn u16_to_u8s_roundtrip() {
        let (hi, lo) = u16_to_u8s(0xABCD);
        assert_eq!(hi, 0xAB);
        assert_eq!(lo, 0xCD);
        assert_eq!(u8s_to_u16(hi, lo), 0xABCD);
    }
}
