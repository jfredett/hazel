use super::*;


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

impl PieceBoard {
    #[instrument]
    pub fn set_startpos(&mut self) {
        self.set_fen(&FEN::new(START_POSITION_FEN))
    }

    #[instrument]
    pub fn set_fen(&mut self, fen: &FEN) {
        fen::setup_mut(fen, self);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod fen {
        use super::*;

        use crate::board::interface::query;
        use crate::constants::{EMPTY_POSITION_FEN, START_POSITION_FEN};

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

        #[test]
        pub fn converts_from_borrowed_reference_correctly() {
            let fen = FEN::new(START_POSITION_FEN);
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
}
