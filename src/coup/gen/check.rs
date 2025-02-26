use crate::{coup::rep::Move, game::position::Position};

pub fn is_in_check(position: &Position) -> bool {
    position.their_reach().is_set(position.our_king())
}

pub fn losing_checkmate(position: &Position) -> bool {
    todo!();
    false
}


// Generate all valid moves which resolve the check, this is any kind move, or any intervening move
pub fn generate_moves(position: &Position) -> impl Iterator<Item = Move> {
    vec![].into_iter()
}
