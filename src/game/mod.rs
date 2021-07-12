use crate::{constants::{Color, Piece}, movement::Move, ply::Ply};
use serde::{Serialize, Deserialize};

mod initialization;

#[derive(PartialEq, Eq, Clone, Hash, Debug, Serialize, Deserialize)]
pub struct Game {
    position: Ply,
    moves: Vec<Move>,               // TODO: Maybe a 'finite stack' class would be better here? 
    captures: Vec<(Color, Piece)>,  //       ditto
    metadata: Vec<(String, String)> // TODO: String -> Some custom 'tag' type
}

impl Game {
    // #make/1              --> proxies down to Ply
    pub fn make(&mut self, mov: Move) {
        self.position.make(mov);
        self.moves.push(mov);
        if mov.is_capture() {
            // add captured piece
            if let Some(p) = self.position.piece_at_index(mov.target_idx().into()) {
                self.captures.push(p)
            }
        }
    }

    // #unmake/0            --> proxies down to Ply
    pub fn unmake(&mut self) {
        todo!();        
    }

    // #evaluate/0          --> should probably proxy down to a method on Ply
    // 

    /// current_player/0
    ///
    /// The Color of the current player
    pub fn current_player(&self) -> Color {
        self.position.current_player()
    }
}