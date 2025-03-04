use crate::{coup::rep::Move, game::position::Position};

pub fn is_in_check(position: &Position) -> bool {
    position.their_reach().is_set(position.our_king())
}

pub fn losing_checkmate(_position: &Position) -> bool {
    todo!()
}


// Generate all valid moves which resolve the check, this is any kind move, or any intervening move
pub fn generate_moves(_position: &Position) -> impl Iterator<Item = Move> {
    vec![].into_iter()
}
