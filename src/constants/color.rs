use std::ops::Not;

use crate::bitboard::Bitboard;
use crate::constants::{RANK_1, RANK_2, RANK_7, RANK_8};

use super::Direction;
use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Color {
    WHITE = 0,
    BLACK = 1,
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
