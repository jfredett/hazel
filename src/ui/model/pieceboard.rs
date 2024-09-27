#[allow(unused_imports)]
use tracing::debug;

use crate::constants::{Piece, NOTATION_TO_COORDS};
use crate::driver::Engine;
use crate::uci::UCIMessage;
use crate::ui::model::{occupant::Occupant, alteration::Alteration};
use crate::constants::color::Color;

use std::fmt::Debug;

#[derive(Default, Clone, Copy)]
pub struct PieceBoard {
    pub board: [[Occupant; 8]; 8],
}

impl Debug for PieceBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for row in self.board.iter() {
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
        [ Occupant::white(Piece::Rook) , Occupant::white(Piece::Knight) , Occupant::white(Piece::Bishop) , Occupant::white(Piece::Queen) , Occupant::white(Piece::King) , Occupant::white(Piece::Bishop) , Occupant::white(Piece::Knight) , Occupant::white(Piece::Rook) ] ,
        [ Occupant::white(Piece::Pawn) , Occupant::white(Piece::Pawn)   , Occupant::white(Piece::Pawn)   , Occupant::white(Piece::Pawn)  , Occupant::white(Piece::Pawn) , Occupant::white(Piece::Pawn)   , Occupant::white(Piece::Pawn)   , Occupant::white(Piece::Pawn) ] ,
        [ Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()             , Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()            ] ,
        [ Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()             , Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()            ] ,
        [ Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()             , Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()            ] ,
        [ Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()             , Occupant::empty()            , Occupant::empty()              , Occupant::empty()              , Occupant::empty()            ] ,
        [ Occupant::black(Piece::Pawn) , Occupant::black(Piece::Pawn)   , Occupant::black(Piece::Pawn)   , Occupant::black(Piece::Pawn)  , Occupant::black(Piece::Pawn) , Occupant::black(Piece::Pawn)   , Occupant::black(Piece::Pawn)   , Occupant::black(Piece::Pawn) ] ,
        [ Occupant::black(Piece::Rook) , Occupant::black(Piece::Knight) , Occupant::black(Piece::Bishop) , Occupant::black(Piece::Queen) , Occupant::black(Piece::King) , Occupant::black(Piece::Bishop) , Occupant::black(Piece::Knight) , Occupant::black(Piece::Rook) ] ,
    ]
};

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

    pub fn get(&self, row: usize, col: usize) -> Occupant {
        self.board[row][col]
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
                _ => { panic!("Invalid FEN character: {}", c); }
            }
        }

        Self { board }
    }

    pub fn alter(&mut self, moves: Vec<Alteration>) {
        for m in moves {
            match m {
                Alteration::Move(from, to) => {
                    self.board[to.0][to.1] = self.board[from.0][from.1];
                    self.board[from.0][from.1] = Occupant::empty();
                },
                Alteration::Remove(pos) => {
                    self.board[pos.0][pos.1] = Occupant::empty();
                },
                Alteration::Place(pos, occupant) => {
                    self.board[pos.0][pos.1] = occupant;
                }
            }
        }
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
                    self.set_board(PieceBoard::from_fen(&fen).board);
                }
                let mut alterations = vec![];
                for m in moves {
                    let source = NOTATION_TO_COORDS(&m[0..2]);
                    let dest = NOTATION_TO_COORDS(&m[2..4]);
                    let source_piece = self.get(source.0, source.1);
                    let current_color = source_piece.color().unwrap();
                    let dest_piece = self.get(dest.0, dest.1);
                    let is_promotion = source_piece.piece().unwrap() == Piece::Pawn 
                                    && ((dest.0 == 0 && current_color == Color::BLACK)
                                    ||  (dest.0 == 7 && current_color == Color::WHITE));

                    if is_promotion {
                        let promotion = match &m[5..5] {
                            "q" => Piece::Queen,
                            "r" => Piece::Rook,
                            "b" => Piece::Bishop,
                            "n" => Piece::Knight,
                            _ => panic!("Invalid promotion piece: {}", &m)
                        };
                        alterations.push(Alteration::Remove(source));
                        alterations.push(Alteration::Remove(dest));
                        alterations.push(Alteration::Place(dest, Occupant::Occupied(promotion, current_color)));
                    } else if dest_piece != Occupant::Empty {
                        alterations.push(Alteration::Remove(source));
                        alterations.push(Alteration::Move(source, dest));
                    } else {
                        alterations.push(Alteration::Move(source, dest));
                    }
                }
                self.alter(alterations);

                vec![]
            },
            _ => vec![]
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    mod get {
        use super::*;

        #[test]
        pub fn gets_piece_correctly() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
            let board = PieceBoard::from_fen(fen);
            assert_eq!(board.get(0, 0), Occupant::white(Piece::Rook));
            assert_eq!(board.get(7, 7), Occupant::black(Piece::Rook));
            assert_eq!(board.get(3, 3), Occupant::empty());
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
            let moves = vec![
                Alteration::Place((4, 4), Occupant::black(Piece::Pawn)),
                Alteration::Move((4, 4), (4, 5))
            ];
            board.alter(moves);
            assert_eq!(board.get(4, 4), Occupant::empty());
            assert_eq!(board.get(4, 5), Occupant::black(Piece::Pawn));

            board.alter(vec![Alteration::Remove((4, 5))]);
            assert_eq!(board.get(4, 5), Occupant::empty());
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
