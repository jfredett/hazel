use std::collections::HashMap;
use std::io;

use tracing::*;

use crate::engine::uci::UCIMessage;
use crate::game::variation::Variation;
use crate::types::witch::{MessageFor, Witch, WitchHandle};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum State {
    #[default] Idle,
    Ready,
    Pondering,
    Quitting,
}

#[derive(Default, Clone, Debug)]
pub struct Hazel {
    state: State,
    game: Variation,
    options: HashMap<String, Option<String>>
}

#[derive(Clone, Debug, Default)]
pub enum HazelResponse {
    #[default] Silence,
    FullState(Hazel), // NOTE: Probably I don't actually want this, but useful for the start.
    UCIResponse(UCIMessage),
}

pub type WitchHazel<const BUF_SIZE: usize> = WitchHandle<BUF_SIZE, Hazel, HazelResponse>;

impl<const BUF_SIZE: usize> WitchHazel<BUF_SIZE> {

}

impl<const BUF_SIZE: usize> MessageFor<Witch<BUF_SIZE, Hazel, HazelResponse>> for UCIMessage {
    fn run(&self, actor: &mut Witch<BUF_SIZE, Hazel, HazelResponse>) {
        todo!()
    }
}
