use super::*;

use std::fmt::Debug;


impl From<Bitboard> for u64 {
    fn from(val: Bitboard) -> Self {
        val.0
    }
}

impl<N : TryInto<Square>> From<N> for Bitboard where N : Debug {
    fn from(n: N) -> Bitboard {
        if let Ok(sq) = n.try_into() {
            Bitboard(1 << sq.index())
        } else {
            panic!("Invalid square specification");
        }
    }
}


impl Bitboard {
    /// From is not const, which is fine, but I do need to build these at compile time.
    pub const fn const_from(b: u64) -> Bitboard {
        Bitboard(b)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_const_from_u64() {
        // technically covered by the standard from case, but belt and.
        let b = Bitboard::const_from(0x0000000000000001u64);
        assert_eq!(b, Bitboard(0x0000000000000001u64));
    }

    #[test]
    fn bitboard_from_u64() {
        let b = Bitboard::from(0x0000000000000000u64);
        assert_eq!(b, Bitboard(0x0000000000000001u64));
    }

    #[test]
    fn u64_from_bitboard() {
        let b = Bitboard(0x0000000000000001u64);
        assert_eq!(u64::from(b), 0x0000000000000001u64);
    }

    #[test]
    fn bitboard_from_usize() {
        let b = Bitboard::from(0x00000002usize);
        assert_eq!(b, Bitboard(0x0000000000000004));
    }
}
