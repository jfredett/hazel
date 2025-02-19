use crate::interface::{alter::Alter, alteration::Alteration, query::Query};
use crate::constants::START_POSITION_FEN;
use crate::types::Occupant;
use crate::notation::*;
use crate::notation::fen::*;

use tracing::instrument;

pub mod display_debug;
pub mod from_into;


#[derive(Clone, Copy, PartialEq)]
pub struct PieceBoard {
    pub board: [Occupant; 64],
}

impl Default for PieceBoard {
    fn default() -> Self {
        Self { board: [Occupant::empty(); 64] }
    }
}

pub struct OccupantIterator<Q> where Q : Query {
    idx: RankFile,
    // FIXME: this should probably be a RO reference
    source: Q
}

impl<Q> Iterator for OccupantIterator<Q> where Q : Query {
    type Item = (Square, Occupant);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(sq) = self.idx.next() else { return None; };
            if self.source.is_occupied(sq) { return Some((sq, self.source.get(sq))); }
        }
    }
}

impl PieceBoard {
    pub fn set(&mut self, square: impl Into<Square>, occupant: Occupant) {
        let sq = square.into();

        self.board[sq.index()] = occupant;
    }

    pub fn by_occupant(&self) -> OccupantIterator<PieceBoard> {
        OccupantIterator {
            source: self.clone(),
            idx: Square::by_rank_and_file()
        }

    }
}

impl Query for PieceBoard {
    fn get(&self, square: impl Into<Square>) -> Occupant {
        let sq = square.into();
        self.board[sq.index()]
    }
}

impl From<PieceBoard> for FEN {
    fn from(board: PieceBoard) -> Self {
        crate::interface::query::to_fen(&board)
    }
}

impl Alter for PieceBoard {
    #[instrument]
    fn alter(&self, alter: Alteration) -> PieceBoard {
        let mut board = *self;
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
        pub fn alter_returns_new_board() {
            let b1 = PieceBoard::default();
            let b2 = b1.alter(Alteration::place(D5, Occupant::white_pawn()));

            assert!(b1 != b2);
            assert_eq!(b1.get(D5), Occupant::empty());
            assert_eq!(b2.get(D5), Occupant::white_pawn());
        }

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
}
