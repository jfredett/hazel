#![allow(dead_code)]

use crate::driver::stockfish::Stockfish;
use crate::ui::model::pieceboard::PieceBoard;
use crate::driver::Driver;
use crate::uci::UCIMessage;
use crate::engine::Engine;

use std::fmt::{Debug, Formatter};

use std::collections::HashMap;


pub struct Entry {
    pub config: Configuration, // for the current configuration options for the engine.
    pub boardstate: PieceBoard,
    pub engine: Box<dyn Engine<UCIMessage>>,
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entry")
            .field("config", &self.config)
            .field("boardstate", &self.boardstate)
            .finish()
    }
}

type Configuration = HashMap<String, String>;

impl Entry {
    fn new(engine: Box<dyn Engine<UCIMessage>>) -> Self {
        Self {
            config: HashMap::new(),
            boardstate: PieceBoard::new(),
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

/*
* TODO: I don't want to debug this right now, but I think there is some kind of combinator logic to
* be had here. Proxying groups of engines seems like a good idea. I may want to have it return a
* richer structure, though, as right now it would not distinguish between which engine except by
* order.
pub struct EntryPair {
    pub left: Entry,
    pub right: Entry,
}

impl EntryPair {
    pub fn new(left: Entry, right: Entry) -> Self {
        Self { left, right }
    }
}

impl Engine<UCIMessage> for EntryPair {
    fn exec(&mut self, message: UCIMessage) -> Vec<UCIMessage> {
        let left = self.left.exec(message.clone());
        let right = self.right.exec(message);
        left.into_iter().chain(right.into_iter()).collect()
    }

    fn exec_message(&mut self, message: &str) -> Vec<UCIMessage> {
        let left = self.left.exec_message(message);
        let right = self.right.exec_message(message);
        left.into_iter().chain(right.into_iter()).collect()
    }
}
*/

impl Engine<UCIMessage> for Entry {
    fn exec(&mut self, message: UCIMessage) -> Vec<UCIMessage> {
        // update the boardstate
        self.engine.exec(message)
    }

    fn exec_message(&mut self, message: &str) -> Vec<UCIMessage> {
        self.engine.exec_message(message)
    }
}
