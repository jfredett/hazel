use crate::{constants::{Color, Piece}, movement::Move, ply::Ply};
use crate::ply::metadata::Metadata;
use serde::{Serialize, Deserialize};

mod initialization;
mod arbitrary;
mod perft;
mod debug;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
enum History {
    Make(Move),
    Unmake(Move)
}

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Game {
    position: Ply,
    played: Vec<Move>,              // TODO: Maybe a 'finite stack' class would be better here? 
    #[cfg(test)] history: Vec<History>, // Temporary, add only history for debugging purposes -- would be cool to featureflag this so we can turn it on when needed
    captures: Vec<Piece>,  //       ditto.
    game_history: Vec<Metadata>,
    metadata: Vec<(String, String)> // TODO: String -> Some custom 'tag' type
    // NOTE: Do captures need to record color? We have the move recorded, so the color could be deduced.
}

impl Game {
    // #make/1              --> proxies down to Ply
    pub fn make(&mut self, mov: Move) {
        // NOTE: It is important to do this _before making the move_ so that we add the correct piece to the capture stack.
        self.played.push(mov);

        #[cfg(test)] self.history.push(History::Make(mov));
        self.game_history.push(self.position.meta);

        if let Some(p) = self.position.make(mov).unwrap() {
            self.captures.push(p)
        }
    }

    // #unmake/0            --> proxies down to Ply
    pub fn unmake(&mut self) {
        if self.played.is_empty() { return }

        let mov = self.played.pop().unwrap();

        #[cfg(test)] self.history.push(History::Unmake(mov));

        let captured_piece = if mov.is_capture() {
            self.captures.pop()
        } else {
            None
        };
        let metadata = self.game_history.pop().unwrap();

        if let Err(e) = self.position.unmake(mov, captured_piece, metadata) {
            panic!("error: {:?}, game: {:?}", e, self)
        }
    }

    // #evaluate/0          --> should probably proxy down to a method on Ply
    // 

    /// current_player/0
    ///
    /// The Color of the current player
    pub fn current_player(&self) -> Color {
        self.position.current_player()
    }
    
    /// other_player/0
    ///
    /// The Color of the other player
    pub fn other_player(&self) -> Color {
        self.position.other_player()
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
        let mov = Move::from_notation("d2", "d4", MoveType::QUIET);
        
        game.make(mov);
        // because we track the full history in the struct, we have to compare it 
        // in chunks
        assert_ne!(game.position, original.position);
            
        game.unmake();

        assert_eq!(game.position, original.position);
    }
}