use super::Direction;
use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Color {
    WHITE = 0,
    BLACK = 1
}

impl Color {
    pub fn pawn_direction(self) -> Direction {
        match self {
            Color::WHITE => { Direction::N }
            Color::BLACK => { Direction::S }
        }
    }
}

pub const COLORS : [Color; 2] = [
    Color::WHITE,
    Color::BLACK
];