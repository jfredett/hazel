use crate::game::chess::position::Position;
use crate::Query;
use crate::types::Occupant;
use crate::coup::rep::{Move, MoveType};
use crate::notation::*;
use crate::types::color::Color;

/*
*
* something like a 'MoveQuery' object which takes a position and a square, and returns a classified
* grouping of possible moves
*/


// TODO: These are all probably pre-calculateable.
pub fn double_pawn_moves(position: &Position, color: Color) -> impl Iterator<Item = Move> {
    let bb = position.pawns_for(&color) & color.pawn_mask();
    let blockers = position.all_blockers();
    let first_advance = bb.shift(color.pawn_direction()) & !blockers; // advance all pawns by 1, mask off anyone who runs into a blocker
    let second_advance = first_advance.shift(color.pawn_direction()) & !blockers; // advance again, masking out blockers
    second_advance.into_iter().map(move |target_sq| {
        let source_sq = target_sq.set_rank(color.pawn_rank());
        Move::new(source_sq, target_sq, MoveType::DOUBLE_PAWN)
    })
}

// pub fn quiet_pawn_moves(position: &Position, color: Color) -> impl Iterator<Item = Move> {
//     vec![Move::empty()].into_iter()
// }

// pub fn pawn_attacks(position: &Position, color: Color) -> impl Iterator<Item = Move> {
//     vec![Move::empty()].into_iter()
// }



// // TODO: Return an iterator?
// pub fn generate_moves(position: &Position, color: Color) -> impl Iterator<Item = Move> {
//     double_pawn_moves(position, color).chain(
//     quiet_pawn_moves(position, color)).chain(
//     pawn_attacks(position, color))
// }


#[cfg(test)]
mod tests {
    use ben::BEN;

    use super::*;

    #[macro_export]
    macro_rules! assert_finds_moves {
        ($func_name:ident, $fen:expr, count = $expected_count:expr) => {
            assert_finds_moves!($func_name, $fen, count = $expected_count, []);
        };
        ($func_name:ident, $fen:expr, count = $expected_count:expr, [ $($move:expr),* ]) => {
            let mut position = Position::new(
                BEN::new($fen),
                vec![]
            );
            let moves : Vec<Move> = $func_name(&position, Color::WHITE).collect();

            tracing::debug!("{:?}", moves);

            assert_eq!(moves.len(), $expected_count);

            for expected_move in [ $($move),* ] {
                assert!(moves.contains(&expected_move));
            }
        };
    }

    mod double_pawn {
        use crate::coup::rep::MoveType;

        use super::*;

        #[test]
        #[tracing_test::traced_test]
        fn double_pawn_push() {
            assert_finds_moves!(
                double_pawn_moves,
                "8/8/8/8/8/8/3P4/8 w KQkq - 0 1",
                count = 1,
                [ Move::new(D2, D4, MoveType::DOUBLE_PAWN) ]
            );
        }

        #[test]
        fn finds_multiple_doublepushes() {
            assert_finds_moves!(
                double_pawn_moves,
                 "8/8/8/8/8/8/3P1P2/8 w KQkq - 0 1",
                count = 2,
                [ Move::new(D2, D4, MoveType::DOUBLE_PAWN), Move::new(F2, F4, MoveType::DOUBLE_PAWN) ]
            );
        }

        #[test]
        fn does_not_find_push_in_illegal_position() {
            assert_finds_moves!(
                double_pawn_moves,
                "8/8/8/8/8/P7/8/8 w KQkq - 0 1",
                count = 0
            );
        }

    }

}
