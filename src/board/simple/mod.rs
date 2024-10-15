use std::fmt::Debug;
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


#[derive(Clone, Copy, PartialEq)]
pub struct PieceBoard {
    pub board: [Occupant; 64],
}

impl Default for PieceBoard {
    fn default() -> Self {
        Self { board: [Occupant::empty(); 64] }
    }
}

impl Debug for PieceBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        write!(f, "{}", display_board(self))
    }
}

impl PieceBoard {
    pub fn set_startpos(&mut self) {
        self.set_fen(&FEN::new(START_POSITION_FEN))
    }

    pub fn set_fen(&mut self, fen: &FEN) {
        fen::setup_mut(fen, self);
    }

    pub fn set<S>(&mut self, square: S, occupant: Occupant) where S : SquareNotation {
        let sq = square.into();

        self.board[sq.index()] = occupant;
    }
}

impl From<FEN> for PieceBoard {
    fn from(fen: FEN) -> Self {
        fen::setup(&fen)
    }
}

impl From<&FEN> for PieceBoard {
    fn from(fen: &FEN) -> Self {
        fen::setup(fen)
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
        self.exec(UCIMessage::parse(message))
    }

    #[instrument]
    fn exec(&mut self, message: UCIMessage) -> Vec<UCIMessage> {
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
                    let uci = UCI::try_from(&m).expect(format!("Invalid Move: {}", &m).as_str());
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

        #[test]
        pub fn bottom_left_is_A1() {
            let mut board = PieceBoard::default();
            board.set(A1, Occupant::white_rook());
            let rep = format!("{:?}", board);
            let expected_rep = "
8 . . . . . . . .
7 . . . . . . . .
6 . . . . . . . .
5 . . . . . . . .
4 . . . . . . . .
3 . . . . . . . .
2 . . . . . . . .
1 R . . . . . . .
  a b c d e f g h
";
            println!("{}", rep);
            println!("{}", expected_rep);

            // The board should find the rook
            assert_eq!(board.get(A1), Occupant::white(Piece::Rook));
            // it should be in the bottom left of the representation
            assert_eq!(rep, expected_rep);
        }
    }

    mod fen {
        use super::*;
        use crate::board::interface::query;

        #[test]
        pub fn converts_start_position_correctly() {
            let mut board = PieceBoard::default();
            board.set_startpos();
            let fen = query::to_fen(&board);
            assert_eq!(fen, FEN::new(START_POSITION_FEN));
        }

        #[test]
        pub fn converts_empty_board_correctly() {
            let board = PieceBoard::default();
            let fen = query::to_fen(&board);
            assert_eq!(fen, FEN::new(EMPTY_POSITION_FEN));
        }

        #[test]
        pub fn converts_fen_to_board_correctly() {
            let fen = FEN::new(START_POSITION_FEN);
            let mut board = PieceBoard::default();
            board.set_fen(&fen);
            let fen2 = query::to_fen(&board);
            assert_eq!(fen, fen2);
        }

        #[test]
        pub fn converts_each_offset_correctly() {
            let fen = FEN::new("p7/1p6/2p5/3p4/4p3/5p2/6p1/7p w KQkq - 0 1");
            let mut board = PieceBoard::default();
            board.set_fen(&fen);
            let fen2 = query::to_fen(&board);
            assert_eq!(fen, fen2);
        }

        /* For want of a FEN type and an Arbitrary instance 
        #[quickcheck]
        pub fn converts_fen_to_board_correctly_quickcheck(fen: FEN) -> bool {
            let board = PieceBoard::from_fen(&fen);
            let fen2 = board.to_fen();
            fen == fen2
        }
        */
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
            board.exec(message);
            let fen = query::to_fen(&board);
            assert_eq!(fen, FEN::new("r2qkb1r/ppp2ppp/2npbn2/4p3/2B1P3/3PBN2/PPP2PPP/RN1QK2R"));
        }
    }
}
