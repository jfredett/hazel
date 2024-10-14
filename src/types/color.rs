use std::fmt::Display;
use std::ops::Not;

use crate::types::Bitboard;
use crate::types::Direction;
use crate::constants::{RANK_1, RANK_2, RANK_7, RANK_8};

use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Color {
    WHITE = 0,
    BLACK = 1,
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

    pub fn pawn_rank(self) -> Bitboard {
        match self {
            Color::WHITE => *RANK_2,
            Color::BLACK => *RANK_7,
        }
    }

    pub fn promotion_rank(self) -> Bitboard {
        match self {
            Color::WHITE => *RANK_8,
            Color::BLACK => *RANK_1,
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
    use super::*;

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
        assert_eq!(Color::WHITE.pawn_rank(), *RANK_2);
        assert_eq!(Color::BLACK.pawn_rank(), *RANK_7);
    }

    #[test]
    fn promotion_rank() {
        assert_eq!(Color::WHITE.promotion_rank(), *RANK_8);
        assert_eq!(Color::BLACK.promotion_rank(), *RANK_1);
    }

    #[test]
    fn color_count() {
        assert_eq!(COLOR_COUNT, 2);
    }
}
