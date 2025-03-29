use quickcheck::{Arbitrary, Gen};
use serde::{Deserialize, Serialize};

use crate::{color::{Color, COLOR_COUNT}, square::Square};

/// Represents a piece, the ordering is important since in move generation the promotion piecetype is
/// encoded in 2 bits, this ordering allows us to cast it directly into this enum.
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Piece {
    Knight = 0,
    Bishop = 1,
    Rook = 2,
    Queen = 3,
    King = 4,
    Pawn = 5,
}

impl Arbitrary for Piece {
    fn arbitrary(g: &mut Gen) -> Self {
        PIECES[usize::arbitrary(g) % PIECE_COUNT]
    }
}

impl Piece {
    pub fn last_piece() -> Piece {
        Piece::Pawn
    }

    pub fn to_fen(&self, color: Color) -> char {
        ASCII_PIECE_CHARS[color as usize][*self as usize]
    }

    pub fn movements_at(&self, square: impl Into<Square>, color: Color) -> impl Iterator<Item=Square> {
        square.into().moves_for(self, &color)
    }
}

impl From<u16> for Piece {
    fn from(v: u16) -> Self {
        PIECES[(v & 0x0007) as usize]
    }
}

pub const PIECE_COUNT: usize = 6;

/// A convenience array for looping over the pieces in the right order.
pub const PIECES: [Piece; PIECE_COUNT] = [
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
    Piece::Pawn,
];

/// ASCII representations of each piece
pub const ASCII_PIECE_CHARS: [[char; PIECE_COUNT]; COLOR_COUNT] = [
    ['N', 'B', 'R', 'Q', 'K', 'P'],
    ['n', 'b', 'r', 'q', 'k', 'p'],
];

// FIXME: I couldn't get the formatting right w/ unicode, but I don't want to lose
// track of the unicode characters, so I'm keeping them here.
#[allow(unused_variables)]
pub const UNICODE_PIECE_CHARS: [[char; 6]; 2] = [
    [
        '\u{2658}', //'♘';
        '\u{2657}', //'♗';
        '\u{2656}', //'♖';
        '\u{2655}', //'♕';
        '\u{2659}', //'♙';
        '\u{2654}', //'♔';
    ],
    [
        '\u{265E}', //'♞';
        '\u{265D}', //'♝';
        '\u{265C}', //'♜';
        '\u{265B}', //'♛';
        '\u{265F}', //'♟︎';
        '\u{265A}', //'♚';
    ],
];


#[cfg(test)]
mod test {
    use crate::square::*;

    use super::*;

    #[test]
    fn last_piece() {
        assert_eq!(Piece::last_piece(), Piece::Pawn);
    }

    #[quickcheck]
    fn movements_at(color: bool) {
        let color = if color { Color::WHITE } else { Color::BLACK };
        similar_asserts::assert_eq!(Piece::Knight.movements_at(A1, color).collect::<Vec<_>>(), vec![B3, C2]);
        similar_asserts::assert_eq!(Piece::Bishop.movements_at(A1, color).collect::<Vec<_>>(), vec![B2, C3, D4, E5, F6, G7, H8]);
        similar_asserts::assert_eq!(Piece::Rook.movements_at(A1, color).collect::<Vec<_>>(), vec![A2, A3, A4, A5, A6, A7, A8, B1, C1, D1, E1, F1, G1, H1]);
        similar_asserts::assert_eq!(Piece::Queen.movements_at(A1,color).collect::<Vec<_>>(), vec![A2, A3, A4, A5, A6, A7, A8, B1, C1, D1, E1, F1, G1, H1, B2, C3, D4, E5, F6, G7, H8]);
        similar_asserts::assert_eq!(Piece::King.movements_at(A1, color).collect::<Vec<_>>(), vec![A2, B1, B2]);
        similar_asserts::assert_eq!(Piece::Pawn.movements_at(D2, Color::WHITE).collect::<Vec<_>>(), vec![D3, D4, E3, C3]);
        similar_asserts::assert_eq!(Piece::Pawn.movements_at(D7, Color::BLACK).collect::<Vec<_>>(), vec![D6, D5, E6, C6]);
        similar_asserts::assert_eq!(Piece::Pawn.movements_at(D4, Color::WHITE).collect::<Vec<_>>(), vec![D5, D6, E5, C5]);
        similar_asserts::assert_eq!(Piece::Pawn.movements_at(D5, Color::BLACK).collect::<Vec<_>>(), vec![D4, D3, E4, C4]);
    }

    mod from {
        use super::*;

        fn converts_correctly(val: u16, piece: Piece) {
            let p = Piece::from(val);
            assert_eq!(p, piece);
        }

        #[test]
        pub fn converts_knights_correctly() {
            converts_correctly(0, Piece::Knight);
        }
        #[test]
        pub fn converts_bishops_correctly() {
            converts_correctly(1, Piece::Bishop);
        }
        #[test]
        pub fn converts_rooks_correctly() {
            converts_correctly(2, Piece::Rook);
        }
        #[test]
        pub fn converts_queens_correctly() {
            converts_correctly(3, Piece::Queen);
        }
        #[test]
        pub fn converts_kings_correctly() {
            converts_correctly(4, Piece::King);
        }
        #[test]
        pub fn converts_pawns_correctly() {
            converts_correctly(5, Piece::Pawn);
        }
    }

    mod to_fen {
        use super::*;

        fn converts_correctly(piece: Piece, color: Color, expected: char) {
            let c = piece.to_fen(color);
            assert_eq!(c, expected);
        }

        #[test]
        pub fn converts_knights_correctly() {
            converts_correctly(Piece::Knight, Color::WHITE, 'N');
            converts_correctly(Piece::Knight, Color::BLACK, 'n');
        }
        #[test]
        pub fn converts_bishops_correctly() {
            converts_correctly(Piece::Bishop, Color::WHITE, 'B');
            converts_correctly(Piece::Bishop, Color::BLACK, 'b');
        }
        #[test]
        pub fn converts_rooks_correctly() {
            converts_correctly(Piece::Rook, Color::WHITE, 'R');
            converts_correctly(Piece::Rook, Color::BLACK, 'r');
        }
        #[test]
        pub fn converts_queens_correctly() {
            converts_correctly(Piece::Queen, Color::WHITE, 'Q');
            converts_correctly(Piece::Queen, Color::BLACK, 'q');
        }
        #[test]
        pub fn converts_kings_correctly() {
            converts_correctly(Piece::King, Color::WHITE, 'K');
            converts_correctly(Piece::King, Color::BLACK, 'k');
        }
        #[test]
        pub fn converts_pawns_correctly() {
            converts_correctly(Piece::Pawn, Color::WHITE, 'P');
            converts_correctly(Piece::Pawn, Color::BLACK, 'p');
        }
    }
}
