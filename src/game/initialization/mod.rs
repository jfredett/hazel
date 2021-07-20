use crate::{constants::START_POSITION_FEN, moveset::MoveSet};

use super::*;

mod mainline;
// mod with_variations;

impl Game {
    /// ::from_fen/1
    /// 
    /// Creates a Game initialized to the given fen string
    #[cfg(test)]
    pub fn from_fen(fen: &str) -> Game {
        Game {
            position: Ply::from_fen(fen),
            played: vec![],
            history: vec![],
            captures: vec![],
            metadata: vec![]
        }
    }

    #[cfg(not(test))]
    pub fn from_fen(fen: &str) -> Game {
        Game {
            position: Ply::from_fen(fen),
            played: vec![],
            captures: vec![],
            metadata: vec![]
        }
    }

    /// ::start_position/1
    /// 
    /// Creates a Game initialized to the start position.
    pub fn start_position() -> Game {
        Game::from_fen(START_POSITION_FEN)
    }
    
    pub fn moves(&self) -> MoveSet {
        self.position.moves()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creation {
        use crate::constants::POS2_KIWIPETE_FEN;

        use super::*;

        #[test]
        fn create_from_fen() {
            let g = Game::from_fen(POS2_KIWIPETE_FEN);
            assert_eq!(g.position.to_fen(), POS2_KIWIPETE_FEN);
        }
        
        #[test]
        fn start_position() {
            let g = Game::start_position();
            assert_eq!(g.position.to_fen(), START_POSITION_FEN);
        }
    }
    
}