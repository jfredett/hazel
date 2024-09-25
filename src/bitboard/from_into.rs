use super::*;

impl From<u64> for Bitboard {
    fn from(u: u64) -> Self {
        Bitboard::from(u)
    }
}

impl From<Bitboard> for u64 {
    fn from(val: Bitboard) -> Self {
        val.0
    }
}

// NOTE: hazel expects to be run on a 64b machine. If you want to run it on a 32b machine. Don't.
impl From<usize> for Bitboard {
    fn from(u: usize) -> Self {
        Bitboard::from(u as u64)
    }
}

// NOTE: hazel expects to be run on a 64b machine. If you want to run it on a 32b machine. Don't.
impl From<Bitboard> for usize {
    fn from(val: Bitboard) -> Self {
        val.0 as usize
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_from_u64() {
        let b = Bitboard::from(0x0000000000000001);
        assert_eq!(b, Bitboard(0x0000000000000001));
    }

    #[test]
    fn u64_from_bitboard() {
        let b = Bitboard(0x0000000000000001);
        assert_eq!(u64::from(b), 0x0000000000000001);
    }

    #[test]
    fn bitboard_from_usize() {
        let b = Bitboard::from(0x00000001);
        assert_eq!(b, Bitboard(0x0000000000000001));
    }

    #[test]
    fn usize_from_bitboard() {
        let b = Bitboard(0x00000001);
        assert_eq!(usize::from(b), 0x0000000000000001);
    }



}
