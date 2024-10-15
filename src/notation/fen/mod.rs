mod position_metadata;
mod castle_rights;

use std::fmt::Display;
use std::str::SplitWhitespace;

use crate::board::{Alter, Alteration};
use crate::constants::EMPTY_POSITION_FEN;
use crate::types::{Color, Occupant, Piece};
use crate::game::interface::Chess;
use crate::notation::*;


use position_metadata::PositionMetadata;
use castle_rights::CastleRights;

// NOTE: There exists a metadata type in `Ply` which might be useful here. I intended it to be
// packed into 4 bytes, but I think I implemented it as a plain struct, either way, it can come in
// here.
// I need to start reorganizing things more aggressively, and pruning out the stuff I won't need
// anymore. It's messy in here.

#[derive(Debug, Clone)]
pub struct FEN {
    original_fen: String,
    position: Vec<Alteration>,
    metadata: PositionMetadata
}


impl PartialEq for FEN {
    fn eq(&self, other: &Self) -> bool {
        self.original_fen == other.original_fen
    }
}
impl Eq for FEN {}


impl Default for FEN {
    fn default() -> Self {
        Self::new(EMPTY_POSITION_FEN)
    }
}

impl FEN {
    /// Sometimes you just want to specify the position without all the metadata, this
    /// assumes you are describing a position with white-to-move, all castling rights, no en
    /// passant square.
    pub fn with_default_metadata(fen: &str) -> Self {
        let fenprime = format!("{} {}", fen, PositionMetadata::default());
        let position = Self::compile(&fenprime);

        Self {
            original_fen: fenprime,
            position,
            metadata: PositionMetadata::default(),
        }
    }

    /// Expects a full FEN string with all metadata.
    pub fn new(fen: &str) -> Self {
        let mut metadata = PositionMetadata::default();

        let mut parts = fen.split_whitespace();
        let position_str = parts.next().unwrap();
        let position = Self::compile(position_str);

        metadata.parse(&mut parts);

        Self {
            original_fen: fen.to_string(),
            position,
            metadata
        }
    }

    pub fn side_to_move(&self) -> Color {
        self.metadata.side_to_move
    }

    pub fn castling(&self) -> CastleRights {
        self.metadata.castling
    }

    pub fn en_passant(&self) -> Option<Square> {
        self.metadata.en_passant
    }

    pub fn halfmove_clock(&self) -> usize {
        self.metadata.halfmove_clock
    }

    pub fn fullmove_number(&self) -> usize {
        self.metadata.fullmove_number
    }

    pub fn setup<C>(&self) -> C where C : Chess {
        let mut board = C::default();
        for alteration in &self.position {
            board.alter_mut(*alteration);
        }
        board
    }

    fn compile(fen: &str) -> Vec<Alteration> {
        let mut alterations = Vec::new();
        let mut rank = 7;
        let mut file = 0;
        for c in fen.chars() {
            let square = Square::from((file, rank));
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    file += c.to_digit(10).unwrap() as usize;
                }
                _ => {
                    let color = if c.is_uppercase() { Color::WHITE } else { Color::BLACK };
                    let piece = match c.to_ascii_lowercase() {
                        'p' => Piece::Pawn,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'r' => Piece::Rook,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => {
                            // FIXME: This is ugly.
                            alterations.push(Alteration::Place{ square, occupant: Occupant::Empty } );
                            file += 1;
                            continue;
                        },
                    };
                    let occupant = Occupant::Occupied(piece, color);
                    alterations.push(Alteration::Place { square,  occupant } );
                    file += 1;
                }
            }
        }
        alterations
    }
}

impl Display for FEN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            self.original_fen,
        )
    }
}

pub fn setup<A : Alter + Default>(fen: &FEN) -> A {
    let mut board = A::default();
    setup_mut(fen, &mut board);
    board
}

pub fn setup_mut<A : Alter>(fen: &FEN, board: &mut A) {
    for alteration in &fen.position {
        board.alter_mut(*alteration);
    }
}

#[cfg(test)]
mod tests {
    use crate::board::simple::PieceBoard;
    use crate::constants::{POS2_KIWIPETE_FEN, START_POSITION_FEN};
    use crate::types::Color;


    use super::*;

    #[test]
    fn fen_startpos() {
        let fen = FEN::new(START_POSITION_FEN);
        assert_eq!(fen.original_fen, START_POSITION_FEN);
        // We test the position part below in the #setup test
        assert_eq!(fen.side_to_move(), Color::WHITE);
        assert_eq!(fen.castling().white_short, true);
        assert_eq!(fen.castling().white_long, true);
        assert_eq!(fen.castling().black_short, true);
        assert_eq!(fen.castling().black_long, true);
        assert_eq!(fen.en_passant(), None);
        assert_eq!(fen.halfmove_clock(), 0);
        assert_eq!(fen.fullmove_number(), 1);
    }

    #[test]
    fn fen_kiwipete_position() {
        let fen = FEN::new(POS2_KIWIPETE_FEN);
        assert_eq!(fen.original_fen, POS2_KIWIPETE_FEN);
        // We test the position part below in the #setup test
        assert_eq!(fen.side_to_move(), Color::WHITE);
        assert_eq!(fen.castling().white_short, true);
        assert_eq!(fen.castling().white_long, true);
        assert_eq!(fen.castling().black_short, true);
        assert_eq!(fen.castling().black_long, true);
        assert_eq!(fen.en_passant(), None);
        assert_eq!(fen.halfmove_clock(), 0);
        assert_eq!(fen.fullmove_number(), 1);
    }

    #[test]
    fn fen_startpos_setup() {
        // FIXME:  This might be testing the same codepath now?
        let fen = FEN::new(START_POSITION_FEN);
        // This is the new implementation
        let board = fen.setup::<PieceBoard>();

        // this is the old. It can be deprecated once this is done, then this test will need to
        // change, probably.
        let mut expected = PieceBoard::default();
        expected.set_fen(&FEN::new(START_POSITION_FEN));

        assert_eq!(board, expected);
    }

    #[test]
    fn fen_kiwipete_setup() {
        let fen = FEN::new(POS2_KIWIPETE_FEN);
        // This is the new implementation
        let board = fen.setup::<PieceBoard>();

        // this is the old. It can be deprecated once this is done, then this test will need to
        // change, probably.
        let mut expected = PieceBoard::default();
        expected.set_fen(&FEN::new(POS2_KIWIPETE_FEN));
        assert_eq!(board, expected);
    }

    #[test]
    fn fen_empty_board_setup() {
        let fen = FEN::new("8/8/8/8/8/8/8/8 w KQkq - 0 1");
        let board = fen.setup::<PieceBoard>();
        let expected = PieceBoard::default();
        assert_eq!(board, expected);
    }

    #[test]
    fn fen_empty_board() {
        let fen = FEN::new("8/8/8/8/8/8/8/8 w KQkq - 0 1");
        assert_eq!(fen.original_fen, "8/8/8/8/8/8/8/8 w KQkq - 0 1");
        // We test the position part below in the #setup test
        assert_eq!(fen.side_to_move(), Color::WHITE);
        assert_eq!(fen.castling().white_short, true);
        assert_eq!(fen.castling().white_long, true);
        assert_eq!(fen.castling().black_short, true);
        assert_eq!(fen.castling().black_long, true);
        assert_eq!(fen.en_passant(), None);
        assert_eq!(fen.halfmove_clock(), 0);
        assert_eq!(fen.fullmove_number(), 1);
    }
    
    #[test]
    fn fen_display() {
        let fen = FEN::new(START_POSITION_FEN);
        let expected = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(format!("{}", fen), expected);
    }

}

