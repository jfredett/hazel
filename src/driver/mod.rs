// Actual implementation of Hazel as a chess engine


use tracing::*;


use std::io;
use crate::uci::UCIMessage;
use crate::game::Game;


struct Driver {
}

impl Driver {

    pub fn new() -> Self {
        Driver {}
    }

    pub fn exec(&mut self, message: UCIMessage) -> UCIMessage {
        match message {
            UCIMessage::IsReady => {
                info!("Received IsReady");
                UCIMessage::ReadyOk
            }
            UCIMessage::UCI => {
                info!("Received UCI");
                UCIMessage::ID("Hazel".to_string(), "0.1".to_string())
            }

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
        assert_eq!(response, UCIMessage::ReadyOk);
    }

    #[traced_test]
    #[test]
    fn driver_parses_uci() {
        let mut driver = Driver::new();
        let response = driver.exec(UCIMessage::UCI);
        assert_eq!(response, UCIMessage::ID("Hazel".to_string(), "0.1".to_string()));
    }



}
