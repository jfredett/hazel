use std::fmt::Display;
use std::ops::Not;

use crate::types::Bitboard;
use crate::types::Direction;
use crate::constants::{RANK_2, RANK_7};

use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Color {
    WHITE = 0,
    BLACK = 1,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::WHITE,
            1 => Color::BLACK,
            _ => panic!("Invalid color index"),
        }
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        color as u8
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::WHITE => write!(f, "w"),
            Color::BLACK => write!(f, "b"),
        }
    }
}

impl Color {
    pub fn pawn_direction(self) -> Direction {
        match self {
            Color::WHITE => Direction::N,
            Color::BLACK => Direction::S,
        }
    }

    /// a mask for the promotion rank
    pub fn promotion_rank(self) -> usize {
        (!self).pawn_rank()
    }

    /// a mask for the promotion rank
    pub fn promotion_mask(self) -> Bitboard {
        (!self).pawn_mask()
    }

    pub fn en_passant_rank(self) -> usize {
        match self {
            Color::WHITE => 5,
            Color::BLACK => 2,
        }
    }

    pub fn pawn_rank(self) -> usize {
        match self {
            Color::WHITE => 1,
            Color::BLACK => 6,
        }
    }

    pub fn pawn_mask(self) -> Bitboard {
        match self {
            Color::WHITE => *RANK_2,
            Color::BLACK => *RANK_7,
        }
    }

    pub fn is_black(self) -> bool {
        self == Color::BLACK
    }

    pub fn is_white(self) -> bool {
        self == Color::WHITE
    }
}

pub const COLOR_COUNT: usize = 2;
pub const COLORS: [Color; COLOR_COUNT] = [Color::WHITE, Color::BLACK];

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }
}


#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;

    impl Arbitrary for Color {
        fn arbitrary(g: &mut Gen) -> Self {
            if bool::arbitrary(g) {
                Color::WHITE
            } else {
                Color::BLACK
            }
        }
    }

    #[test]
    fn is_white() {
        assert!(Color::WHITE.is_white());
        assert!(!Color::BLACK.is_white());
    }

    #[test]
    fn is_black() {
        assert!(Color::BLACK.is_black());
        assert!(!Color::WHITE.is_black());
    }

    #[test]
    fn not() {
        assert_eq!(!Color::WHITE, Color::BLACK);
        assert_eq!(!Color::BLACK, Color::WHITE);
    }

    #[test]
    fn pawn_direction() {
        assert_eq!(Color::WHITE.pawn_direction(), Direction::N);
        assert_eq!(Color::BLACK.pawn_direction(), Direction::S);
    }

    #[test]
    fn pawn_rank() {
        assert_eq!(Color::WHITE.pawn_rank(), 1);
        assert_eq!(Color::BLACK.pawn_rank(), 6);
    }
    
    #[test]
    fn pawn_mask() {
        assert_eq!(Color::WHITE.pawn_mask(), *RANK_2);
        assert_eq!(Color::BLACK.pawn_mask(), *RANK_7);
    }

    #[test]
    fn promotion_rank() {
        assert_eq!(Color::WHITE.promotion_rank(), 6);
        assert_eq!(Color::BLACK.promotion_rank(), 1);
    }

    #[test]
    fn color_count() {
        assert_eq!(COLOR_COUNT, 2);
    }
}
