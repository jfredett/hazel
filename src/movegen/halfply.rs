use crate::{board::Query, movement::Move};

// This contains the source notation, the calculated Move object, and the board state prior to the
// move.
#[derive(Debug, Clone, PartialEq)]
pub struct HalfPly {
    notation: String, // this should be an Enum or otherwise encode the notation _type_
    mov: Move,
}

impl From<Move> for HalfPly {
    fn from(mov: Move) -> Self {
        Self {
            notation: mov.to_uci(),
            mov,
        }
    }
}

impl From<&str> for HalfPly {
    /// Assumes string is UCI notation
    /// TODO: Implement `Notation`, a type which tags a string with its notation type, allows
    /// conversion to canonical type, and then this method can be implemented for `Notation`.
    fn from(notation: &str) -> Self {
        Self {
            notation: notation.to_string(),
            mov: Move::from_uci(notation),
        }
    }
}

impl Into<Move> for HalfPly {
    fn into(self) -> Move {
        self.mov
    }
}

impl Into<Move> for &HalfPly {
    fn into(self) -> Move {
        self.mov.clone()
    }
}

impl HalfPly {
    pub fn notation(&self) -> &str {
        &self.notation
    }

    pub fn to_pgn<C>(&self, context: &C) -> String where C: Query {
        self.mov.to_pgn(context)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod creation {
        use super::*;

        #[test]
        fn from_uci() {
            let half_ply = HalfPly::from("d2d4");
            dbg!(&half_ply);
            assert_eq!(half_ply.mov.source_idx(), 0o13);
            assert_eq!(half_ply.mov.target_idx(), 0o33);
        }

        #[test]
        fn from_move() {
            let half_ply = HalfPly::from(Move::from_uci("d2d4"));
            assert_eq!(half_ply.mov.source_idx(), 0o13);
            assert_eq!(half_ply.mov.target_idx(), 0o33);
        }
    }

    mod to_pgn {
        use crate::board::{occupant::Occupant, pieceboard::PieceBoard};
        use crate::constants::NOTATION_TO_INDEX;
        use crate::engine::Engine;

        use super::*;


        #[test]
        fn to_pgn_quiet() {
            let mut board = PieceBoard::default();
            board.set_startpos();

            let half_ply = HalfPly::from("d2d3");
            assert_eq!(half_ply.to_pgn(&board), "d3");
        }

        #[test]
        fn to_pgn_double_pawn() {
            let mut board = PieceBoard::default();
            board.set_startpos();


            let half_ply = HalfPly::from("d2d4");
            assert_eq!(half_ply.to_pgn(&board), "d4");
        }

        #[test]
        fn to_pgn_capture() {
            let mut board = PieceBoard::default();
            board.exec_message("position startpos moves d2d4 e7e5");
            let half_ply = HalfPly::from("d4e5");
            assert_eq!(half_ply.to_pgn(&board), "dxe5");
        }

        #[test]
        fn to_pgn_promotion() {
            let mut board = PieceBoard::default();
            board.set(NOTATION_TO_INDEX("a7"), Occupant::white_pawn());
            let half_ply = HalfPly::from("a7a8q");
            assert_eq!(half_ply.to_pgn(&board), "a8=Q");
        }

        #[test]
        fn to_pgn_capture_promotion() {
            let mut board = PieceBoard::default();
            board.set(NOTATION_TO_INDEX("a7"), Occupant::white_pawn());
            board.set(NOTATION_TO_INDEX("b8"), Occupant::black_queen());

            dbg!(&board);

            let half_ply = HalfPly::from("a7b8q");
            assert_eq!(half_ply.to_pgn(&board), "axb8=Q");
        }

        #[test]
        fn to_pgn_kingside_castle() {
            let mut board = PieceBoard::default();
            board.exec_message("position startpos moves e2e4 e7e5 g1f3 b8c6 f1c4 g8f6");

            let half_ply = HalfPly::from("e1g1");
            assert_eq!(half_ply.to_pgn(&board), "O-O");
        }

        #[test]
        fn to_pgn_queenside_castle() {
            // 1. d4 h6 2. Bf4 g6 3. Nc3 f6 4. Qd3 e6 5. O-O-O
            let mut board = PieceBoard::default();
            board.exec_message("position startpos moves d2d4 h7h6 c1f4 g7g6 b1c3 f7f6 d1d3 e7e6");
            let half_ply = HalfPly::from("e1c1");
            assert_eq!(half_ply.to_pgn(&board), "O-O-O");
        }
    }
}
