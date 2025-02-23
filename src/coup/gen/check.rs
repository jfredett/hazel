use crate::{game::position::Position, types::Piece};

fn is_in_check(position: &Position) -> bool {
    position.all_attacked_squares(&position.villain()).is_set(position.our_king())
}

fn losing_checkmate(position: &Position) -> bool {
    if is_in_check(position) && position.intervention_squares().is_empty() {
        todo!();
    }
    false
}
