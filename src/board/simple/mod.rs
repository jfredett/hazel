use tracing::{instrument, debug};

use crate::board::{alter::Alter, alteration::Alteration, query::{display_board , Query}};
use crate::constants::{START_POSITION_FEN, EMPTY_POSITION_FEN};
use crate::engine::Engine;
use crate::coup::rep::Move;
use crate::notation::*;
use crate::notation::uci::UCI;
use crate::notation::fen::{self, setup_mut, FEN};
use crate::types::{Piece, Occupant};
use crate::engine::uci::UCIMessage;
use crate::game::interface::Chess;



pub mod display_debug;
pub mod from_into;

pub use display_debug::*;
pub use from_into::*;

#[derive(Clone, Copy, PartialEq)]
pub struct PieceBoard {
    pub board: [Occupant; 64],
}

impl Default for PieceBoard {
    fn default() -> Self {
        Self { board: [Occupant::empty(); 64] }
    }
}


impl PieceBoard {
    pub fn set<S>(&mut self, square: S, occupant: Occupant) where S : SquareNotation {
        let sq = square.into();

        self.board[sq.index()] = occupant;
    }
}

impl Query for PieceBoard {
    fn get<S>(&self, square: S) -> Occupant where S: SquareNotation {
        let sq = square.into();
        self.board[sq.index()]
    }
}

impl Alter for PieceBoard {
    #[instrument]
    fn alter(&self, alter: Alteration) -> PieceBoard {
        let mut board = self.clone();
        board.alter_mut(alter);
        board
    }

    #[instrument]
    fn alter_mut(&mut self, alter: Alteration) -> &mut Self {
        match alter {
            Alteration::Place { square, occupant } => {
                self.set(square, occupant);
            },
            #[allow(unused_variables)] // _occupant is does not work, syntax error.
            Alteration::Remove { square, occupant } => {
                self.set(square, Occupant::empty());
            },
            _ => {}
        }
        self
    }
}

impl Engine<UCIMessage> for PieceBoard {
    fn exec_message(&mut self, message: &str) -> Vec<UCIMessage> {
        self.exec(&UCIMessage::parse(message))
    }

    #[instrument]
    fn exec(&mut self, message: &UCIMessage) -> Vec<UCIMessage> {
        let ret = match message {
            UCIMessage::UCI => vec![UCIMessage::ID("name".to_string(), "Hazel Pieceboard".to_string()), UCIMessage::UCIOk],
            UCIMessage::IsReady => vec![UCIMessage::ReadyOk],
            UCIMessage::UCINewGame => {
                self.set_startpos();
                vec![]
            },
            UCIMessage::Position(fen, moves) => {
                if fen == "startpos" {
                    self.set_startpos();
                } else {
                    self.set_fen(&FEN::new(&fen));
                }

                debug!("Here");

                for m in moves {
                    let uci = UCI::try_from(m).expect(format!("Invalid Move: {}", &m).as_str());
                    self.make_mut(uci.into());
                }

                vec![]
            },
            _ => vec![]
        };
        debug!("Done With Exec");
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod get_set {
        use super::*;
        use crate::notation::Square;

        #[test]
        pub fn gets_piece_correctly() {
            let mut board = PieceBoard::default();
            board.set_fen(&FEN::new(START_POSITION_FEN));
            assert_eq!(board.get(A1), Occupant::white_rook());
            assert_eq!(board.get(H8), Occupant::black_rook());
            assert_eq!(board.get(D4), Occupant::empty());
        }

        #[test]
        pub fn sets_piece_correctly() {
            let mut board = PieceBoard::default();
            board.set(A1, Occupant::white_rook());
            board.set(H8, Occupant::black_rook());
            assert_eq!(board.get(A1), Occupant::white_rook());
            assert_eq!(board.get(H8), Occupant::black_rook());
        }

    }


    mod alter {
        use super::*;

        #[test]
        pub fn alters_board_correctly() {
            let mut board = PieceBoard::default();
            assert_eq!(board.get(D5), Occupant::empty());

            board.alter_mut(Alteration::place(D5, Occupant::white_pawn()));
            assert_eq!(board.get(D5), Occupant::white_pawn());

            // piece choice is irrelevant
            board.alter_mut(Alteration::remove(D5, Occupant::black_pawn()));
            assert_eq!(board.get(D5), Occupant::empty());
        }

        #[test]
        pub fn piece_removed_does_not_matter() {
            // This is only used as metadata for unmoving later. It's generally not used in the
            // forward alteration.
            let mut board = PieceBoard::default();
            board.set_startpos();

            // Note that it's a pawn being 'removed'.
            board.alter_mut(Alteration::remove(E1, Occupant::white_pawn()));
            assert_eq!(board.get(E1), Occupant::empty());
        }
    }

    mod engine {
        use tracing_test::traced_test;
        use crate::board::interface::query;

        use super::*;

        #[traced_test]
        #[test]
        pub fn executes_position_correctly() {
            let mut board = PieceBoard::default();
            let moves = vec!["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "g8f6", "d2d3", "d7d6", "c1e3", "c8e6"];
            let message = UCIMessage::Position("startpos".to_string(), moves.iter().map(|s| s.to_string()).collect());
            board.exec(&message);
            let fen = query::to_fen(&board);
            assert_eq!(fen, FEN::with_default_metadata("r2qkb1r/ppp2ppp/2npbn2/4p3/2B1P3/3PBN2/PPP2PPP/RN1QK2R"));
        }
    }
}
