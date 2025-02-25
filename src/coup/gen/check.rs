use crate::game::position::Position;

pub fn is_in_check(position: &Position) -> bool {
    position.their_reach().is_set(position.our_king())
}

pub fn losing_checkmate(position: &Position) -> bool {
    todo!();
    false
}
