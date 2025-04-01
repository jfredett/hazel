pub mod bitboard;

pub mod constants;
pub mod extensions;
pub mod pextboard;

pub use extensions::*;

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;


#[cfg(test)]
mod test {
    use hazel_core::{color::Color, piece::Piece, square::Square};

    use crate::bitboard::Bitboard;

    use super::*;

    #[quickcheck]
    fn bitboard_moves_match_square_moves(piece: Piece, color: Color, sq: Square) -> bool {
        // Ignore Pawns, Kings, and Knights for now, so we can use the pextboard stuff
        if piece == Piece::Pawn { return true; }
        if piece == Piece::King { return true; }
        if piece == Piece::Knight { return true; }

        // this is a bitboard set with all the moves for a given piece
        let bbmoves = pextboard::attacks_for(piece, sq, Bitboard::empty());

        sq.moves_for(&piece, &color).all(|x| bbmoves.is_set(x))
    }
}
