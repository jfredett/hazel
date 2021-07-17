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
    
    pub fn is_black(self) -> bool {
        self == Color::BLACK
    }
    
    pub fn is_white(self) -> bool {
        self == Color::WHITE
    }
}

pub const COLORS : [Color; 2] = [
    Color::WHITE,
    Color::BLACK
];