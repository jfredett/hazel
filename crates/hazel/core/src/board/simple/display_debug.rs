use std::fmt::{Display, Debug};

use crate::{board::PieceBoard, extensions::query::display_board};


impl Debug for PieceBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_board(self))
    }
}


impl Display for PieceBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_board(self))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use hazel_basic::interface::Query;
    use hazel_basic::{piece::Piece, square::*};
    use crate::board::simple::Occupant;


    #[test]
    pub fn bottom_left_is_a1_display() {
        let mut board = PieceBoard::default();
        board.set(A1, Occupant::white_rook());
        let rep = format!("{}", board);
        let expected_rep = "8 . . . . . . . .
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

    #[test]
    pub fn bottom_left_is_a1_debug() {
        let mut board = PieceBoard::default();
        board.set(A1, Occupant::white_rook());
        let rep = format!("{:?}", board);
        let expected_rep = "8 . . . . . . . .
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
