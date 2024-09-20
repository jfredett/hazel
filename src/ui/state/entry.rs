use crate::driver::stockfish::Stockfish;
use crate::driver::Driver;
use crate::uci::UCIMessage;
use crate::engine::Engine;
use crate::ui::viewmodels::pieceboard::PieceBoard;

use std::fmt::{Debug, Formatter};

use std::collections::HashMap;


pub struct Entry {
    board: PieceBoard, // for the current state of the board
    config: Configuration, // for the current configuration options for the engine.
    engine: Box<dyn Engine<UCIMessage>>,
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("board", &self.board)
            .field("config", &self.config)
            .finish()
    }
}

type Configuration = HashMap<String, String>;

impl Entry {
    fn new(engine: Box<dyn Engine<UCIMessage>>) -> Self {
        Self {
            board: PieceBoard::new(),
            config: HashMap::new(),
            engine
        }
    }
}

pub fn stockfish() -> Entry {
    Entry::new(Box::new(Stockfish::new()))
}

pub fn hazel() -> Entry {
    Entry::new(Box::new(Driver::new()))
}
