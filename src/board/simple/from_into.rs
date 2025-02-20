use ben::BEN;

use crate::alter::setup;

use super::*;

impl From<BEN> for PieceBoard {
    fn from(ben: BEN) -> Self {
        let alters = ben.to_alterations();
        setup(alters)
    }
}

impl From<&BEN> for PieceBoard {
    fn from(ben: &BEN) -> Self {
        let alters = ben.to_alterations();
        setup(alters)
    }
}

impl PieceBoard {
    pub fn set_startpos(&mut self) {
        self.set_fen(BEN::start_position())
    }

    pub fn set_fen(&mut self, fen: impl Into<BEN>) {
        let mut alterations = vec![ Alteration::clear() ];
        let new_setup = fen.into();
        alterations.extend(new_setup.to_alterations());
        for alter in alterations {
            self.alter_mut(alter);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod fen {
        use super::*;

        use crate::interface::query;
        use crate::constants::{EMPTY_POSITION_FEN, START_POSITION_FEN};

        #[test]
        pub fn converts_start_position_correctly() {
            let mut board = PieceBoard::default();
            board.set_startpos();
            let fen = query::to_fen(&board);
            assert_eq!(fen, BEN::new(START_POSITION_FEN));
        }

        #[test]
        pub fn converts_empty_board_correctly() {
            let board = PieceBoard::default();
            let fen = query::to_fen(&board);
            assert_eq!(fen, BEN::new(EMPTY_POSITION_FEN));
        }

        #[test]
        pub fn converts_fen_to_board_correctly() {
            let ben = BEN::new(START_POSITION_FEN);
            let board = PieceBoard::from(&ben);
            let fen2 = query::to_fen(&board);
            assert_eq!(ben, fen2.into());
        }

        #[test]
        pub fn converts_each_offset_correctly() {
            let ben = BEN::new("p7/1p6/2p5/3p4/4p3/5p2/6p1/7p w KQkq - 0 1");
            let board : PieceBoard = ben.into();
            let fen2 = query::to_fen(&board);
            assert_eq!(ben, fen2.into());
        }

        #[test]
        pub fn converts_from_borrowed_reference_correctly() {
            let fen = BEN::new(START_POSITION_FEN);
            let mut board = PieceBoard::default();
            board.set_fen(fen);
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
