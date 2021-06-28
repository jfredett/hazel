use super::Direction;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
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