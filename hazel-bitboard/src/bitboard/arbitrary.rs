#![cfg(test)]

pub use quickcheck::{Arbitrary, Gen};

use super::*;

impl Arbitrary for Bitboard {
    fn arbitrary(g: &mut Gen) -> Bitboard {
        Bitboard::new(u64::arbitrary(g))
    }
}
