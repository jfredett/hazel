use crate::constants::move_tables::PAWN_MOVES;
use crate::engine::driver::Position;
use crate::Query;
use crate::types::{Occupant, Piece};
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

// const PAWN_MOVES: [[PossibleMove; 4]; 48] = {
//     for sq in A2..=G8 {
//     }
//     [[PossibleMove::Impossible; 4]; 64]
// }

pub fn generate_moves(position: &Position, color: Color) -> Vec<PossibleMove> {
    let mut ret = vec![];
    for source_sq in position.find_all(Piece::Pawn) {
        let bb = PAWN_MOVES[source_sq.into()][color.into()];
        for target_sq in bb {
            if let Occupant::Occupied(..) = position.get(target_sq) {
                ret.push(PossibleMove::Possible(source_sq, target_sq));
            } else {
                // if it's an attack move, then it's not possible, if it's an advance move, then
                // it's valid.
                // if it's a double-push, we need to verify the 
            }
        }
    }

    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_pawn_push() {
        let mut position = Position::from_fen("8/8/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1").unwrap();
        let moves = generate_moves(&position, Color::White);
        assert_eq!(moves.len(), 16);

    }
}
