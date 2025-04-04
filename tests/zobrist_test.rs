use hazel_core::zobrist::HazelZobrist;
use hazel_representation::{coup::rep::{Move, MoveType}, game::position::Position};
use hazel_core::{ben::BEN, zobrist::Zobrist};
use hazel_core::{color::Color, piece::Piece};
use hazel_core::zobrist::ZOBRIST_TABLE_SIZE;
use hazel_core::square::*;
use quickcheck_macros::quickcheck;

mod zobrist_table {



    use super::*;

    #[test]
    fn zobrist_table_is_not_all_zeros() {
        let table = HazelZobrist::TABLE;
        for e in table {
            assert_ne!(e, 0);
        }
    }

    #[test]
    fn zobrist_table_is_all_distinct() {
        let table = HazelZobrist::TABLE;
        for i in 0..ZOBRIST_TABLE_SIZE {
            for j in 0..i {
                if table[i] == table[j] {
                    panic!("TABLE[{}] == TABLE[{}]` == {}", i, j, table[i]);
                }
            }
        }
    }

    #[quickcheck]
    fn depth_for_works(sq: Square, color: Color, piece: Piece) -> bool {
        let depth = HazelZobrist::depth_for(sq, color, piece) as usize;
        // NOTE: Depth only maps from [0, (ZTS-1)], the `ZTS`th spot is for the side-to-move
        // mask
        depth < (ZOBRIST_TABLE_SIZE - 1)
    }

    #[quickcheck]
    fn slow_mask_is_the_same_as_fast_mask(sq: Square, color: Color, piece: Piece) -> bool {
        let slow = HazelZobrist::slow_zobrist_mask_for(sq, color, piece);
        let fast = HazelZobrist::zobrist_mask_for(sq, color, piece);
        if slow != fast {
            dbg!(slow, fast);
            false
        } else { true }
    }
}

mod zobrist {

    use hazel_core::interface::{Alter, Alteration};

    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn zobrist_is_nonzero() {
        let p = Position::new(BEN::start_position());
        assert_ne!(p.zobrist().position, Zobrist::empty());
        assert_ne!(p.zobrist().current, Zobrist::empty());
    }

    #[test]
    fn zobrist_is_different_after_a_move_is_made() {
        let p1 = Position::new(BEN::start_position());
        let p2 = Position::with_moves(BEN::start_position(), vec![Move::new(D2, D4, MoveType::QUIET)]);
        assert_ne!(p1.zobrist(), p2.zobrist());
    }

    #[test]
    fn zobrist_is_same_for_same_position() {
        let p1 = Position::new(BEN::start_position());
        let p2 = Position::new(BEN::start_position());

        assert_eq!(p1.zobrist(), p2.zobrist());
    }

    #[test]
    #[tracing_test::traced_test]
    fn zobrist_is_same_for_transposition() {
        let variation_1 = vec![
            Move::new(D2, D4, MoveType::QUIET),
            Move::new(D7, D5, MoveType::QUIET),
            Move::new(C1, F4, MoveType::QUIET),
            Move::new(G8, F6, MoveType::QUIET),
            Move::new(E2, E3, MoveType::QUIET),
        ];
        let variation_2 = vec![
            Move::new(D2, D4, MoveType::QUIET),
            Move::new(G8, F6, MoveType::QUIET),
            Move::new(C1, F4, MoveType::QUIET),
            Move::new(D7, D5, MoveType::QUIET),
            Move::new(E2, E3, MoveType::QUIET),
        ];
        let p1 = Position::with_moves(BEN::start_position(), variation_1);
        let p2 = Position::with_moves(BEN::start_position(), variation_2);

        assert_eq!(p1.zobrist(), p2.zobrist());
    }

    // This checks if z == z ^ a1 ^ a1
    #[quickcheck]
    fn zobrist_update_is_idempotent_if_not_clear(zobrist: Zobrist, alteration: Alteration) -> bool {
        // Clear resets the zobrist to 0, so it needs to be skipped for our test, it is the only
        // exception to the idempotence rule.
        if alteration == Alteration::Clear { return true; }

        zobrist.alter(alteration).alter(alteration) == zobrist

    }
}
