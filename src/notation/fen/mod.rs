mod position_metadata;
mod position;
mod castle_rights;

use std::fmt::{Debug, Display};

use tracing::instrument;

use crate::board::Alter;
use crate::board::Alteration;
use crate::constants::{EMPTY_POSITION_FEN, START_POSITION_FEN};
use crate::types::Color;
use crate::notation::*;


pub use position_metadata::PositionMetadata;
pub use castle_rights::CastleRights;
use position::Position;

#[derive(Clone)]
pub struct FEN {
    original_fen: String,
    position: Position,
    metadata: PositionMetadata
}

impl Debug for FEN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.position, self.metadata)
    }
}

impl PartialEq for FEN {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position &&
        self.metadata == other.metadata
    }
}
impl Eq for FEN {}

impl Default for FEN {
    fn default() -> Self {
        Self::new(EMPTY_POSITION_FEN)
    }
}

impl FEN {
    pub fn start_position() -> Self {
        Self::new(START_POSITION_FEN)
    }

    /// Sometimes you just want to specify the position without all the metadata, this
    /// assumes you are describing a position with white-to-move, all castling rights, no en
    /// passant square.
    #[instrument]
    pub fn with_default_metadata(fen: &str) -> Self {
        let fenprime = format!("{} {}", fen, PositionMetadata::default());
        let position = Position::new(fen);

        Self {
            original_fen: fenprime,
            position,
            metadata: PositionMetadata::default(),
        }
    }

    /// Expects a full FEN string with all metadata.
    #[instrument]
    pub fn new(fen: &str) -> Self {
        let mut metadata = PositionMetadata::default();

        let mut parts = fen.split_whitespace();
        let position_str = parts.next().unwrap();
        let position = Position::new(position_str);

        metadata.parse(&mut parts);

        Self {
            original_fen: fen.to_string(),
            position,
            metadata
        }
    }

    #[instrument]
    pub fn side_to_move(&self) -> Color {
        self.metadata.side_to_move
    }

    #[instrument]
    pub fn castling(&self) -> CastleRights {
        self.metadata.castling
    }

    #[instrument]
    pub fn en_passant(&self) -> Option<Square> {
        self.metadata.en_passant
    }

    #[instrument]
    pub fn halfmove_clock(&self) -> u8 {
        self.metadata.halfmove_clock
    }

    #[instrument]
    pub fn fullmove_number(&self) -> u16 {
        self.metadata.fullmove_number
    }

    #[instrument]
    pub fn setup<A>(&self) -> A where A : Alter + Default {
        let mut board = A::default();
        for alteration in self.position.clone().into_iter() {
            board.alter_mut(alteration);
        }
        board
    }

    pub fn compile(&self) -> Vec<Alteration> {
        self.position.clone().into_iter().collect()
    }

    pub fn metadata(&self) -> PositionMetadata {
        self.metadata.clone()
    }
}

impl Display for FEN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.position, self.metadata)
    }
}

pub fn setup<A : Alter + Default>(fen: &FEN) -> A {
    let mut board = A::default();
    setup_mut(fen, &mut board);
    board
}

pub fn setup_mut<A : Alter>(fen: &FEN, board: &mut A) {
    for alteration in fen.position.clone().into_iter() {
        board.alter_mut(alteration);
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
        assert!(fen.castling().white_short);
        assert!(fen.castling().white_long);
        assert!(fen.castling().black_short);
        assert!(fen.castling().black_long);
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
        assert!(fen.castling().white_short);
        assert!(fen.castling().white_long);
        assert!(fen.castling().black_short);
        assert!(fen.castling().black_long);
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
        assert!(fen.castling().white_short);
        assert!(fen.castling().white_long);
        assert!(fen.castling().black_short);
        assert!(fen.castling().black_long);
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

    #[test]
    fn fen_debug() {
        let problem = FEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d2 0 2");

        assert!(true);
    }

}

