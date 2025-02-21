use crate::game::chess::position::Position;
use crate::Query;
use crate::types::{Direction, Occupant};
use crate::coup::rep::{Move, MoveType};
use crate::notation::*;
use crate::types::color::Color;

/*
*
* something like a 'MoveQuery' object which takes a position and a square, and returns a classified
* grouping of possible moves
*/


/// Finds all double-pawn pushes.
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

/// Finds all "normal" pawn moves, does not find promotions or double moves
pub fn quiet_pawn_moves(position: &Position, color: Color) -> impl Iterator<Item = Move> {
    let bb = position.pawns_for(&color) & !color.promotion_mask();
    let blockers = position.all_blockers();
    let advance = bb.shift(color.pawn_direction()) & !blockers;
    advance.into_iter().map(move |target_sq| {
        let source_sq = target_sq.backward(&color).expect("This should be impossible, good luck finding the bug, future me.");
        Move::new(source_sq, target_sq, MoveType::QUIET)
    })
}

/// Finds all "normal" pawn attacks, does not find promotion captures
pub fn pawn_attacks(position: &Position, color: Color) -> impl Iterator<Item = Move> {
    let bb = position.pawns_for(&color) & !color.promotion_mask();
    let enemies = position.all_pieces_of(&!color);
    let east_attacks = bb.shift(color.pawn_direction()).shift(Direction::E) & enemies;
    let west_attacks = bb.shift(color.pawn_direction()).shift(Direction::W) & enemies;

    // FIXME: this might could be better, IDK. The unwraps should never fail since we slid things
    // to get there, and we're just unsliding, but I don't love this implementation
    east_attacks.into_iter().map(move |target_sq| {
        let source_sq = target_sq.shift((!color).pawn_direction()).unwrap().shift(Direction::W).unwrap();
        Move::new(source_sq, target_sq, MoveType::CAPTURE)
    }).chain(west_attacks.into_iter().map(move |target_sq| {
        let source_sq = target_sq.shift((!color).pawn_direction()).unwrap().shift(Direction::E).unwrap();
        Move::new(source_sq, target_sq, MoveType::CAPTURE)
    }))
}


// en_passant?


// // TODO: Return an iterator?
// pub fn generate_moves(position: &Position, color: Color) -> impl Iterator<Item = Move> {
//     double_pawn_moves(position, color).chain(
//     quiet_pawn_moves(position, color)).chain(
//     pawn_attacks(position, color))
// }


#[cfg(test)]
mod tests {
    use ben::BEN;
    use crate::coup::rep::MoveType;

    use super::*;

    #[macro_export]
    macro_rules! assert_finds_moves {
        ($func_name:ident, $fen:expr) => {
            assert_finds_moves!($func_name, $fen, color = Color::WHITE, []);
        };
        ($func_name:ident, $fen:expr, [ $($move:expr),* ]) => {
            assert_finds_moves!($func_name, $fen, color = Color::WHITE, [ $($move),* ]);
        };
        ($func_name:ident, $fen:expr, color = $color:expr, [ $($move:expr),* ]) => {
            let mut position = Position::new(
                BEN::new($fen),
                vec![]
            );
            let mut moves : Vec<Move> = $func_name(&position, $color).collect();
            let mut expected_moves : Vec<Move> = vec![$($move),*];

            similar_asserts::assert_eq!(moves.sort(), expected_moves.sort());
        };
    }

    mod pawn_attacks {
        use super::*;


        #[test]
        fn pawn_attacks_combine() {
            assert_finds_moves!(
                pawn_attacks,
                "8/8/8/8/8/2p1p3/3P4/8 w KQkq - 0 1",
                [ Move::new(D2, C3, MoveType::CAPTURE),
                  Move::new(D2, E3, MoveType::CAPTURE)
                ]
            );
        }

        #[test]
        fn pawn_attacks_west() {
            assert_finds_moves!(
                pawn_attacks,
                "8/8/8/8/8/2p6/3P4/8 w KQkq - 0 1",
                [ Move::new(D2, C3, MoveType::CAPTURE) ]
            );
        }

        #[test]
        fn pawn_attacks_east() {
            assert_finds_moves!(
                pawn_attacks,
                "8/8/8/8/8/4p3/3P4/8 w KQkq - 0 1",
                [ Move::new(D2, E3, MoveType::CAPTURE) ]
            );
        }

    }

    mod quiet_pawn {
        use super::*;

        #[test]
        fn single_pawn_push() {
            assert_finds_moves!(
                quiet_pawn_moves,
                "8/8/8/8/8/8/3P4/8 w KQkq - 0 1",
                [ Move::new(D2, D3, MoveType::QUIET) ]
            );
        }

        #[test]
        fn does_not_capture_promotion_pushes() {
            assert_finds_moves!(
                quiet_pawn_moves,
                "8/8/8/8/8/8/3p4/8 b KQkq - 0 1",
                color = Color::BLACK,
                [ ]
            );

        }

    }

    mod double_pawn {

        use super::*;

        #[test]
        fn double_pawn_push() {
            assert_finds_moves!(
                double_pawn_moves,
                "8/8/8/8/8/8/3P4/8 w KQkq - 0 1",
                [ Move::new(D2, D4, MoveType::DOUBLE_PAWN) ]
            );
        }

        #[test]
        fn finds_multiple_doublepushes() {
            assert_finds_moves!(
                double_pawn_moves,
                 "8/8/8/8/8/8/3P1P2/8 w KQkq - 0 1",
                [ Move::new(D2, D4, MoveType::DOUBLE_PAWN), Move::new(F2, F4, MoveType::DOUBLE_PAWN) ]
            );
        }

        #[test]
        fn does_not_find_push_in_illegal_position() {
            assert_finds_moves!(
                double_pawn_moves,
                "8/8/8/8/8/P7/8/8 w KQkq - 0 1",
                [ ]
            );
        }

    }

}
