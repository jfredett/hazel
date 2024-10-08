#[allow(unused_imports)]
use tracing::debug;

use crate::constants::Piece;
use crate::driver::Engine;
use crate::movement::Move;
use crate::uci::UCIMessage;
use crate::board::occupant::Occupant;
use crate::movegen::alteration::Alteration;
use crate::board::interface::Board;
use tracing::instrument;

use std::fmt::Debug;

use super::{Alter, Query};

#[derive(Default, Clone, Copy)]
pub struct PieceBoard {
    pub board: [[Occupant; 8]; 8],
}

impl Debug for PieceBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for row in self.board.iter().rev() {
            for occupant in row.iter() {
write!(f, "{}", occupant)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub const START_POSITION : PieceBoard = PieceBoard {
    board: [
        [ Occupant::white_rook() , Occupant::white_knight() , Occupant::white_bishop() , Occupant::white_queen() , Occupant::white_king() , Occupant::white_bishop() , Occupant::white_knight() , Occupant::white_rook() ]       ,
        [ Occupant::white_pawn() , Occupant::white_pawn()   , Occupant::white_pawn()   , Occupant::white_pawn()  , Occupant::white_pawn() , Occupant::white_pawn()   , Occupant::white_pawn()   , Occupant::white_pawn() ]       ,
        [ Occupant::empty()      , Occupant::empty()        , Occupant::empty()        , Occupant::empty()       , Occupant::empty()      , Occupant::empty()        , Occupant::empty()        , Occupant::empty()            ] ,
        [ Occupant::empty()      , Occupant::empty()        , Occupant::empty()        , Occupant::empty()       , Occupant::empty()      , Occupant::empty()        , Occupant::empty()        , Occupant::empty()            ] ,
        [ Occupant::empty()      , Occupant::empty()        , Occupant::empty()        , Occupant::empty()       , Occupant::empty()      , Occupant::empty()        , Occupant::empty()        , Occupant::empty()            ] ,
        [ Occupant::empty()      , Occupant::empty()        , Occupant::empty()        , Occupant::empty()       , Occupant::empty()      , Occupant::empty()        , Occupant::empty()        , Occupant::empty()            ] ,
        [ Occupant::black_pawn() , Occupant::black_pawn()   , Occupant::black_pawn()   , Occupant::black_pawn()  , Occupant::black_pawn() , Occupant::black_pawn()   , Occupant::black_pawn()   , Occupant::black_pawn() ]       ,
        [ Occupant::black_rook() , Occupant::black_knight() , Occupant::black_bishop() , Occupant::black_queen() , Occupant::black_king() , Occupant::black_bishop() , Occupant::black_knight() , Occupant::black_rook() ]       ,
    ]
};


// FIXME: I think this renders the board upside down rn? indices are being read a 0oRF, with 0o0F
// being the 'top' row as rendered, and 0o_7 being the 'rightmost' file
impl PieceBoard {
    pub fn new() -> Self {
        Self {
            board: [[Occupant::empty(); 8]; 8]
        }
    }

    pub fn set_board(&mut self, board: [[Occupant; 8]; 8]) {
        self.board = board;
    }

    pub fn set_startpos(&mut self) {
        self.set_board(START_POSITION.board);
    }

    pub fn set_fen(&mut self, fen: &str) {
        self.set_board(Self::from_fen(fen).board);
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        for row in self.board.iter().rev() {
            let mut empty = 0;
            for occupant in row.iter() {
                match occupant {
                    Occupant::Occupied(piece, color) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        fen.push(piece.to_fen(*color));
                    },
                    Occupant::Empty => {
                        empty += 1;
                    }
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
            }
            fen.push('/');
        }
        fen.pop(); // remove the last '/'
        fen
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = [[Occupant::empty(); 8]; 8];
        let mut row = 7;
        let mut col = 0;
        dbg!(fen);
        for c in fen.chars() {
            match c {
                'r' => { board[row][col] = Occupant::black(Piece::Rook); col += 1; },
                'n' => { board[row][col] = Occupant::black(Piece::Knight); col += 1; },
                'b' => { board[row][col] = Occupant::black(Piece::Bishop); col += 1; },
                'q' => { board[row][col] = Occupant::black(Piece::Queen); col += 1; },
                'k' => { board[row][col] = Occupant::black(Piece::King); col += 1; },
                'p' => { board[row][col] = Occupant::black(Piece::Pawn); col += 1; },

                'R' => { board[row][col] = Occupant::white(Piece::Rook); col += 1; },
                'N' => { board[row][col] = Occupant::white(Piece::Knight); col += 1; },
                'B' => { board[row][col] = Occupant::white(Piece::Bishop); col += 1; },
                'Q' => { board[row][col] = Occupant::white(Piece::Queen); col += 1; },
                'K' => { board[row][col] = Occupant::white(Piece::King); col += 1; },
                'P' => { board[row][col] = Occupant::white(Piece::Pawn); col += 1; },

                '1' => { col += 1; },
                '2' => { col += 2; },
                '3' => { col += 3; },
                '4' => { col += 4; },
                '5' => { col += 5; },
                '6' => { col += 6; },
                '7' => { col += 7; },
                '8' => { col += 8; },

                '/' => { row -= 1; col = 0; },
                _ => { debug!("Unsupported FEN character: '{}'", c); break; }
            }
        }

        Self { board }
    }

    pub fn set(&mut self, index: usize, occupant: Occupant) {
        let row = index / 8;
        let col = index % 8;
        self.board[row][col] = occupant;
    }
}

impl Query for PieceBoard {
    fn get(&self, index: usize) -> Occupant {
        let row = index / 8;
        let col = index % 8;
        self.board[row][col]
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
            Alteration::Place { index, occupant } => {
                self.set(index, occupant);
            },
            #[allow(unused_variables)] // _occupant is does not work, syntax error.
            Alteration::Remove { index, occupant } => {
                self.set(index, Occupant::empty());
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

    fn exec(&mut self, message: UCIMessage) -> Vec<UCIMessage> {
        match message {
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
                    self.set_fen(&fen);
                }

                for m in moves {
                    self.make_mut(Move::from_uci(&m));
                }

                vec![]
            },
            _ => vec![]
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod get_set {
        use super::*;

        #[test]
        pub fn gets_piece_correctly() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
            let board = PieceBoard::from_fen(fen);
            assert_eq!(board.get(0o00), Occupant::white(Piece::Rook));
            assert_eq!(board.get(0o77), Occupant::black(Piece::Rook));
            assert_eq!(board.get(0o33), Occupant::empty());
        }

        #[test]
        pub fn sets_piece_correctly() {
            let mut board = PieceBoard::new();
            board.set(0o00, Occupant::white(Piece::Rook));
            board.set(0o77, Occupant::black(Piece::Rook));
            assert_eq!(board.get(0o00), Occupant::white(Piece::Rook));
            assert_eq!(board.get(0o77), Occupant::black(Piece::Rook));
        }

        #[test]
        pub fn bottom_left_is_0o00() {
            let mut board = PieceBoard::new();
            board.set(0o00, Occupant::white(Piece::Rook));
            let rep = format!("{:?}", board);
            let expected_rep = "\n........\n........\n........\n........\n........\n........\n........\nR.......\n";

            // The board should find the rook
            assert_eq!(board.get(0o00), Occupant::white(Piece::Rook));
            // it should be in the bottom left of the representation
            assert_eq!(rep, expected_rep);
        }
    }

    mod fen {
        use super::*;

        #[test]
        pub fn converts_start_position_correctly() {
            let mut board = PieceBoard::new();
            board.set_startpos();
            dbg!(&board);
            let fen = board.to_fen();
            assert_eq!(fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        }

        #[test]
        pub fn converts_empty_board_correctly() {
            let board = PieceBoard::new();
            let fen = board.to_fen();
            assert_eq!(fen, "8/8/8/8/8/8/8/8");
        }

        #[test]
        pub fn converts_fen_to_board_correctly() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
            let board = PieceBoard::from_fen(fen);
            let fen2 = board.to_fen();
            assert_eq!(fen, fen2);
        }

        #[test]
        pub fn converts_each_offset_correctly() {
            let fen = "p7/1p6/2p5/3p4/4p3/5p2/6p1/7p";
            let board = PieceBoard::from_fen(fen);
            let fen2 = board.to_fen();
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
            let mut board = PieceBoard::new();
            assert_eq!(board.get(0o34), Occupant::empty());

            board.alter_mut(Alteration::place(0o34, Occupant::white_pawn()));
            assert_eq!(board.get(0o34), Occupant::white_pawn());

            // piece choice is irrelevant
            board.alter_mut(Alteration::remove(0o34, Occupant::black_pawn()));
            assert_eq!(board.get(0o34), Occupant::empty());
        }

        #[test]
        pub fn piece_removed_does_not_matter() {
            // This is only used as metadata for unmoving later. It's generally not used in the
            // forward alteration.
            let mut board = PieceBoard::new();
            board.set_startpos();

            // Note that it's a pawn being 'removed'.
            board.alter_mut(Alteration::remove(0o40, Occupant::white_pawn()));
            assert_eq!(board.get(0o40), Occupant::empty());
        }
    }

    mod engine {
        use tracing_test::traced_test;

        use super::*;

        #[traced_test]
        #[test]
        pub fn executes_position_correctly() {
            let mut board = PieceBoard::new();
            let moves = vec!["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "g8f6", "d2d3", "d7d6", "c1e3", "c8e6"];
            let message = UCIMessage::Position("startpos".to_string(), moves.iter().map(|s| s.to_string()).collect());
            dbg!(&message);
            dbg!(board);
            board.exec(message);
            dbg!(board);
            let fen = board.to_fen();
            assert_eq!(fen, "r2qkb1r/ppp2ppp/2npbn2/4p3/2B1P3/3PBN2/PPP2PPP/RN1QK2R");
        }
    }
}
