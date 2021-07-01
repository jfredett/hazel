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

