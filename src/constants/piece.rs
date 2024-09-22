use serde::{Deserialize, Serialize};
use crate::constants::Color;

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

impl Piece {
    pub fn last_piece() -> Piece {
        Piece::Pawn
    }

    pub fn to_fen(&self, color: Color) -> char {
        ASCII_PIECE_CHARS[color as usize][*self as usize]
    }
}

impl From<u16> for Piece {
    fn from(v: u16) -> Self {
        PIECES[(v & 0x0007) as usize]
    }
}


/// A convenience array for looping over the pieces in the right order.
pub const PIECES: [Piece; 6] = [
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
    Piece::Pawn,
];

/// ASCII representations of each piece
pub const ASCII_PIECE_CHARS: [[char; 6]; 2] = [
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
    use super::*;

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
