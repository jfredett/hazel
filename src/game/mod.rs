use crate::{constants::{Color, Piece}, movement::Move, ply::Ply};
use serde::{Serialize, Deserialize};

mod initialization;
mod arbitrary;
mod perft;

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
        if self.moves.is_empty() { return }

        let mov = self.moves.pop().unwrap();

        let captured_piece = if mov.is_capture() {
            self.captures.pop()
        } else {
            None
        };

        self.position.unmake(mov, captured_piece)
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


#[cfg(test)]
mod tests {
    use either::Either;
    use crate::movement::MoveType;
    use crate::constants::START_POSITION_FEN;

    use super::*;

    #[test]
    fn make_and_unmake_are_inverses() {
        let mut game = Game::start_position();
        let original = game.clone();
        let mov = Move::from_notation("d2", "d4", Either::Left(MoveType::quiet()));
        
        game.make(mov);
        assert_ne!(game, original);
            
        game.unmake();
        assert_eq!(game, original);
    }
}