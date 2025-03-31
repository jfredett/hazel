use hazel_bitboard::constants::move_tables::KNIGHT_MOVES;
use hazel_representation::coup::rep::{Move, MoveType};
use hazel_representation::game::position::Position;

pub fn generate_moves(position: &Position) -> impl Iterator<Item = Move> {
    let knights = position.our_knights().into_iter().map(|sq| (sq, KNIGHT_MOVES[sq.index()]));
    let moves = position.our_knight_moves();
    let enemies = position.enemies();


    let mut ret = vec![];

    for (source, mask) in knights {
        let attacks = moves & mask & enemies;
        let quiet = moves & mask & !enemies;

        for target_sq in attacks.into_iter() {
            ret.push(Move::new(source, target_sq, MoveType::CAPTURE));
        }

        for target_sq in quiet.into_iter() {
            ret.push(Move::new(source, target_sq, MoveType::QUIET));
        }
    }

    ret.into_iter()
}


#[cfg(test)]
mod tests {

    use hazel_basic::square::*;
    use hazel_basic::ben::BEN;

    use super::*;

    #[test]
    fn test_position() {
        let position = Position::new(BEN::new("8/8/2b1b3/1P3P2/3N4/1P3P2/8/8 w - - 0 1"));
        let moves = generate_moves(&position);

        assert_eq!(moves.collect::<Vec<Move>>(), vec![
            Move::new(D4, C6, MoveType::CAPTURE),
            Move::new(D4, E6, MoveType::CAPTURE),
            Move::new(D4, C2, MoveType::QUIET),
            Move::new(D4, E2, MoveType::QUIET)
        ]);
    }
}
