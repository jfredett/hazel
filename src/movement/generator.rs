use super::*;
use crate::{
    bitboard::{Bitboard, Direction},
    constants::*,
    ply::*
};


pub fn calculate_knight_moves() -> [Bitboard; 64] {
    let mut out : [Bitboard; 64] = [Bitboard::empty(); 64];
    for i in 0..64 {
            let mut bb = Bitboard::empty();
            bb.set_by_index(i);
            
            let position_board = bb.shift(Direction::N).shift(Direction::N).shift(Direction::E) // NNE
                               | bb.shift(Direction::N).shift(Direction::N).shift(Direction::W) // NNW
                               | bb.shift(Direction::W).shift(Direction::W).shift(Direction::N) // WWN
                               | bb.shift(Direction::W).shift(Direction::W).shift(Direction::S) // WWS
                               | bb.shift(Direction::S).shift(Direction::S).shift(Direction::W) // SSW
                               | bb.shift(Direction::S).shift(Direction::S).shift(Direction::E) // SSE
                               | bb.shift(Direction::E).shift(Direction::E).shift(Direction::S) // EES
                               | bb.shift(Direction::E).shift(Direction::E).shift(Direction::N) // EEN
                               ;
            out[i] = position_board;
    }
    out
}

lazy_static! {
    /// A lookup table to convert a knight on idx `n` -> it's bitboard of attacks/moves
    pub static ref KNIGHT_MOVES: [Bitboard; 64] = calculate_knight_moves();

    
}



impl Move {
    /// Generates all valid moves from the given ply.
    /// NOTE: Initial version is quite naive and does no precomputation. This is intentional.
    ///     Future versions will be refactored from this to build a faster algorithm.
    pub fn generate(&ply : &Ply, color: Color) -> Vec<Move> {
        let mut out : Vec<Move> = vec![];
        let occupancy = ply.occupancy();
        let pawn_direction = match color {
            Color::WHITE => Direction::N,
            Color::BLACK => Direction::S
        };
        // pawn moves
        for &file in FILE_MASKS.iter() {
            for &rank in RANK_MASKS.iter() {
                let pawn = ply.pawns[color as usize] & file & rank;
                if pawn.is_empty() { continue; }
                if (occupancy & pawn.shift(pawn_direction)).is_empty() {
                    let shifted = pawn.shift(pawn_direction);
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
                    if (occupancy & pawn.shift(pawn_direction).shift(pawn_direction)).is_empty() {
                        // TODO: maybe a bb_shift! macro for shifting by arbitrary amounts?
                        // TODO: extract the source_idx/target_idx stuff into a helper
                        let shifted = pawn.shift(pawn_direction).shift(pawn_direction);
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
        for k in knights.all_set_indices() {
            let unblocked_squares = KNIGHT_MOVES[k] & !ply.occupancy_for(color);
            for square in unblocked_squares.all_set_indices() {
                out.push(Move::from(k as u16, square as u16, false, 0b000));
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
    use crate::constants::*;
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
    
    #[test]
    fn calculates_move_after_1_d4_correctly() {
        let ply = Ply::from_fen(&String::from(D4_POSITION_FEN));
        let moves = Move::generate(&ply, Color::BLACK);
        for m in D4_MOVES.iter() {
            if !moves.contains(&m) { dbg!("missing move", m); }
            assert!(moves.contains(&m));
        }
    }
}