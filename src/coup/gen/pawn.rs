use crate::constants::move_tables::PAWN_MOVES;
use crate::game::chess::position::Position;
use crate::Query;
use crate::types::{Bitboard, Occupant, Piece};
use crate::coup::rep::Move;
use crate::notation::*;
use crate::types::color::Color;

// this gets projected down to an Option<Move> in some context
enum PossibleMove {
    Impossible,
    Possible(Square, Square)
}

impl PossibleMove {
    pub fn reduce(&self, c: &impl Query) -> Option<Move> {
        todo!();
    }
}

impl Query for Position {
    fn get(&self, square: impl Into<Square>) -> Occupant {
        todo!()
    }
}

pub fn generate_moves(position: &Position, color: Color) -> Vec<PossibleMove> {
    let mut ret = vec![];
    for source_sq in position.find(&Occupant::Occupied(Piece::Pawn, color)) {
        // OQ: This might be faster to return a list of 4 option<square>s?
        // let bb : Bitboard = PAWN_MOVES[source_sq.into()][color.into()];
        // for target_sq in bb {
        //     if position.is_occupied(target_sq) {
        //         ret.push(PossibleMove::Possible(source_sq, target_sq));
        //     } else {
        //         // if it's an attack move, then it's not possible, if it's an advance move, then
        //         // it's valid.
        //         // if it's a double-push, we need to verify that we are on the correct rank, and
        //         // that we aren't blocked on both squares.
        //     }
        // }
    }

    ret
}


#[cfg(test)]
mod tests {
    use ben::BEN;

    use super::*;

//     #[test]
    fn double_pawn_push() {
        let mut position = Position::new(
            BEN::new("8/8/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1"),
            vec![]
        );
        let moves = generate_moves(&position, Color::WHITE);
        assert_eq!(moves.len(), 16);
    }
}
