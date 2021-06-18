use crate::{constants::{Color, DIRECTIONS}, moveset::MoveSet, ply::Ply};

use super::Move;
use crate::constants::move_tables::*;


impl Move {
    /// Generates all valid moves from the given ply.
    /// NOTE: Initial version is quite naive and does no precomputation. This is intentional.
    ///     Future versions will be refactored from this to build a faster algorithm.
    pub fn generate(&ply : &Ply, color: Color) -> MoveSet {
        let mut out : MoveSet = MoveSet::empty();
        let other_color = match color {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE
        };

        // pawn moves
        for source in ply.pawns[color as usize].all_set_indices() {
            let target_board = PAWN_MOVES[color as usize][source];
            for target in target_board.all_set_indices() {
                // if it's a promotion, push the promotion moves
                if target >= 56 || target <= 8 { // on the first or last rank
                    out.add_promotion(source, target);
                } 
                
                // the bottom 3 bits of an index determine it's file.
                if (source & 0b0111) == (target & 0b0111) { // advances
                    if !(ply.occupancy_for(color) & target_board).is_empty() {
                        if !ply.occupancy_for(color).all_set_indices().contains(&target) { 
                            out.add_move(source, target);
                        }
                    } 
                } else { // captures
                    if !(ply.occupancy_for(other_color) & target_board).is_empty() {
                        if ply.occupancy_for(other_color).all_set_indices().contains(&target) { 
                            out.add_capture(source, target);
                        }
                    }
                }
            }
        }
        // king moves
        // FIXME: Doesn't account for checks yet.
        let king = ply.kings[color as usize];
        let source = king.all_set_indices()[0];
        for d in DIRECTIONS {
            let m = king.shift(d);
            if m.is_empty() { continue; }

            if (m & ply.occupancy_for(color)).is_empty() {
                let target = m.all_set_indices()[0];
                if ply.occupancy_for(other_color).is_index_set(target) {
                    out.add_capture(source, target);
                } else {
                    out.add_move(source, target);
                }
            }
        }
        // knight moves
        let knights = ply.knights[color as usize];
        for k in knights.all_set_indices() {
            let unblocked_squares = KNIGHT_MOVES[k] & !ply.occupancy_for(color);
            for square in unblocked_squares.all_set_indices() {
                out.add_move(k, square);
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
    

    // TODO: Have a yaml file which describes a bunch of test positions and the valid moves they entail, load them, then generate tests 
    // from them, we can do this by taking random positions from a database, using stockfish to perft 1 them, then grab the results.

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
    
    #[test]
    fn calculates_correct_movecount_kiwipete() {
        let ply = Ply::from_fen(&String::from(POS2_KIWIPETE_FEN));
        let moves = Move::generate(&ply, ply.current_player());
        assert_eq!(moves.len(), POS2_KIWIPETE_PERFT_COUNTS[0]);
    }
    
    #[test]
    fn calculates_moves_for_kiwipete_position_at_depth_1() {
        let ply = Ply::from_fen(&String::from(POS2_KIWIPETE_FEN));
        let moves = Move::generate(&ply, ply.current_player());
        for m in POS2_KIWIPETE_MOVES.iter() {
            if !moves.contains(&m) { dbg!("missing move", m); }
            assert!(moves.contains(&m));
        }
    }
    
}