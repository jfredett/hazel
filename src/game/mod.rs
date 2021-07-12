use crate::{constants::Piece, movement::Move, ply::Ply};
use serde::{Serialize, Deserialize};

// A game monitors the state of a ply, maintains a list of moves that have been made, and can do/undo moves as needed.
// Moves are stored in a stack.


#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Game {
    position: Ply,
    moves: Vec<Move>,     // TODO: Maybe a 'finite stack' class would be better here? 
    captures: Vec<Piece>, //       ditto
    metadata: Vec<String> // TODO: String -> Some custom 'tag' type

}

impl Game {
    // ::from_pgn/1
    // ::from_fen/1
    // ::start_position/1
    // #make/1
    // #unmake/0
    // #evaluate/0
}