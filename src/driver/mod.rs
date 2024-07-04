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
        driver.exec(UCIMessage::IsReady);

    }
}
