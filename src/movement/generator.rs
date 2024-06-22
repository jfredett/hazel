use crate::{
    constants::{move_tables::*, Color, Direction, Piece, A_FILE, H_FILE},
    moveset::MoveSet,
    pextboard::{self},
    ply::Ply,
    bitboard::Bitboard,
};


use super::Move;

impl Move {
    //FIXME: This is in the wrong place, this should live on ply.

    /// Generates all valid moves from the given ply.
    pub fn generate(&ply: &Ply, color: Color) -> MoveSet {
        let mut out: MoveSet = MoveSet::empty();

        // king moves -- covers avoiding checked squares, does not cover if king is already in check.
        let king = ply.king_for(color);
        let source = king.first_index();
        let forbidden_squares = ply.attacked_squares_for(!color);
        let in_check = (king & forbidden_squares).is_nonempty();

        // Building the Moveset
        let blocking_squares = if in_check {
            // If we are in check, then we need to screen all the moves by the squares which
            // could block the check (or capture the attacking piece).

            // Pretend the King is a Queen, any squares which are attackable are squares
            // from which the king can be checked and a block is possible (you cannot block
            // a knight, so we don't need to consider those attacks.
            let king_rays = pextboard::attacks_for(Piece::Queen, source, ply.occupancy());
            let potential_attackers = ply.occupancy_for(!color);
            // then intersect with all the squares we attack to determine which square could
            // be blocked to resolve a check.
            (ply.attacked_squares_for(color) | potential_attackers) & king_rays
        } else {
            // No movement is blocked, so we can mask with the bitboard of all 1's
            Bitboard::full()
        };


        let nominal_king_attacks = ply.king_attack_board_for(color);
        let king_attacks = nominal_king_attacks & !(forbidden_squares | ply.occupancy_for(color));

        let king_captures =
            king_attacks & ply.occupancy_for(!color) & !ply.defended_pieces_for(!color);
        let king_moves = king_attacks & !ply.occupancy_for(!color);

        // Winning Mate: If it's our turn and the enemy king is already in check, we win.
        if (ply.king_for(!color) & ply.attacked_squares_for(color)).is_nonempty() { return out; }
        // Losing Mate: If it's out turn and we are in check and have no moves, we lose.
        if in_check && blocking_squares.is_empty() && king_moves.is_empty() { return out; }

        for capture in king_captures.all_set_indices() {
            out.add_capture(Piece::King, source, capture);
        }

        for target in king_moves.all_set_indices() {
            out.add_move(Piece::King, source, target)
        }


        // pawn moves
        let pawns = ply.pawns_for(color);
        let raw_advances = pawns.shift(ply.pawn_direction()) & !ply.occupancy() & blocking_squares;
        let promotions = raw_advances & color.promotion_rank() & blocking_squares;
        let advances = raw_advances & !color.promotion_rank() & blocking_squares;
        let double_moves = ((pawns & color.pawn_rank()).shift(ply.pawn_direction())
            & !ply.occupancy())
            .shift(ply.pawn_direction())
            & !ply.occupancy();
        let east_attacks_raw = (pawns & !*H_FILE)
            .shift(ply.pawn_direction())
            .shift(Direction::E)
            & ply.occupancy_for(!color);
        let west_attacks_raw = (pawns & !*A_FILE)
            .shift(ply.pawn_direction())
            .shift(Direction::W)
            & ply.occupancy_for(!color);
        let east_attacks = east_attacks_raw & !color.promotion_rank() & blocking_squares;
        let west_attacks = west_attacks_raw & !color.promotion_rank() & blocking_squares;
        let east_attack_promotions = east_attacks_raw & color.promotion_rank() & blocking_squares;
        let west_attack_promotions = west_attacks_raw & color.promotion_rank() & blocking_squares;


        /* TODO:
         * 1. Probably break this into some smaller functions, even if it's artificial.
         * 2. Do king moves first, make sure we're not in check before generating other moves. We already avoid walking into check, so now
         *    we just need to detect check (including doublechecks). I think that's the remaining issue in my perft impl
         * 3. there is a pattern I keep repeating -- generate the attacks bb, isolate captures, isolate moves. That's probably extractable (most
         *    of it is in ply/attacks.rs already), so refactor this thing to use that. Pawns are still tricky, but attacks == moves for all other
         *    pieces so should work well.
         * 4. I do wonder if a table-based approach would be better for pawns, if only because it's cleaner looking than all this algebra.
         */
        if let Some(ep_square) = ply.en_passant() {
            let ep_attackers = (ep_square
                .shift(ply.enemy_pawn_direction())
                .shift(Direction::E)
                | ep_square
                .shift(ply.enemy_pawn_direction())
                .shift(Direction::W))
                & pawns
                & blocking_squares;
            for sq in ep_attackers.all_set_indices() {
                out.add_en_passant_capture(sq, ep_square.first_index());
            }
        }

        let deshift = match ply.pawn_direction() {
            Direction::N => |e: usize| e - 8,
            Direction::S => |e: usize| e + 8,
            _ => unreachable!(),
        };

        // this kind of promotion _cannot_ be a capture since it's a forward move
        for sq in promotions.all_set_indices() {
            out.add_promotion(deshift(sq), sq, false);
        }
        for sq in advances.all_set_indices() {
            out.add_move(Piece::Pawn, deshift(sq), sq);
        }
        for sq in double_moves.all_set_indices() {
            out.add_pawn_double_move(deshift(deshift(sq)), color);
        }
        for sq in east_attacks.all_set_indices() {
            out.add_capture(Piece::Pawn, deshift(sq) - 1, sq);
        }
        for sq in west_attacks.all_set_indices() {
            out.add_capture(Piece::Pawn, deshift(sq) + 1, sq);
        }
        for sq in east_attack_promotions.all_set_indices() {
            out.add_promotion(deshift(sq) - 1, sq, true);
        }
        for sq in west_attack_promotions.all_set_indices() {
            out.add_promotion(deshift(sq) + 1, sq, true);
        }

        // Castling
        if ply.can_castle_short() {
            dbg!(ply);
            out.add_short_castle(color);
        }
        if ply.can_castle_long() {
            out.add_long_castle(color);
        }

        // knight moves
        let knights = ply.knights_for(color);
        for source in knights.all_set_indices() {
            let attacks = KNIGHT_MOVES[source] & !ply.occupancy_for(color) & blocking_squares;

            for capture in (attacks & ply.occupancy_for(!color)).all_set_indices() {
                out.add_capture(Piece::Knight, source, capture);
            }

            for target in (attacks & !ply.occupancy_for(!color)).all_set_indices() {
                out.add_move(Piece::Knight, source, target);
            }
        }

        for piece in [Piece::Bishop, Piece::Rook, Piece::Queen] {
            for source in ply.get_piece(color, piece).all_set_indices() {
                let attacks = pextboard::attacks_for(piece, source, ply.occupancy())
                    & !ply.occupancy_for(color) & blocking_squares;

                for capture in (attacks & ply.occupancy_for(!color)).all_set_indices() {
                    out.add_capture(piece, source, capture)
                }

                for target in (attacks & !ply.occupancy_for(!color)).all_set_indices() {
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

    use super::*;
    use crate::{assert_is_subset, bitboard::Bitboard, constants::*, movement::MoveType};

    // TODO: Have a yaml file which describes a bunch of test positions and the valid moves they entail, load them, then generate tests
    // from them, we can do this by taking random positions from a database, using stockfish to perft 1 them, then grab the results.

    #[test]
    fn calculates_starting_position_moves() {
        let ply = Ply::from_fen(&String::from(START_POSITION_FEN));
        let moves = Move::generate(&ply, Color::WHITE);
        for m in STARTING_MOVES.iter() {
            if !moves.contains(m) {
                dbg!("missing move", m);
            }
            assert!(moves.contains(m));
        }
    }

    #[test]
    fn calculates_move_after_1_d4_correctly() {
        let ply = Ply::from_fen(&String::from(D4_POSITION_FEN));
        let moves = Move::generate(&ply, Color::BLACK);
        for m in D4_MOVES.iter() {
            if !moves.contains(m) {
                dbg!("missing move", m);
            }
            assert!(moves.contains(m));
        }
    }

    #[test]
    fn calculates_correct_movecount_kiwipete_at_depth_1() {
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
            assert!(moves.moves.iter().any(|movset| movset.contains(mov)))
        }
    }

    #[test]
    fn calculated_en_passant_capture() {
        let mut ply = Ply::from_fen(START_POSITION_FEN);
        ply.make_by_notation("h2", "h3", MoveType::QUIET).unwrap();
        ply.make_by_notation("c7", "c5", MoveType::DOUBLE_PAWN)
            .unwrap();

        assert!(ply.meta.en_passant().is_some());
        assert_eq!(
            ply.meta.en_passant().unwrap(),
            Bitboard::from_index(NOTATION_TO_INDEX("c6") as u8)
        );

        ply.make_by_notation("h3", "h4", MoveType::QUIET).unwrap();
        ply.make_by_notation("c5", "c4", MoveType::QUIET).unwrap();
        ply.make_by_notation("d2", "d4", MoveType::DOUBLE_PAWN)
            .unwrap();

        assert!(ply.meta.en_passant().is_some());

        let moves = Move::generate(&ply, Color::BLACK);

        assert!(moves.contains(&Move::from_notation("c4", "d3", MoveType::EP_CAPTURE)));
    }

}
