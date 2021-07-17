use pgn_reader::{CastlingSide, San, Visitor};


use crate::{constants::Piece, moveset::Search};

use super::*;

mod from;

struct MainlineVisitor {
    game: Game
}

impl Game {
    /// ::from_pgn/1
    /// 
    /// Creates a Game initialized to the given pgn
    pub fn from_pgn(pgn: &str) -> Game {
        let mut reader = pgn_reader::BufferedReader::new_cursor(pgn);
        let mut visitor : MainlineVisitor = MainlineVisitor::start();
        if let Some(g) = reader.read_game(&mut visitor).unwrap() { return g; }
        panic!("Failed to parse game");
    }
}

impl MainlineVisitor {
    pub fn start() -> MainlineVisitor {
        MainlineVisitor {
            game: Game::start_position()
        }
    }
    
    fn find_by_target(&self, piece: Piece, to: usize) -> Search {
        self.game.moves().find_by_target(piece, to)
    }
}

impl Visitor for MainlineVisitor {
    type Result = Game;

    fn end_game(&mut self) -> Self::Result {
        self.game.clone()
    }

    fn header(&mut self, key_in: &[u8], value_in: pgn_reader::RawHeader<'_>) { 
        let key = String::from_utf8(key_in.to_vec()).unwrap();
        let value = String::from_utf8(value_in.as_bytes().to_vec()).unwrap();

        self.game.metadata.push((key, value))        
    }

    fn san(&mut self, san_plus: pgn_reader::SanPlus) { 
        let mov: Move = match san_plus.san {
            San::Normal { role , file: _, rank: _, capture: _, to, promotion: _ } => {
                match self.find_by_target(role.into(), to as usize) {
                    Search::Unambiguous(m) => m,
                    Search::Ambiguous(ms) => {
                        ms.into_iter().find(|&e| e.target_idx() == to as usize).unwrap()
                    }
                    Search::Empty => { 
                        panic!("Could not find move") 
                    }
                }
            },
            San::Castle(side) => {
                match side {
                    CastlingSide::KingSide => Move::short_castle(self.game.current_player()),
                    CastlingSide::QueenSide => Move::long_castle(self.game.current_player()),
                }
            },
            San::Put { role: _, to: _ } => panic!("Put moves are not supported"),
            San::Null => panic!("got null move")
        };
        self.game.make(mov);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_PGN: &str = include_str!("../../../../tests/fixtures/no-variations.pgn");

    mod creation {
        use super::*;

        #[test]
        fn parses_pgn_without_variations() {
            let g = Game::from_pgn(TEST_PGN);
            assert_eq!(g.played.len(), 29*2);
        }
    }
}