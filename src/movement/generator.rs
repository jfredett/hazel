use super::*;
use crate::{bitboard::Bitboard, constants::*, moveset::MoveSet, ply::*};

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

pub fn calculate_pawn_moves() -> [[Bitboard; 64]; 2] {
    let mut white_out = [Bitboard::empty(); 64];
    let mut black_out = [Bitboard::empty(); 64];
    // pawn moves, initial rank
    for i in 8..17 {
        let mut wbb = Bitboard::empty();
        wbb.set_by_index(i);
        let mut bbb = Bitboard::empty();
        bbb.set_by_index(64-i);

        wbb |= wbb.shift(Direction::N)
            |  wbb.shift(Direction::N).shift(Direction::N)
            |  wbb.shift(Direction::NE)
            |  wbb.shift(Direction::NW);
            
        bbb |= bbb.shift(Direction::S)
            |  bbb.shift(Direction::S).shift(Direction::S)
            |  bbb.shift(Direction::SE)
            |  bbb.shift(Direction::SW);


        white_out[i] = wbb;
        black_out[64-i] = bbb;
    }

    // all other pawn moves
    for i in 17..64 {
        let mut wbb = Bitboard::empty();
        wbb.set_by_index(i);
        let mut bbb = Bitboard::empty();
        bbb.set_by_index(64-i);

        wbb |= wbb.shift(Direction::N)
            |  wbb.shift(Direction::NE)
            |  wbb.shift(Direction::NW);
            
        bbb |= bbb.shift(Direction::S)
            |  bbb.shift(Direction::SE)
            |  bbb.shift(Direction::SW);

        white_out[i] = wbb;
        black_out[64-i] = bbb;
    }


    [ white_out, black_out ]
}

lazy_static! {
    // NOTE: for all of these we only care about specifying what squares _could_ be moved to, not
    // if that move would be legal or not, we leave legality analysis for the generator
    // That analysis would look something like -- mask off areas based on the opposing color, if they're 
    // 

    /// A lookup table to convert a knight on a given index -> it's bitboard of moves
    pub static ref KNIGHT_MOVES: [Bitboard; 64] = calculate_knight_moves();
    /// A lookup table to convert a pawn on a given index -> it's bitboard of moves
    pub static ref PAWN_MOVES: [[Bitboard; 64]; 2] = calculate_pawn_moves();
}

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