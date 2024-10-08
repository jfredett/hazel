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

    pub const fn is_empty(&self) -> bool {
        matches!(self, Occupant::Empty)
    }

    pub const fn is_occupied(&self) -> bool {
        !self.is_empty()
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

    #[inline(always)] pub const fn white_king() -> Self { Self::white(Piece::King) }
    #[inline(always)] pub const fn white_queen() -> Self { Self::white(Piece::Queen) }
    #[inline(always)] pub const fn white_rook() -> Self { Self::white(Piece::Rook) }
    #[inline(always)] pub const fn white_bishop() -> Self { Self::white(Piece::Bishop) }
    #[inline(always)] pub const fn white_knight() -> Self { Self::white(Piece::Knight) }
    #[inline(always)] pub const fn white_pawn() -> Self { Self::white(Piece::Pawn) }

    #[inline(always)] pub const fn black_king() -> Self { Self::black(Piece::King) }
    #[inline(always)] pub const fn black_queen() -> Self { Self::black(Piece::Queen) }
    #[inline(always)] pub const fn black_rook() -> Self { Self::black(Piece::Rook) }
    #[inline(always)] pub const fn black_bishop() -> Self { Self::black(Piece::Bishop) }
    #[inline(always)] pub const fn black_knight() -> Self { Self::black(Piece::Knight) }
    #[inline(always)] pub const fn black_pawn() -> Self { Self::black(Piece::Pawn) }

    #[inline(always)] pub const fn rook(color: Color) -> Self { Self::Occupied(Piece::Rook, color) }
    #[inline(always)] pub const fn knight(color: Color) -> Self { Self::Occupied(Piece::Knight, color) }
    #[inline(always)] pub const fn bishop(color: Color) -> Self { Self::Occupied(Piece::Bishop, color) }
    #[inline(always)] pub const fn queen(color: Color) -> Self { Self::Occupied(Piece::Queen, color) }
    #[inline(always)] pub const fn king(color: Color) -> Self { Self::Occupied(Piece::King, color) }
    #[inline(always)] pub const fn pawn(color: Color) -> Self { Self::Occupied(Piece::Pawn, color) }
}

impl Display for Occupant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Occupant::Occupied(piece, color) => write!(f, "{}", piece.to_fen(*color)),
            Occupant::Empty => write!(f, ".")
        }
    }
}
