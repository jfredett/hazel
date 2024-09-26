use std::fmt::Display;
use crate::constants::{piece::Piece, color::Color};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Occupant {
    Occupied(Piece, Color),
    #[default] Empty
}

impl Occupant {
    pub const fn empty() -> Self {
        Self::Empty
    }

    pub const fn white(piece: Piece) -> Self {
        Self::Occupied(piece, Color::WHITE)
    }

    pub const fn black(piece: Piece) -> Self {
        Self::Occupied(piece, Color::BLACK)
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            Occupant::Occupied(_, color) => Some(*color),
            Occupant::Empty => None
        }
    }

    pub fn piece(&self) -> Option<Piece> {
        match self {
            Occupant::Occupied(piece, _) => Some(*piece),
            Occupant::Empty => None
        }
    }
}

impl Display for Occupant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Occupant::Occupied(piece, color) => write!(f, "{}", piece.to_fen(*color)),
            Occupant::Empty => write!(f, ".")
        }
    }
}
