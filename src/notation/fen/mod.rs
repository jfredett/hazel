mod position_metadata;
mod position;
mod castle_rights;

use std::fmt::{Debug, Display};

use tracing::instrument;

use crate::board::Alter;
use crate::board::Alteration;
use crate::board::PieceBoard;
use crate::board::Query;
use crate::constants::{EMPTY_POSITION_FEN, START_POSITION_FEN};
use crate::types::Color;
use crate::notation::*;
use crate::types::Occupant;


pub use position_metadata::PositionMetadata;
pub use castle_rights::CastleRights;
use position::Position;

#[derive(Clone)]
pub struct FEN {
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


impl Query for FEN {
    fn get(&self, s: impl Into<Square>) -> Occupant {
        // TODO: This can be done directly from the string representation of the FEN, but this is
        // two lines of mindless code and I am lazy.
        let pb = PieceBoard::from(self);
        pb.get(s)
    }
}

// This is a little cursed, but it is handy to be able to alter any board representation, and this
// is a board representation. Maybe it should live under coup::rep? :D
impl Alter for FEN {
    fn alter(&self, alteration: Alteration) -> Self {
        let mut new = self.clone();
        new.alter_mut(alteration);
        new
    }

    // HACK: This doesn't do metadata, it probably should.
    fn alter_mut(&mut self, alteration: Alteration) -> &mut Self {
        let mut pb = PieceBoard::from(self.clone());
        pb.alter_mut(alteration);
        self.position = pb.into();
        self
    }
}

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
        Self {
            position: Position::new(fen),
            metadata: PositionMetadata::default(),
        }
    }

    pub fn set_metadata(&mut self, metadata: PositionMetadata) {
        self.metadata = metadata;
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
    use crate::types::Occupant;


    use super::*;

    mod alter {
        use super::*;

        #[test]
        fn alter_is_correct_for_empty_pos() {
            let fen = FEN::default();
            let altered = fen.alter(Alteration::Place { square: A1, occupant: Occupant::white_rook() });
            assert_eq!(altered, FEN::new("8/8/8/8/8/8/8/R7 w KQkq - 0 1"));
        }

        #[test]
        fn remove_works() {
            let fen = FEN::new("8/8/8/8/8/8/8/R7 w KQkq - 0 1");
            let altered = fen.alter(Alteration::Remove { square: A1, occupant: Occupant::white_rook() });
            assert_eq!(altered, FEN::default());
        }


    }



    #[test]
    fn compile_is_correct_for_empty_pos() {
        let fen = FEN::new(EMPTY_POSITION_FEN);
        let alterations = fen.compile();
        assert_eq!(alterations.len(), 0);
    }

    #[test]
    fn compile_is_correct_for_start_pos() {
        let fen = FEN::new(START_POSITION_FEN);
        let alterations = fen.compile();
        assert_eq!(alterations.len(), 32);
        assert_eq!(alterations, vec![
            Alteration::Place { square: A8, occupant: Occupant::black_rook() },
            Alteration::Place { square: B8, occupant: Occupant::black_knight() },
            Alteration::Place { square: C8, occupant: Occupant::black_bishop() },
            Alteration::Place { square: D8, occupant: Occupant::black_queen() },
            Alteration::Place { square: E8, occupant: Occupant::black_king() },
            Alteration::Place { square: F8, occupant: Occupant::black_bishop() },
            Alteration::Place { square: G8, occupant: Occupant::black_knight() },
            Alteration::Place { square: H8, occupant: Occupant::black_rook() },
            Alteration::Place { square: A7, occupant: Occupant::black_pawn() },
            Alteration::Place { square: B7, occupant: Occupant::black_pawn() },
            Alteration::Place { square: C7, occupant: Occupant::black_pawn() },
            Alteration::Place { square: D7, occupant: Occupant::black_pawn() },
            Alteration::Place { square: E7, occupant: Occupant::black_pawn() },
            Alteration::Place { square: F7, occupant: Occupant::black_pawn() },
            Alteration::Place { square: G7, occupant: Occupant::black_pawn() },
            Alteration::Place { square: H7, occupant: Occupant::black_pawn() },
            Alteration::Place { square: A2, occupant: Occupant::white_pawn() },
            Alteration::Place { square: B2, occupant: Occupant::white_pawn() },
            Alteration::Place { square: C2, occupant: Occupant::white_pawn() },
            Alteration::Place { square: D2, occupant: Occupant::white_pawn() },
            Alteration::Place { square: E2, occupant: Occupant::white_pawn() },
            Alteration::Place { square: F2, occupant: Occupant::white_pawn() },
            Alteration::Place { square: G2, occupant: Occupant::white_pawn() },
            Alteration::Place { square: H2, occupant: Occupant::white_pawn() },
            Alteration::Place { square: A1, occupant: Occupant::white_rook() },
            Alteration::Place { square: B1, occupant: Occupant::white_knight() },
            Alteration::Place { square: C1, occupant: Occupant::white_bishop() },
            Alteration::Place { square: D1, occupant: Occupant::white_queen() },
            Alteration::Place { square: E1, occupant: Occupant::white_king() },
            Alteration::Place { square: F1, occupant: Occupant::white_bishop() },
            Alteration::Place { square: G1, occupant: Occupant::white_knight() },
            Alteration::Place { square: H1, occupant: Occupant::white_rook() },
        ]);
    }

    #[test]
    fn fen_startpos() {
        let fen = FEN::new(START_POSITION_FEN);
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
    fn fen_parses_ep_square() {
        let problem = FEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d2 0 2");
        // This round-trips the string into our structures and back out.
        assert_eq!(format!("{:?}", problem), "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d2 0 2");
    }

}

