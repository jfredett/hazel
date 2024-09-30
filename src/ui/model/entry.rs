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
* TODO: (24-SEP-2024) I don't want to debug this right now, but I think there is some kind of
* combinator logic to be had here. Proxying groups of engines seems like a good idea. I may want to
* have it return a richer structure, though, as right now it would not distinguish between which
* engine except by order.
*

pub struct EntryPair { pub left: Entry, pub right: Entry, }

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

26-SEP-2024 further note: I realized I could just implement tuples and vectors and such for this. A
vector of engines that speak the same protocol is also an engine that speaks the same protocol.
This will make it easy to chain arbitary engines together. But that led me to a better idea.

This will let me build 'sidecars' that can be attached to engines to do things like logging or
profiling or displaying to the UI, as the initial plan is (with PieceBoard running alongside the
main engine, and tracking the position of the game in a way convenient for the UI.

This makes it easy to manage a bunch of different chunks of functionality in little self-contained
packages. This I think needs to coincide with a move of this interface to `async` and be a little
bit more deliberate in the design.

I think I'm going to rename this to a trait:

```

trait Speaks<P> {
    type Error;
    type Configuration;
    fn async tell(&mut self, message: P) -> Result<P, Error>;
    fn async configure(&mut self, config: Configuration) -> Result<(), Error>;
}

```

This can be implemented for whatever protocol we like. Then there is an object:

```

struct Configuration<P, S> {
    engine: Box<dyn Speaks<P, S>>,
    spec: S
}

struct Rumble {
    engines: HashMap<ID, Box<dyn Speaks<impl P, S>)>>
    // Other fields as needed to store state
}

```

Don't quote me on the type, but ideally a Rumble of engines would be fully heterogeonous and simply
ignore messages it did not understand. I don't think I can do that necessarily, but that'd be the coolest way.

The idea is each engine can be added and dynamically configured, both within the Rumble (i.e.,
should Rumble prefer the response of a particular engine?) and within the Speaker itself, this
configuration is bespoke to the engine, provided by the Configuration type.

This means I can shove a big pile of engines into a Rumble, these can be utility engines,
profiling, strategy/evaluation, etc.
*/

impl Engine<UCIMessage> for Entry {
    // TODO: Should this be a static method instead of a trait method?
    fn exec(&mut self, message: UCIMessage) -> Vec<UCIMessage> {
        // update the boardstate
        self.engine.exec(message)
    }

    fn exec_message(&mut self, message: &str) -> Vec<UCIMessage> {
        self.engine.exec_message(message)
    }
}
