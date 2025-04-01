
use hazel_representation::coup::rep::{Move, MoveType};
use hazel_representation::game::chess::position::Position;

use hazel_core::direction::Direction;
use hazel_core::interface::Query;
use hazel_core::occupant::Occupant;
use hazel_core::piece::Piece;
use hazel_core::square::Square;
use hazel_bitboard::ColorMasks;


/// Finds all double-pawn pushes.
pub fn double_pawn_moves(position: &Position) -> impl Iterator<Item = Move> {
    let color = position.hero();
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
pub fn quiet_pawn_moves(position: &Position) -> impl Iterator<Item = Move> {
    let color = position.hero();
    let bb = position.pawns_for(&color) & !color.promotion_mask();
    let blockers = position.all_blockers();
    let advance = bb.shift(color.pawn_direction()) & !blockers;
    advance.into_iter().map(move |target_sq| {
        let source_sq = target_sq.backward(&color).expect("This should be impossible, good luck finding the bug, future me.");
        Move::new(source_sq, target_sq, MoveType::QUIET)
    })
}

/// Finds all "normal" pawn attacks, does not find promotion captures
pub fn pawn_attacks(position: &Position) -> impl Iterator<Item = Move> {
    let color = position.hero();
    let bb = position.pawns_for(&color) & !color.promotion_mask();
    let enemies = position.all_pieces_of(&!color);
    let advance = bb.shift(color.pawn_direction());
    let east_attacks = advance.shift(Direction::E) & enemies;
    let west_attacks = advance.shift(Direction::W) & enemies;

    east_attacks.into_iter().map(move |target_sq| {
        let source_sq = target_sq.shift((!color).pawn_direction()).unwrap().shift(Direction::W).unwrap();
        Move::new(source_sq, target_sq, MoveType::CAPTURE)
    }).chain(west_attacks.into_iter().map(move |target_sq| {
        let source_sq = target_sq.shift((!color).pawn_direction()).unwrap().shift(Direction::E).unwrap();
        Move::new(source_sq, target_sq, MoveType::CAPTURE)
    }))
}

pub fn en_passant(position: &Position) -> impl Iterator<Item = Move> {
    // TODO: is this just `self.our_pawn_attacks() & bitboard!(ep_square)`?

    let mut ret = vec![];
    if let Some(ep_file) = position.metadata().en_passant {
        let ep_square = Square::from((position.hero().en_passant_rank(), ep_file));
        let color = position.hero();
        if let Some(sq) = ep_square.left_oblique(&!color) {
            if position.get(sq) == Occupant::Occupied(Piece::Pawn, color) {
               ret.push(Move::new(sq, ep_square, MoveType::EP_CAPTURE));
            }
        }

        if let Some(sq) = ep_square.right_oblique(&!color) {
            if position.get(sq) == Occupant::Occupied(Piece::Pawn, color) {
               ret.push(Move::new(sq, ep_square, MoveType::EP_CAPTURE));
            }
        }
    }

    ret.into_iter()
}


pub fn promotions(position: &Position) -> impl Iterator<Item = Move> {
    let color = position.hero();
    let pawns = position.pawns_for(&color) & color.promotion_mask();
    let pawns = pawns.shift(color.pawn_direction()) & !position.all_blockers();

    const PROMOTION_OPTIONS : [MoveType; 4] = [
        MoveType::PROMOTION_ROOK,
        MoveType::PROMOTION_QUEEN,
        MoveType::PROMOTION_KNIGHT,
        MoveType::PROMOTION_BISHOP
    ];


    pawns.into_iter().flat_map(move |target_sq| {
        let source_sq = target_sq.shift((!color).pawn_direction()).unwrap();
        PROMOTION_OPTIONS.map(|opt|
            Move::new(source_sq, target_sq, opt)
        )
    })
}

pub fn promotion_captures(position: &Position) -> impl Iterator<Item = Move> {
    let color = position.hero();
    let pawns = position.pawns_for(&color) & color.promotion_mask();
    let enemies = position.all_pieces_of(&!color);
    let advance = pawns.shift(color.pawn_direction());
    let east_attacks = advance.shift(Direction::E) & enemies;
    let west_attacks = advance.shift(Direction::W) & enemies;

    const PROMOTION_OPTIONS : [MoveType; 4] = [
        MoveType::PROMOTION_CAPTURE_ROOK,
        MoveType::PROMOTION_CAPTURE_QUEEN,
        MoveType::PROMOTION_CAPTURE_KNIGHT,
        MoveType::PROMOTION_CAPTURE_BISHOP
    ];

    east_attacks.into_iter().flat_map(move |target_sq| {
        let source_sq = target_sq.shift((!color).pawn_direction()).unwrap().shift(Direction::W).unwrap();
        PROMOTION_OPTIONS.map(|opt|
            Move::new(source_sq, target_sq, opt)
        )
    }).chain(west_attacks.into_iter().flat_map(move |target_sq| {
        let source_sq = target_sq.shift((!color).pawn_direction()).unwrap().shift(Direction::E).unwrap();
        PROMOTION_OPTIONS.map(|opt|
            Move::new(source_sq, target_sq, opt)
        )
    }))
}

pub fn generate_moves(position: &Position) -> impl Iterator<Item = Move> {
    double_pawn_moves(position).chain(
    quiet_pawn_moves(position)).chain(
    pawn_attacks(position)).chain(
    promotions(position)).chain(
    promotion_captures(position)).chain(
    en_passant(position))
}


#[cfg(test)]
mod tests {
    use hazel_core::square::*;
    use hazel_core::ben::BEN;
    use hazel_representation::coup::rep::MoveType;

    use super::*;

    #[macro_export]
    macro_rules! assert_finds_moves {
        ($func_name:ident, $fen:expr, [ $($move:expr),* ]) => {
            let position = Position::new(BEN::new($fen));
            let mut moves : Vec<Move> = $func_name(&position).collect();
            let mut expected_moves : Vec<Move> = vec![$($move),*];

            moves.sort();
            expected_moves.sort();

            similar_asserts::assert_eq!(moves, expected_moves);
        };
    }

    mod promotion_captures {
        use super::*;


        #[test]
        fn finds_promotions() {
            assert_finds_moves!(
                promotion_captures,
                "p1p5/1P6/8/8/8/8/8/8 w KQkq d3 0 1",
                [ Move::new(B7, A8, MoveType::PROMOTION_CAPTURE_KNIGHT) , Move::new(B7, C8, MoveType::PROMOTION_CAPTURE_KNIGHT),
                  Move::new(B7, A8, MoveType::PROMOTION_CAPTURE_ROOK)   , Move::new(B7, C8, MoveType::PROMOTION_CAPTURE_ROOK),
                  Move::new(B7, A8, MoveType::PROMOTION_CAPTURE_BISHOP) , Move::new(B7, C8, MoveType::PROMOTION_CAPTURE_BISHOP),
                  Move::new(B7, A8, MoveType::PROMOTION_CAPTURE_QUEEN)  , Move::new(B7, C8, MoveType::PROMOTION_CAPTURE_QUEEN) 
                ]
            );
        }
    }


    mod promotions {
        use super::*;

        #[test]
        fn finds_promotions() {
            assert_finds_moves!(
                promotions,
                "8/P7/8/8/8/8/8/8 w KQkq d3 0 1",
                [ Move::new(A7, A8, MoveType::PROMOTION_KNIGHT),
                  Move::new(A7, A8, MoveType::PROMOTION_ROOK),
                  Move::new(A7, A8, MoveType::PROMOTION_BISHOP),
                  Move::new(A7, A8, MoveType::PROMOTION_QUEEN) ]
            );
        }
    }

    mod en_passant {
        use super::*;


        #[test]
        #[tracing_test::traced_test]
        fn finds_en_passant() {
            assert_finds_moves!(
                en_passant,
                "rnbqkbnr/pp1p1ppp/8/8/2pPp3/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1",
                [ Move::new(C4, D3, MoveType::EP_CAPTURE),
                  Move::new(E4, D3, MoveType::EP_CAPTURE)
                ]
            );
        }

        #[test]
        fn finds_single_en_passant() {
            assert_finds_moves!(
                en_passant,
                "rnbqkbnr/pp1p1ppp/8/8/3Pp3/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1",
                [ Move::new(E4, D3, MoveType::EP_CAPTURE) ]
            );
        }


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
                "8/8/8/8/8/2p5/3P4/8 w KQkq - 0 1",
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
