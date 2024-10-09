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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupant_display() {
        assert_eq!(Occupant::white_king().to_string(), "K");
        assert_eq!(Occupant::black_king().to_string(), "k");
        assert_eq!(Occupant::white_queen().to_string(), "Q");
        assert_eq!(Occupant::black_queen().to_string(), "q");
        assert_eq!(Occupant::white_rook().to_string(), "R");
        assert_eq!(Occupant::black_rook().to_string(), "r");
        assert_eq!(Occupant::white_bishop().to_string(), "B");
        assert_eq!(Occupant::black_bishop().to_string(), "b");
        assert_eq!(Occupant::white_knight().to_string(), "N");
        assert_eq!(Occupant::black_knight().to_string(), "n");
        assert_eq!(Occupant::white_pawn().to_string(), "P");
        assert_eq!(Occupant::black_pawn().to_string(), "p");
        assert_eq!(Occupant::empty().to_string(), ".");
    }

    #[test]
    fn occupant_white() {
        assert_eq!(Occupant::white_king(), Occupant::white(Piece::King));
        assert_eq!(Occupant::white_queen(), Occupant::white(Piece::Queen));
        assert_eq!(Occupant::white_rook(), Occupant::white(Piece::Rook));
        assert_eq!(Occupant::white_bishop(), Occupant::white(Piece::Bishop));
        assert_eq!(Occupant::white_knight(), Occupant::white(Piece::Knight));
        assert_eq!(Occupant::white_pawn(), Occupant::white(Piece::Pawn));
    }

    #[test]
    fn occupant_black() {
        assert_eq!(Occupant::black_king(), Occupant::black(Piece::King));
        assert_eq!(Occupant::black_queen(), Occupant::black(Piece::Queen));
        assert_eq!(Occupant::black_rook(), Occupant::black(Piece::Rook));
        assert_eq!(Occupant::black_bishop(), Occupant::black(Piece::Bishop));
        assert_eq!(Occupant::black_knight(), Occupant::black(Piece::Knight));
        assert_eq!(Occupant::black_pawn(), Occupant::black(Piece::Pawn));
    }

    #[test]
    fn occupant_color_piece() {
        assert_eq!(Occupant::rook(Color::WHITE), Occupant::Occupied(Piece::Rook, Color::WHITE));
        assert_eq!(Occupant::knight(Color::WHITE), Occupant::Occupied(Piece::Knight, Color::WHITE));
        assert_eq!(Occupant::bishop(Color::WHITE), Occupant::Occupied(Piece::Bishop, Color::WHITE));
        assert_eq!(Occupant::queen(Color::WHITE), Occupant::Occupied(Piece::Queen, Color::WHITE));
        assert_eq!(Occupant::king(Color::WHITE), Occupant::Occupied(Piece::King, Color::WHITE));
        assert_eq!(Occupant::pawn(Color::WHITE), Occupant::Occupied(Piece::Pawn, Color::WHITE));

        assert_eq!(Occupant::rook(Color::BLACK), Occupant::Occupied(Piece::Rook, Color::BLACK));
        assert_eq!(Occupant::knight(Color::BLACK), Occupant::Occupied(Piece::Knight, Color::BLACK));
        assert_eq!(Occupant::bishop(Color::BLACK), Occupant::Occupied(Piece::Bishop, Color::BLACK));
        assert_eq!(Occupant::queen(Color::BLACK), Occupant::Occupied(Piece::Queen, Color::BLACK));
        assert_eq!(Occupant::king(Color::BLACK), Occupant::Occupied(Piece::King, Color::BLACK));
        assert_eq!(Occupant::pawn(Color::BLACK), Occupant::Occupied(Piece::Pawn, Color::BLACK));
    }

    #[test]
    fn occupant_color() {
        assert_eq!(Occupant::white_king().color(), Some(Color::WHITE));
        assert_eq!(Occupant::black_king().color(), Some(Color::BLACK));
        assert_eq!(Occupant::white_queen().color(), Some(Color::WHITE));
        assert_eq!(Occupant::black_queen().color(), Some(Color::BLACK));
        assert_eq!(Occupant::white_rook().color(), Some(Color::WHITE));
        assert_eq!(Occupant::black_rook().color(), Some(Color::BLACK));
        assert_eq!(Occupant::white_bishop().color(), Some(Color::WHITE));
        assert_eq!(Occupant::black_bishop().color(), Some(Color::BLACK));
        assert_eq!(Occupant::white_knight().color(), Some(Color::WHITE));
        assert_eq!(Occupant::black_knight().color(), Some(Color::BLACK));
        assert_eq!(Occupant::white_pawn().color(), Some(Color::WHITE));
        assert_eq!(Occupant::black_pawn().color(), Some(Color::BLACK));
        assert_eq!(Occupant::empty().color(), None);
    }

    #[test]
    fn occupant_piece() {
        assert_eq!(Occupant::white_king().piece(), Some(Piece::King));
        assert_eq!(Occupant::black_king().piece(), Some(Piece::King));
        assert_eq!(Occupant::white_queen().piece(), Some(Piece::Queen));
        assert_eq!(Occupant::black_queen().piece(), Some(Piece::Queen));
        assert_eq!(Occupant::white_rook().piece(), Some(Piece::Rook));
        assert_eq!(Occupant::black_rook().piece(), Some(Piece::Rook));
        assert_eq!(Occupant::white_bishop().piece(), Some(Piece::Bishop));
        assert_eq!(Occupant::black_bishop().piece(), Some(Piece::Bishop));
        assert_eq!(Occupant::white_knight().piece(), Some(Piece::Knight));
        assert_eq!(Occupant::black_knight().piece(), Some(Piece::Knight));
        assert_eq!(Occupant::white_pawn().piece(), Some(Piece::Pawn));
        assert_eq!(Occupant::black_pawn().piece(), Some(Piece::Pawn));
        assert_eq!(Occupant::empty().piece(), None);
    }

    #[test]
    fn occupant_is_empty() {
        assert!(Occupant::empty().is_empty());
        assert!(!Occupant::white_king().is_empty());
        assert!(!Occupant::black_king().is_empty());
        assert!(!Occupant::white_queen().is_empty());
        assert!(!Occupant::black_queen().is_empty());
        assert!(!Occupant::white_rook().is_empty());
        assert!(!Occupant::black_rook().is_empty());
        assert!(!Occupant::white_bishop().is_empty());
        assert!(!Occupant::black_bishop().is_empty());
        assert!(!Occupant::white_knight().is_empty());
        assert!(!Occupant::black_knight().is_empty());
        assert!(!Occupant::white_pawn().is_empty());
        assert!(!Occupant::black_pawn().is_empty());
    }

    #[test]
    fn occupant_is_occupied() {
        assert!(!Occupant::empty().is_occupied());
        assert!(Occupant::white_king().is_occupied());
        assert!(Occupant::black_king().is_occupied());
        assert!(Occupant::white_queen().is_occupied());
        assert!(Occupant::black_queen().is_occupied());
        assert!(Occupant::white_rook().is_occupied());
        assert!(Occupant::black_rook().is_occupied());
        assert!(Occupant::white_bishop().is_occupied());
        assert!(Occupant::black_bishop().is_occupied());
        assert!(Occupant::white_knight().is_occupied());
        assert!(Occupant::black_knight().is_occupied());
        assert!(Occupant::white_pawn().is_occupied());
        assert!(Occupant::black_pawn().is_occupied());
    }


}

