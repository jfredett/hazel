use std::collections::HashMap;

use async_trait::async_trait;
use tracing::error;

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

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Hazel {
    /// The current state of the engine.
    state: State,
    /// The current position of the active game. If this is `None`, it means no game is currently
    /// being played.
    position: Option<Position>,
    /// A Variation containing games loaded from some source, or saved from the current gamestate.
    /// NOTE: This is not like the others. Maybe `Hazel` should focus on being just the UCI-related
    /// bits, and then it can talk to a `WitchHazel` which is just the database bits?
    /// 11-JAN-2025 0053 - This should be it's own witch, that acts as the 'database' end to which
    /// the WitchHazel can hold a handle.
    game: Variation,
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



