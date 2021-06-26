use crate::bitboard;

use super::*;


impl From<u64> for Bitboard {
    fn from(u: u64) -> Self {
        Bitboard::from(u)
    }
}

impl Into<u64> for Bitboard {
    fn into(self) -> u64 {
        self.0
    }
}