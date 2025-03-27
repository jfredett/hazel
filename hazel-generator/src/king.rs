use hazel::{coup::rep::{Move, MoveType}, game::position::Position};

pub fn generate_moves(position: &Position) -> impl Iterator<Item = Move> {
    // assumes we aren't in check, captures assume piece is not protected.
    let source_sq = position.our_king();
    let king_attacks = position.our_king_attacks() & !position.their_reach() & !position.friendlies(); // this should really check for defense of the other pieces?
    let king_quiet = position.our_king_moves() & !position.their_reach() & !king_attacks;


    king_attacks.into_iter().map(move |target_sq| Move::new(source_sq, target_sq, MoveType::CAPTURE)).chain(
        king_quiet.into_iter().map(move |target_sq| Move::new(source_sq, target_sq, MoveType::QUIET)))
}


#[cfg(test)]
mod tests {
    use ben::BEN;

    use hazel::{coup::rep::{Move, MoveType}, notation::*};
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_position() {
        let position = Position::new(BEN::new("3k1b2/8/8/2p1P3/3K4/2p1P3/8/8 w - - 0 1"));
        let moves = generate_moves(&position);
        similar_asserts::assert_eq!(moves.collect::<Vec<Move>>(), vec![
            Move::new(D4, C3, MoveType::CAPTURE),
            Move::new(D4, D3, MoveType::QUIET),
            Move::new(D4, C4, MoveType::QUIET),
            Move::new(D4, E4, MoveType::QUIET),
            Move::new(D4, D5, MoveType::QUIET)
        ]);
    }
}
