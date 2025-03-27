use std::collections::HashMap;

use crate::uci::UCIMessage;
use hazel::types::witch::WitchHandle;
use hazel::game::chess::position::Position;

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
    /// TODO: Be able to share a cached version of this via an Arc.
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

#[allow(async_fn_in_trait)] // This is only going to be used in my code, and hopefully I'll be able to eliminate the extension trait soonish
pub trait WitchHazelUnderstandsUCI {
    async fn write_uci(&self, msg: UCIMessage);
}

impl<const BUF_SIZE: usize> WitchHazelUnderstandsUCI for WitchHazel<BUF_SIZE> {
    async fn write_uci(&self, msg: UCIMessage) {
        self.send(Box::new(msg)).await;
    }
}
