use crate::game::chess::position::Position;
use crate::coup::rep::Move;


mod pawn;
mod check;
mod slider;
// mod bishop;
mod knight;
// mod rook;
// mod queen;
mod king;

struct MoveGenerator {
    // This should actually just be passed into the generate_moves, and MoveGen is just for holding
    // caches.
    // TODO: Cache anything worth caching?
}

impl MoveGenerator {
    pub fn new(position: Position) -> Self {
        Self { }
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        let mut moves = Vec::new();

        // TODO: Check cache for this position

        // TODO: Paralellize?
        // moves.append(&mut { pawn::generate_moves(position) });
        // moves.append(&mut { bishop::generate_moves(position) });
        // moves.append(&mut { knight::generate_moves(position) });
        // moves.append(&mut { rook::generate_moves(position) });
        // moves.append(&mut { queen::generate_moves(position) });
        // moves.append(&mut { king::generate_moves(position) });
        moves
    }
}
