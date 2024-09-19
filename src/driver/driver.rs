// Actual implementation of Hazel as a chess engine

use tracing::*;

use std::io;
use crate::uci::UCIMessage;
use crate::game::Game;
use crate::movement::Move;


pub struct Driver {
    debug: bool,
    game: Option<Game>
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            debug: false,
            game: None
        }
    }

    /// This method simplifies testing by allowing the driver to be fed a string
    /// which is then parsed by the UCI implementation. This exercises both sides of the UCI
    /// implementation. Since Driver doesn't handle the UCI stream directly, we know we'll
    /// always be listening to our dialect of UCI anyway.
    #[cfg(test)]
    pub fn exec_message(&mut self, message: &str) -> Option<UCIMessage> {
        self.exec(UCIMessage::parse(message))
    }

    pub fn exec(&mut self, message: UCIMessage) -> Option<UCIMessage> {
        info!("Executing UCI instruction: {:?}", message);

        match message {
            // GUI -> Engine
            UCIMessage::IsReady => {
                Some(UCIMessage::ReadyOk)
            }
            UCIMessage::UCI => {
                Some(UCIMessage::ID("Hazel".to_string(), "0.1".to_string()))
            }
            UCIMessage::Debug(flag) => {
                self.debug = flag;
                None
            }
            UCIMessage::SetOption(name, values) => {
                None
            }
            UCIMessage::Register => {
                None
            }
            UCIMessage::UCINewGame => {
                self.game = None;
                None
            }
            UCIMessage::Position(fen, moves) => {
                let mut game = Game::from_fen(&fen);
                for m in moves {
                    game.make(Move::from_uci(&m));
                }

                self.game = Some(game);
                None
            }
            UCIMessage::Go(_) => {
                /*
                let moves = self.game.moves();
                // select one at random
                let m = moves[0].clone();
                self.game.make(m);
                None
                */
                None
            }
            UCIMessage::Stop => {
                None
            }
            UCIMessage::PonderHit => {
                None
            }
            UCIMessage::Quit => {
                None
            }
            // Engine -> GUI
            UCIMessage::ID(_,_) => None,
            UCIMessage::ReadyOk => None,
            UCIMessage::BestMove(_, _) => None,
            UCIMessage::CopyProtection => None,
            UCIMessage::Registration => None,
            UCIMessage::Info(_) => None,
            UCIMessage::Option(_) => None,
            _ => {
                error!("Unexpected message: {:?}", message);
                panic!("Unexpected message");
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    use crate::constants::{START_POSITION_FEN, POS2_KIWIPETE_FEN};

    #[test]
    fn driver_parses_isready() {
        let mut driver = Driver::new();
        let response = driver.exec(UCIMessage::IsReady);
        assert_eq!(response, Some(UCIMessage::ReadyOk));
    }

    #[test]
    fn driver_parses_uci() {
        let mut driver = Driver::new();
        let response = driver.exec(UCIMessage::UCI);
        assert_eq!(response, Some(UCIMessage::ID("Hazel".to_string(), "0.1".to_string())));
    }

    #[test]
    fn driver_parses_debug() {
        let mut driver = Driver::new();
        assert!(!driver.debug);
        let response = driver.exec(UCIMessage::Debug(true));
        assert_eq!(response, None);
        assert!(driver.debug)
    }

    #[test]
    fn driver_sets_up_start_position() {
        let mut driver = Driver::new();
        let response = driver.exec_message("position startpos moves");
        assert_eq!(response, None);
        assert!(driver.game.is_some());
        assert!(driver.game.unwrap().to_fen() == START_POSITION_FEN);
    }

    #[test]
    fn driver_sets_up_arbitrary_position() {
        let mut driver = Driver::new();

        let response = driver.exec_message(&format!("position fen {} moves", POS2_KIWIPETE_FEN));
        assert_eq!(response, None);
        assert!(driver.game.is_some());
        assert!(driver.game.unwrap().to_fen() == POS2_KIWIPETE_FEN);
    }

    #[test]
    fn driver_plays_moves_specified_by_position() {
        let mut driver = Driver::new();
        let response = driver.exec_message(&format!("position fen {} moves e2e4 e7e5", START_POSITION_FEN));
        assert_eq!(response, None);
        assert!(driver.game.is_some());
        // BUG: This fen is probably wrong. It should be the position after e2e4 e7e5
        assert_eq!(driver.game.unwrap().to_fen(), "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2");
    }
}
