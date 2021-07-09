use crate::{constants::{A_FILE, Color, DIRECTIONS, Direction, H_FILE, Piece, RANK_1, RANK_2, RANK_7, RANK_8}, moveset::MoveSet, pextboard, ply::Ply};



use super::Move;
use crate::constants::move_tables::*;


impl Move {
    /// Generates all valid moves from the given ply.
    pub fn generate(&ply : &Ply, color: Color) -> MoveSet {
        let mut out : MoveSet = MoveSet::empty();
        let (other_color, pawn_direction, promotion_rank, double_jump_rank) = match color {
            Color::WHITE => (Color::BLACK, Direction::N, *RANK_8, *RANK_2),
            Color::BLACK => (Color::WHITE, Direction::S, *RANK_1, *RANK_7)
        };

        // pawn moves
        let pawns = ply.pawns[color as usize];
        let raw_advances = pawns.shift(pawn_direction) & !ply.occupancy();
        let promotions = raw_advances & promotion_rank;
        let advances = raw_advances & !promotion_rank;
        let double_moves = ((pawns & double_jump_rank).shift(pawn_direction) & !ply.occupancy())
                                 .shift(pawn_direction) & !ply.occupancy();
        let east_attacks = (pawns & !*H_FILE).shift(pawn_direction).shift(Direction::E) & ply.occupancy_for(other_color);
        let west_attacks = (pawns & !*A_FILE).shift(pawn_direction).shift(Direction::W) & ply.occupancy_for(other_color);

        let deshift = match pawn_direction {
            Direction::N => |e: usize| e - 8,
            Direction::S => |e: usize| e + 8,
            _ => unreachable!()
        };
        
        for sq in promotions.all_set_indices() { out.add_promotion(deshift(sq), sq); }
        for sq in advances.all_set_indices() { out.add_move(deshift(sq), sq); }
        for sq in double_moves.all_set_indices() { out.add_move(deshift(deshift(sq)), sq); }
        for sq in east_attacks.all_set_indices() { out.add_capture(deshift(sq) - 1, sq); }
        for sq in west_attacks.all_set_indices() { out.add_capture(deshift(sq) + 1, sq); }

        // king moves
        // FIXME: Doesn't account for checks yet.
        // NOTE: We should check king moves first to see if we're in check, since we can bail earlier if we are.
        let king = ply.kings[color as usize];
        let source = king.first_index();
        for d in DIRECTIONS {
            let m = king.shift(d);
            if m.is_empty() { continue; }

            if (m & ply.occupancy_for(color)).is_empty() {
                let target = m.first_index();
                if ply.occupancy_for(other_color).is_index_set(target) {
                    out.add_capture(source, target);
                } else {
                    out.add_move(source, target);
                }
            }
        }
        
        // Castling
        if ply.can_castle_short() { out.add_short_castle(color); }
        if ply.can_castle_long() { out.add_long_castle(color); }

        // knight moves
        let knights = ply.knights[color as usize];
        for k in knights.all_set_indices() {
            let unblocked_squares = KNIGHT_MOVES[k] & !ply.occupancy_for(color);
            for square in unblocked_squares.all_set_indices() {
                out.add_move(k, square);
            }
        }

        // rook moves
        for source in ply.rooks[color as usize].all_set_indices() {
            let attacks = pextboard::attacks_for(Piece::Rook, source, ply.occupancy()) & !ply.occupancy_for(color);
            for target in attacks.all_set_indices() {
                if ply.occupancy_for(other_color).is_index_set(target) {
                    out.add_capture(source, target)    
                } else {
                    out.add_move(source, target);
                }
            }
        }

        // bishop moves
        for source in ply.bishops[color as usize].all_set_indices() {
            let attacks = pextboard::attacks_for(Piece::Bishop, source, ply.occupancy()) & !ply.occupancy_for(color);
            for target in attacks.all_set_indices() {
                if ply.occupancy_for(other_color).is_index_set(target) {
                    out.add_capture(source, target)    
                } else {
                    out.add_move(source, target);
                }
            }
        }

        // queen moves
        for source in ply.queens[color as usize].all_set_indices() {
            let attacks = pextboard::attacks_for(Piece::Queen, source, ply.occupancy()) & !ply.occupancy_for(color);
            for target in attacks.all_set_indices() {
                if ply.occupancy_for(other_color).is_index_set(target) {
                    out.add_capture(source, target)    
                } else {
                    out.add_move(source, target);
                }
            }
        }
        out
    }
}


#[cfg(test)]
mod test {
    use crate::{assert_is_subset, constants::*};
    use super::*;
    

    // TODO: Have a yaml file which describes a bunch of test positions and the valid moves they entail, load them, then generate tests 
    // from them, we can do this by taking random positions from a database, using stockfish to perft 1 them, then grab the results.

    #[test]
    fn calculates_starting_position_moves() {
        let ply = Ply::from_fen(&String::from(START_POSITION_FEN));
        let moves = Move::generate(&ply, Color::WHITE);
        for m in STARTING_MOVES.iter() {
            if !moves.contains(m) { dbg!("missing move", m); }
            assert!(moves.contains(m));
        }
    }
    
    #[test]
    fn calculates_move_after_1_d4_correctly() {
        let ply = Ply::from_fen(&String::from(D4_POSITION_FEN));
        let moves = Move::generate(&ply, Color::BLACK);
        for m in D4_MOVES.iter() {
            if !moves.contains(m) { dbg!("missing move", m); }
            assert!(moves.contains(m));
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

        assert_is_subset!(&moves.moves, *POS2_KIWIPETE_MOVES);
        dbg!("Missing Moves");
        assert_is_subset!(POS2_KIWIPETE_MOVES.iter(), &moves.moves);
    }
    
}