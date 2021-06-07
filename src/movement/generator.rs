use super::*;
use crate::{
    bitboard::{Direction},
    constants::{Color, FILE_MASKS, RANK_MASKS, RANK_1, RANK_2, RANK_7, RANK_8}, 
    ply::*
};

impl Move {
    /// Generates all valid moves from the given ply.
    /// NOTE: Initial version is quite naive and does no precomputation. This is intentional.
    ///     Future versions will be refactored from this to build a faster algorithm.
    pub fn generate(&ply : &Ply, color: Color) -> Vec<Move> {
        let mut out : Vec<Move> = vec![];
        let occupancy = ply.occupancy();
        // pawn moves
        for &file in FILE_MASKS.iter() {
            for &rank in RANK_MASKS.iter() {
                let pawn = ply.pawns[color as usize] & file & rank;
                if pawn.is_empty() { continue; }
                if (occupancy & pawn.shift(Direction::N)).is_empty() {
                    let shifted = pawn.shift(Direction::N);
                    let source_idx = pawn.all_set_indices()[0];
                    let target_idx = shifted.all_set_indices()[0];
                    if rank == *RANK_8 || rank == *RANK_1 {
                        // we can promote to any piece we like.
                        out.push(Move::from(source_idx as u16, target_idx as u16, true, 0b00));
                        out.push(Move::from(source_idx as u16, target_idx as u16, true, 0b01));
                        out.push(Move::from(source_idx as u16, target_idx as u16, true, 0b10));
                        out.push(Move::from(source_idx as u16, target_idx as u16, true, 0b11));
                    } else {
                        // TODO: Do attack/check/capture checking
                        out.push(Move::from(source_idx as u16, target_idx as u16, false, 0b000));
                    }
                }

                if rank == *RANK_2 || rank == *RANK_7 {
                    if (occupancy & pawn.shift(Direction::N).shift(Direction::N)).is_empty() {
                        // TODO: maybe a bb_shift! macro for shifting by arbitrary amounts?
                        // TODO: extract the source_idx/target_idx stuff into a helper
                        let shifted = pawn.shift(Direction::N).shift(Direction::N);
                        let source_idx = pawn.all_set_indices()[0];
                        let target_idx = shifted.all_set_indices()[0];
                        // TODO: Do attack/check/capture checking
                        out.push(Move::from(source_idx as u16, target_idx as u16, false, 0b000));
                    }
                }
            }
        }
        // king moves
        // knight moves
        let knights = ply.knights[color as usize];
        for &file in FILE_MASKS.iter() {
            for &rank in RANK_MASKS.iter() {
                let k = knights & file & rank;
                if k.is_empty() { continue; }

                let source_idx = k.all_set_indices()[0];
                let k_nne = k.shift(Direction::N).shift(Direction::N).shift(Direction::E);
                // if it's not off-the-board, and it's not blocked by our guys, then we add it
                if !k_nne.is_empty() && (k_nne & ply.occupancy_for(color)).is_empty() {
                    let target_idx = k_nne.all_set_indices()[0];
                    // TODO: attack/check/capture checking
                    out.push(Move::from(source_idx as u16, target_idx as u16, false, 0b000)) 
                }

                let k_nnw = k.shift(Direction::N).shift(Direction::N).shift(Direction::W);
                // if it's not off-the-board, and it's not blocked by our guys, then we add it
                if !k_nnw.is_empty() && (k_nnw & ply.occupancy_for(color)).is_empty() {
                    let target_idx = k_nnw.all_set_indices()[0];
                    // TODO: attack/check/capture checking
                    out.push(Move::from(source_idx as u16, target_idx as u16, false, 0b000)) 
                }
            }
        }
        // rook moves
        // bishop moves
        // queen moves
        out
    }
}


#[cfg(test)]
mod test {
    use crate::constants::{START_POSITION_FEN, STARTING_MOVES};

    use super::*;
    

    #[test]
    fn calculates_starting_position_moves() {
        let ply = Ply::from_fen(&String::from(START_POSITION_FEN));
        let moves = Move::generate(&ply, Color::WHITE);
        for m in STARTING_MOVES.iter() {
            if !moves.contains(&m) { dbg!("missing move", m); }
            assert!(moves.contains(&m));
        }
    }
}