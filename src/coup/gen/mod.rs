use crate::engine::driver::Position;
use crate::coup::rep::Move;


mod pawn;
mod bishop;
mod knight;
mod rook;
mod queen;
mod king;


// What I think I need is an engine, ideally asynchronous, that takes a position specified w/ all
// relevant metadata (a BEN, maybe `position` should be the `impl Into` target a la `move` or
// `square`?) and then allows you to query it via message passing. It should then support all the
// verbs from the old generator:
//
// - `find(piece, color)`: Returns an iterator of all squares containing the specified piece and color (shows up as `king_for` in the old generator)
// - `attacks(piece, sq)`: Returns an iterator of all squares attacked by the specified piece from the specified square
// - `occupancy(color)`: Returns an iterator of all squares occupied by the specified color
// - `empty()`: Returns an iterator of all empty squares
// - `occupied()`: Returns an iterator of all occupied squares
// - `in_check(color)`: Returns whether the specified color is in check
// - `demense(color)`: An iterator of all the squares attacked by any piece of the specified color
// - `threats(sq)` : An iterator of all the squares attacking the specified square
//
// at a low level, as well:
//
// - `validate(Move)`: Returns whether the specified move is valid (i.e., legal)
//
//
// and so on, these, internally, would call other messages and eventually return the desired result
// to an output stream. Ideally it would allow for multiple, independent output streams somehow.
// This engine could be extended arbitrarily to support new messages; and for other board
// representations, it could be implemented differently, eventually allowing for a single interface
// to query boardstate regardless of the underlying representation.
//
// Ideally this will allow the little engine actually doing all the calculations to pro-actively
// arrange and reduce work via caching or other optimizations.
//


struct MoveGenerator {
    position: Position,
    // TODO: Cache anything worth caching?
}

impl MoveGenerator {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        // Check fr Early Exit conditions, e.g., check, etc.


        // TODO: Paralellize?
        moves.append(&mut { pawn::generate_moves(&self.position) });
        moves.append(&mut { bishop::generate_moves(&self.position) });
        moves.append(&mut { knight::generate_moves(&self.position) });
        moves.append(&mut { rook::generate_moves(&self.position) });
        moves.append(&mut { queen::generate_moves(&self.position) });
        moves.append(&mut { king::generate_moves(&self.position) });
        moves
    }
}
