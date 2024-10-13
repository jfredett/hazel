#![allow(dead_code)] // this is a stub for a future refactor

use super::*;

impl Ply {
    // TODO: Rename this to moves and replace the other implementation
    /// Generates a moveset of all the available moves for the current player.
    fn new_moves(&self) -> MoveSet {
        let mut out = MoveSet::empty();
        // FIXME: This is almost certainly a wildly inefficient way to do this. Each call to merge
        // is a O(n) loop where this probably could be O(1)
        out.merge(self.pawn_advances())
            .merge(self.pawn_attacks())
            .merge(self.pawn_double_advances())
            .merge(self.knight_moves())
            .merge(self.bishop_moves())
            .merge(self.rook_moves())
            .merge(self.queen_moves())
            .merge(self.king_moves());

        out
    }

    fn my_pawns(&self) -> Bitboard {
        self.pawns_for(self.current_player())
    }
    fn my_knights(&self) -> Bitboard {
        self.knights_for(self.current_player())
    }
    fn my_bishops(&self) -> Bitboard {
        self.bishops_for(self.current_player())
    }
    fn my_rooks(&self) -> Bitboard {
        self.rooks_for(self.current_player())
    }
    fn my_queens(&self) -> Bitboard {
        self.queens_for(self.current_player())
    }
    fn my_king(&self) -> Bitboard {
        self.king_for(self.current_player())
    }

    fn king_moves(&self) -> MoveSet {
        todo!()
    }

    /// Calculate all single-pawn advances
    fn pawn_advances(&self) -> MoveSet {
        let moves = self.my_pawns().shift(self.pawn_direction()) & !self.occupancy();
        let mut out = MoveSet::empty();

        for target_idx in moves {
            // using enemy pawn direction here is a little hacky, basically we need to 'put back'
            // the pawn so we know where we started.
            out.add_move(
                Piece::Pawn,
                self.enemy_pawn_direction().index_shift(target_idx),
                target_idx,
            );
        }

        out
    }

    fn pawn_double_advances(&self) -> MoveSet {
        todo!()
    }

    fn pawn_attacks(&self) -> MoveSet {
        todo!()
    }

    fn knight_moves(&self) -> MoveSet {
        todo!()
    }

    fn bishop_moves(&self) -> MoveSet {
        todo!()
    }

    fn rook_moves(&self) -> MoveSet {
        todo!()
    }

    fn queen_moves(&self) -> MoveSet {
        todo!()
    }
}
