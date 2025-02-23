
use crate::{coup::rep::{Move, MoveType}, game::position::Position, notation::Square, types::{pextboard, Bitboard, Direction, Occupant, Piece}};

mod bishop {
    use super::*;
    pub fn generate_moves(position: &Position) -> impl Iterator<Item = Move> {
        generate_slider_moves(position, Piece::Bishop)
    }
}

mod rook {
    use super::*;
    pub fn generate_moves(position: &Position) -> impl Iterator<Item = Move> {
        generate_slider_moves(position, Piece::Rook)
    }
}

mod queen {
    use super::*;
    pub fn generate_moves(position: &Position) -> impl Iterator<Item = Move> {
        generate_slider_moves(position, Piece::Queen)
    }
}

fn generate_slider_moves(position: &Position, piece: Piece) -> impl Iterator<Item = Move> {
    let pieces = position.find(|(sq, occ)| *occ == Occupant::Occupied(piece, position.hero()));
    let blockers = position.all_blockers();
    let enemies = position.enemies();
    let friendlies = position.friendlies();

    pieces.into_iter().flat_map(move |source_sq| {
        let moves = pextboard::attacks_for(piece, source_sq, blockers) & !friendlies;
        (moves & !enemies).into_iter().map(move |target_sq| Move::new(source_sq, target_sq, MoveType::QUIET)).chain(
        (moves & enemies).into_iter().map(move |target_sq| Move::new(source_sq, target_sq, MoveType::CAPTURE)))
    })
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use ben::BEN;

    use crate::notation::*;

    use super::*;

    #[test]
    fn bishop_test_position() {
        let position = Position::new(BEN::new("8/8/1p3p2/8/3B4/2P1P3/8/8 w - - 0 1"), vec![]);
        let moves = bishop::generate_moves(&position);
        let mut expected = vec![
            Move::new(D4, C5, MoveType::QUIET),
            Move::new(D4, E5, MoveType::QUIET),
            Move::new(D4, B6, MoveType::CAPTURE),
            Move::new(D4, F6, MoveType::CAPTURE)
        ];
        expected.sort();

        similar_asserts::assert_eq!(moves.sorted().collect::<Vec<Move>>(),expected)
    }

    #[test]
    fn queen_test_position() {
        let position = Position::new(BEN::new("8/3p4/1p3p2/8/2PQP3/2PPP3/8/8 w - - 0 1"), vec![]);
        let moves = queen::generate_moves(&position);
        let mut expected =  vec![
            Move::new(D4, C5, MoveType::QUIET),
            Move::new(D4, E5, MoveType::QUIET),
            Move::new(D4, D5, MoveType::QUIET),
            Move::new(D4, D6, MoveType::QUIET),
            Move::new(D4, D7, MoveType::CAPTURE),
            Move::new(D4, B6, MoveType::CAPTURE),
            Move::new(D4, F6, MoveType::CAPTURE)
        ];
        expected.sort();

        similar_asserts::assert_eq!(moves.sorted().collect::<Vec<Move>>(), expected);
    }

    #[test]
    fn rook_test_position() {
        let position = Position::new(BEN::new("8/8/3p4/8/3R1p2/3P4/8/3P4 w - - 0 1"), vec![]);
        let moves = rook::generate_moves(&position);
        let mut expected = vec![
            Move::new(D4, C4, MoveType::QUIET),
            Move::new(D4, B4, MoveType::QUIET),
            Move::new(D4, A4, MoveType::QUIET),
            Move::new(D4, E4, MoveType::QUIET),
            Move::new(D4, D5, MoveType::QUIET),
            Move::new(D4, F4, MoveType::CAPTURE),
            Move::new(D4, D6, MoveType::CAPTURE)
        ];
        expected.sort();

        similar_asserts::assert_eq!(moves.sorted().collect::<Vec<Move>>(), expected);
    }
}
