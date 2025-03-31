use hazel_core::color::Color;
use hazel_core::file::File;

use crate::bitboard::Bitboard;
use crate::constants::masks::{FILE_MASKS, RANK_2, RANK_7};

pub trait ColorMasks {
    /// a mask for the promotion rank
    fn promotion_mask(self) -> Bitboard;

    // A mask for the starting rank for pawns
    fn pawn_mask(self) -> Bitboard;
}

pub trait AsBitboard {
    fn as_bitboard(self) -> Bitboard;
}

impl AsBitboard for File {
    fn as_bitboard(self) -> Bitboard {
        FILE_MASKS[self as usize]
    }

}

// Extend
impl ColorMasks for Color {
    fn promotion_mask(self) -> Bitboard {
        (!self).pawn_mask()
    }

    fn pawn_mask(self) -> Bitboard {
        match self {
            Color::WHITE => *RANK_2,
            Color::BLACK => *RANK_7,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn promotion_mask() {
        assert_eq!(Color::WHITE.pawn_mask(), *RANK_2);
        assert_eq!(Color::BLACK.pawn_mask(), *RANK_7);
    }

    #[test]
    fn pawn_mask() {
        assert_eq!(Color::WHITE.pawn_mask(), *RANK_2);
        assert_eq!(Color::BLACK.pawn_mask(), *RANK_7);
    }
}
