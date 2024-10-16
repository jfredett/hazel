// Actual implementation of Hazel as a chess engine


// TODO: This should grab UCI messages and take a Hazel::Brain and wire the two together. It can do
// some basic parsing of the UCI Messages to Hazel types, but otherwise be pretty 'dumb'
use tracing::*;

use crate::engine::uci::UCIMessage;

pub use crate::engine::Engine;


// TODO: Rename this to Hazel
#[derive(Default)]
pub struct Driver {
    debug: bool,
    game: Option<()>
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            debug: false,
            game: None
        }
    }
}

impl Engine<UCIMessage> for Driver {
    /// This method simplifies testing by allowing the driver to be fed a string
    /// which is then parsed by the UCI implementation. This exercises both sides of the UCI
    /// implementation. Since Driver doesn't handle the UCI stream directly, we know we'll
    /// always be listening to our dialect of UCI anyway.
    fn exec_message(&mut self, message: &str) -> Vec<UCIMessage> {
        self.exec(&UCIMessage::parse(message))
    }

    fn exec(&mut self, message: &UCIMessage) -> Vec<UCIMessage> {
        info!("Executing UCI instruction: {:?}", &message);

        match message {
            // GUI -> Engine
            UCIMessage::IsReady => {
                vec![UCIMessage::ReadyOk]
            }
            UCIMessage::UCI => {
                vec![UCIMessage::ID("Hazel".to_string(), "0.1".to_string())]
            }
            UCIMessage::Debug(flag) => {
                self.debug = *flag;
                vec![]
            }
            UCIMessage::SetOption(_name, _values) => {
                vec![]
            }
            UCIMessage::Register => {
                vec![]
            }
            UCIMessage::UCINewGame => {
                self.game = None;
                vec![]
            }
            UCIMessage::Position(_fen, _moves) => {
                vec![]
            }
            UCIMessage::Go(_) => {
                /*
                let moves = self.game.moves();
                // select one at random
                let m = moves[0].clone();
                self.game.make(m);
                None
                */
                vec![]
            }
            UCIMessage::Stop => {
                vec![]
            }
            UCIMessage::PonderHit => {
                vec![]
            }
            UCIMessage::Quit => {
                vec![]
            }
            // Engine -> GUI
            UCIMessage::ID(_,_) => vec![],
            UCIMessage::ReadyOk => vec![],
            UCIMessage::BestMove(_, _) => vec![],
            UCIMessage::CopyProtection => vec![],
            UCIMessage::Registration => vec![],
            UCIMessage::Info(_) => vec![],
            UCIMessage::Option(_) => vec![],
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
        let response = driver.exec(&UCIMessage::IsReady);
        assert_eq!(response, vec![UCIMessage::ReadyOk]);
        // this but with a vec![] instead of Some
    }

    #[test]
    fn driver_parses_uci() {
        let mut driver = Driver::new();
        let response = driver.exec(&UCIMessage::UCI);
        assert_eq!(response, vec![UCIMessage::ID("Hazel".to_string(), "0.1".to_string())]);
    }

    #[test]
    fn driver_parses_debug() {
        let mut driver = Driver::new();
        assert!(!driver.debug);
        let response = driver.exec(&UCIMessage::Debug(true));
        assert_eq!(response, vec![]);
        assert!(driver.debug)
    }

    #[ignore]// WIP as I refactor board rep
    #[test]
    fn driver_sets_up_start_position() {
        let mut driver = Driver::new();
        let response = driver.exec_message("position startpos moves");
        assert_eq!(response, vec![]);
        assert!(driver.game.is_some());
        // assert!(driver.game.unwrap().to_fen() == START_POSITION_FEN);
    }

    #[ignore]// WIP as I refactor board rep
    #[test]
    fn driver_sets_up_arbitrary_position() {
        let mut driver = Driver::new();

        let response = driver.exec_message(&format!("position fen {} moves", POS2_KIWIPETE_FEN));
        assert_eq!(response, vec![]);
        assert!(driver.game.is_some());
        // assert!(driver.game.unwrap().to_fen() == POS2_KIWIPETE_FEN);
    }

    #[ignore] // WIP as I refactor board rep
    #[test]
    fn driver_plays_moves_specified_by_position() {
        let mut driver = Driver::new();
        let response = driver.exec_message(&format!("position fen {} moves e2e4 e7e5", START_POSITION_FEN));
        assert_eq!(response, vec![]);
        assert!(driver.game.is_some());
        // assert_eq!(driver.game.unwrap().to_fen(), "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2");
    }
}
