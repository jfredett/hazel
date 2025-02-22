use crate::{board::PieceBoard, coup::rep::Move, notation::{ben::BEN, Square}, types::{Bitboard, Color, Occupant, Piece}, Alter, Alteration, Query};

use super::position_metadata::PositionMetadata;


#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    // necessaries
    pub initial: BEN,
    pub moves: Vec<Move>,
    metadata: PositionMetadata,
    // caches

    // Alteration Cache should be by piece and color, so I can selectively reconstruct bitboards
    // from the alterations.
    pub(crate) board: PieceBoard,
    pub(crate) alteration_cache: Vec<Alteration>
}

// adding a move should lazily update cached representations, we might get several moves at once.
// We also need to be able to un-apply moves from the alteration cache piecemeal.
//
//

impl Query for Position {
    fn get(&self, square: impl Into<Square>) -> Occupant {
        self.board.get(square)
    }

    fn metadata(&self) -> Option<PositionMetadata> {
        Some(self.metadata)
    }
}


// TODO: this'll implement play at some point

impl Position {
    pub fn new(fen: impl Into<BEN>, moves: Vec<Move>) -> Self {
        let fen = fen.into();
        let mut board = PieceBoard::default();
        board.set_position(fen);

        let mut metadata = fen.metadata();
        let mut alteration_cache : Vec<Alteration> = fen.to_alterations().collect();

        for mov in &moves {
            let alterations = mov.compile(&board);
            for alteration in alterations {
                alteration_cache.push(alteration);
                board.alter_mut(alteration);
            }
            metadata.update(mov, &board);
        }
        Self { initial: fen.into(), moves, metadata, board, alteration_cache }
    }

    pub fn all_pieces_of(&self, color: &Color) -> Bitboard {
        let mut bb = Bitboard::empty();
        for (sq, _) in self.board.by_occupant().filter(|(_, o)| o.color().unwrap() == *color) {
            bb.set(sq);
        }
        bb
    }


    pub fn pawns_for(&self, color: &Color) -> Bitboard {
        let mut bb = Bitboard::empty();
        for (sq, _) in self.board.by_occupant().filter(|(_, o)| o.piece().unwrap() == Piece::Pawn && o.color().unwrap() == *color) {
            bb.set(sq);
        }
        bb
    }

    pub fn all_blockers(&self) -> Bitboard {
        let mut bb = Bitboard::empty();
        for (sq, _) in self.board.by_occupant().filter(|(_, o)| o.is_occupied()) {
            bb.set(sq);
        }
        bb
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use crate::game::castle_rights::CastleRights;

        use super::*;


        #[test]
        fn correctly_caches_start_position() {
            let pos = Position::new(BEN::start_position(), vec![]);
            use crate::notation::*;

            assert_eq!(pos.alteration_cache, vec![
                Alteration::Clear,
                Alteration::Assert(PositionMetadata {
                    side_to_move: Color::WHITE,
                    castling: CastleRights {
                        white_short: true,
                        white_long: true,
                        black_short: true,
                        black_long: true
                    }, 
                    en_passant: None,
                    halfmove_clock: 0,
                    fullmove_number: 1
                }),
                Alteration::place(A1, Occupant::white_rook()),
                Alteration::place(B1, Occupant::white_knight()),
                Alteration::place(C1, Occupant::white_bishop()),
                Alteration::place(D1, Occupant::white_queen()),
                Alteration::place(E1, Occupant::white_king()),
                Alteration::place(F1, Occupant::white_bishop()),
                Alteration::place(G1, Occupant::white_knight()),
                Alteration::place(H1, Occupant::white_rook()),
                Alteration::place(A2, Occupant::white_pawn()),
                Alteration::place(B2, Occupant::white_pawn()),
                Alteration::place(C2, Occupant::white_pawn()),
                Alteration::place(D2, Occupant::white_pawn()),
                Alteration::place(E2, Occupant::white_pawn()),
                Alteration::place(F2, Occupant::white_pawn()),
                Alteration::place(G2, Occupant::white_pawn()),
                Alteration::place(H2, Occupant::white_pawn()),
                Alteration::place(A7, Occupant::black_pawn()),
                Alteration::place(B7, Occupant::black_pawn()),
                Alteration::place(C7, Occupant::black_pawn()),
                Alteration::place(D7, Occupant::black_pawn()),
                Alteration::place(E7, Occupant::black_pawn()),
                Alteration::place(F7, Occupant::black_pawn()),
                Alteration::place(G7, Occupant::black_pawn()),
                Alteration::place(H7, Occupant::black_pawn()),
                Alteration::place(A8, Occupant::black_rook()),
                Alteration::place(B8, Occupant::black_knight()),
                Alteration::place(C8, Occupant::black_bishop()),
                Alteration::place(D8, Occupant::black_queen()),
                Alteration::place(E8, Occupant::black_king()),
                Alteration::place(F8, Occupant::black_bishop()),
                Alteration::place(G8, Occupant::black_knight()),
                Alteration::place(H8, Occupant::black_rook()),
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
