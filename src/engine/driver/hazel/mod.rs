use std::collections::HashMap;

use crate::engine::uci::UCIMessage;
use crate::game::variation::Variation;
use crate::types::witch::WitchHandle;
use crate::game::chess::position::Position;

mod state;
mod response;
mod messages;

pub use state::*;
pub use response::*;
pub use messages::*;

// NOTE: For now, I'm directly dealing with a `Position`, but I'd like to instead have Position be
// a familiar over some Variation, which would be how the UCI stuff would get recorded, and
// ultimately get output to PGN.

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Hazel {
    /// The current state of the engine.
    state: State,
    /// The current position of the active game. If this is `None`, it means no game is currently
    /// being played.
    /// TODO: Replace this with a familiar
    position: Option<Position>,
    /// Options set by the UI or other external sources.
    options: HashMap<String, Option<String>>
}

impl Hazel {
    pub fn is_ready(&self) -> bool {
        self.state == State::Ready
    }
}

pub type WitchHazel<const BUF_SIZE: usize> = WitchHandle<BUF_SIZE, Hazel, HazelResponse>;

impl<const BUF_SIZE: usize> WitchHazel<BUF_SIZE> {
    pub async fn write_uci(&self, msg: UCIMessage) {
        self.send(Box::new(msg)).await;
    }
}
