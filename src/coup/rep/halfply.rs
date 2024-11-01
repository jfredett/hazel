use crate::board::interface::{Alter, Alteration, Query};
use crate::coup::rep::Move;
use crate::notation::uci::UCI;


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
        let m = UCI::try_from(notation).unwrap_or_else(|_| panic!("Invalid Move: {}", notation));
        Self {
            notation: notation.to_string(),
            mov: m.into(),
        }
    }
}

impl From<&HalfPly> for Move {
    fn from(half_ply: &HalfPly) -> Self {
        half_ply.mov
    }
}

impl From<HalfPly> for Move {
    fn from(half_ply: HalfPly) -> Self {
        half_ply.mov
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
    use crate::notation::*;

    mod creation {
        use super::*;

        #[test]
        fn from_notation() {
            let half_ply = HalfPly::from("d2d4");
            // TODO: Demeter
            assert_eq!(half_ply.mov.source(), D2);
            assert_eq!(half_ply.mov.target(), D4);
        }
    }

    mod to_pgn {
        use crate::types::{Occupant, Color};
        use crate::board::interface::{Alter, Alteration, Query};
        use crate::board::simple::PieceBoard;
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
            board.set_startpos();
            board.alter_mut(Alteration::remove(D2, Occupant::white_pawn()));
            board.alter_mut(Alteration::place(D4, Occupant::white_pawn()));
            board.alter_mut(Alteration::remove(E7, Occupant::black_pawn()));
            board.alter_mut(Alteration::place(E5, Occupant::black_pawn()));

            let half_ply = HalfPly::from("d4e5");
            assert_eq!(half_ply.to_pgn(&board), "dxe5");
        }

        #[test]
        fn to_pgn_promotion() {
            let mut board = PieceBoard::default();
            board.set(A7, Occupant::white_pawn());
            let half_ply = HalfPly::from("a7a8q");
            assert_eq!(half_ply.to_pgn(&board), "a8=Q");
        }

        #[test]
        fn to_pgn_capture_promotion() {
            let mut board = PieceBoard::default();
            board.set(A7, Occupant::white_pawn());
            board.set(B8, Occupant::black_queen());


            let half_ply = HalfPly::from("a7b8q");
            assert_eq!(half_ply.to_pgn(&board), "axb8=Q");
        }

        #[test]
        #[ignore]
        fn to_pgn_kingside_castle() {
            let mut board = PieceBoard::default();

            let half_ply = HalfPly::from("e1g1");
            assert_eq!(half_ply.to_pgn(&board), "O-O");
        }

        #[test]
        #[ignore]
        fn to_pgn_queenside_castle() {
            // 1. d4 h6 2. Bf4 g6 3. Nc3 f6 4. Qd3 e6 5. O-O-O
            let mut board = PieceBoard::default();
            let half_ply = HalfPly::from("e1c1");
            assert_eq!(half_ply.to_pgn(&board), "O-O-O");
        }
    }
}
