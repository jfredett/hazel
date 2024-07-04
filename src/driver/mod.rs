// Actual implementation of Hazel as a chess engine


use tracing::*;


use std::io;
use crate::uci::UCIMessage;
use crate::game::Game;


pub struct Driver {
    debug: bool
}

impl Driver {

    pub fn new() -> Self {
        Driver {
            debug: false
        }
    }

    pub fn exec(&mut self, message: UCIMessage) -> Option<UCIMessage> {
        match message {
            // GUI -> Engine
            UCIMessage::IsReady => {
                info!("Received IsReady");
                Some(UCIMessage::ReadyOk)
            }
            UCIMessage::UCI => {
                info!("Received UCI");
                Some(UCIMessage::ID("Hazel".to_string(), "0.1".to_string()))
            }
            UCIMessage::Debug(flag) => {
                info!("Received Debug");
                self.debug = flag;
                None
            }
            UCIMessage::SetOption(name, values) => {
                info!("Received SetOption");
                None
            }
            UCIMessage::Register => {
                info!("Received Register");
                None
            }
            UCIMessage::UCINewGame => {
                info!("Received UCINewGame");
                None
            }
            UCIMessage::Position(_, _) => {
                info!("Received Position");
                None
            }
            UCIMessage::Go(_) => {
                info!("Received Go");
                None
            }
            UCIMessage::Stop => {
                info!("Received Stop");
                None
            }
            UCIMessage::PonderHit => {
                info!("Received PonderHit");
                None
            }
            UCIMessage::Quit => {
                info!("Received Quit");
                None
            }
            // Engine -> GUI
            UCIMessage::ID(_,_) => None,
            UCIMessage::ReadyOk => None,
            UCIMessage::BestMove(_, _) => None,
            UCIMessage::CopyProtection => None,
            UCIMessage::Registration => None,
            UCIMessage::Info(_) => None,
            UCIMessage::Option(_, _) => None,
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

    #[traced_test]
    #[test]
    fn driver_parses_isready() {
        let mut driver = Driver::new();
        let response = driver.exec(UCIMessage::IsReady);
        assert_eq!(response, Some(UCIMessage::ReadyOk));
    }

    #[traced_test]
    #[test]
    fn driver_parses_uci() {
        let mut driver = Driver::new();
        let response = driver.exec(UCIMessage::UCI);
        assert_eq!(response, Some(UCIMessage::ID("Hazel".to_string(), "0.1".to_string())));
    }

    #[traced_test]
    #[test]
    fn driver_parses_debug() {
        let mut driver = Driver::new();
        assert!(!driver.debug);
        let response = driver.exec(UCIMessage::Debug(true));
        assert_eq!(response, None);
        assert!(driver.debug)
    }
}
