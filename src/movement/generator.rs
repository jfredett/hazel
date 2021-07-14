use tracing::info;

use crate::{bitboard::Bitboard, constants::{A_FILE, Color, DIRECTIONS, Direction, H_FILE, NOTATION_TO_INDEX, Piece, RANK_1, RANK_2, RANK_7, RANK_8}, moveset::MoveSet, pextboard::{self, slow_bishop_attacks}, ply::Ply};



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
        
        for sq in promotions.all_set_indices()   { out.add_promotion(deshift(sq), sq); }
        for sq in advances.all_set_indices()     { out.add_move(Piece::Pawn, deshift(sq), sq); }
        for sq in double_moves.all_set_indices() { out.add_move(Piece::Pawn, deshift(deshift(sq)), sq); }
        for sq in east_attacks.all_set_indices() { out.add_capture(Piece::Pawn, deshift(sq) - 1, sq); }
        for sq in west_attacks.all_set_indices() { out.add_capture(Piece::Pawn, deshift(sq) + 1, sq); }

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
                    out.add_capture(Piece::King, source, target);
                } else {
                    out.add_move(Piece::King, source, target);
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
                out.add_move(Piece::Knight, k, square);
            }
        }

        for piece in [Piece::Bishop, Piece::Rook, Piece::Queen] {
            for source in ply.get_piece(color, piece).all_set_indices() {
                let attacks = pextboard::attacks_for(piece, source, ply.occupancy()) & !ply.occupancy_for(color);
                if piece == Piece::Queen {
                    dbg!(attacks);
                }

                for capture in (attacks & ply.occupancy_for(other_color)).all_set_indices() {
                    out.add_capture(piece, source, capture)
                }
                
                for target in (attacks & !ply.occupancy_for(other_color)).all_set_indices() {
                    out.add_move(piece, source, target);
                }
            }
        }
        out
    }
}


#[cfg(test)]
mod test {
    use tracing_test::traced_test;

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

        for movset in moves.clone().moves {
            assert_is_subset!(movset, *POS2_KIWIPETE_MOVES);
        }
        dbg!("Missing Moves");
        for mov in POS2_KIWIPETE_MOVES.iter() {
            assert!(
                moves.moves.iter().any(|movset| 
                    movset.contains(mov)
                )
            )
        }
    }
    
}