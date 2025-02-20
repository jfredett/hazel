use crate::{board::PieceBoard, coup::rep::Move, notation::ben::BEN, types::{Bitboard, Color, Occupant, Piece}, Alter, Alteration};


#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    // necessaries
    pub initial: BEN,
    pub moves: Vec<Move>,
    // caches

    // Alteration Cache should be by piece and color, so I can selectively reconstruct bitboards
    // from the alterations.
    board: PieceBoard,
    pub(crate) alteration_cache: Vec<Alteration>
}

// adding a move should lazily update cached representations, we might get several moves at once.
// We also need to be able to un-apply moves from the alteration cache piecemeal.
//
//

impl From<Position> for Vec<Alteration> {
    fn from(pos: Position) -> Self {
        let mut ret = pos.initial.compile();
        let mut board = PieceBoard::from(pos.initial);
        for m in pos.moves.iter() {
            let alterations = m.compile(&board);
            for a in alterations.iter() {
                board.alter_mut(*a);
            }
            ret.extend(alterations);
        }
        ret
    }
}

// TODO: this'll implement play at some point

impl Position {
    pub fn new(fen: impl Into<BEN>, moves: Vec<Move>) -> Self {
        let fen = fen.into();
        let mut board = PieceBoard::from(fen);
        let mut alteration_cache = fen.compile();

        for mov in &moves {
            let alterations = mov.compile(&board);
            for alteration in alterations {
                alteration_cache.push(alteration);
                board.alter_mut(alteration);
            }
        }
        Self { initial: fen.into(), moves, board, alteration_cache }
    }


    pub fn pawns_for(&self, color: &Color) -> Bitboard {
        let mut bb = Bitboard::empty();
        for (sq, _) in self.board.by_occupant().filter(|(_, o)| o.piece().unwrap() == Piece::Pawn && o.color().unwrap() == *color) {
            bb.set(sq);
        }
        bb
    }

    pub fn all_blockers(&self) -> Bitboard {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;


        #[test]
        fn correctly_caches_start_position() {
            let pos = Position::new(BEN::start_position(), vec![]);
            assert_eq!(pos.alteration_cache, vec![

            ]);

        }
    }

    mod pawns_for {

        use super::*;

        #[test]
        fn startpos() {
            let pos = Position::new(BEN::start_position(), vec![]);
            assert_eq!(
                pos.pawns_for(&Color::WHITE),
                Color::WHITE.pawn_mask()
            );

            assert_eq!(
                pos.pawns_for(&Color::BLACK),
                Color::BLACK.pawn_mask()
            );
        }


    }



}
