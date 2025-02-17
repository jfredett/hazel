use async_trait::async_trait;
use tracing::error;

use crate::{engine::driver::{Hazel, HazelResponse, State}, types::witch::{MessageFor, Witch}};

pub struct HazelInitialization; /* {
    // this is where config file loading can go?
} */

#[async_trait]
impl<const BUF_SIZE: usize> MessageFor<Witch<BUF_SIZE, Hazel, HazelResponse>> for HazelInitialization {
    async fn run(&self, witch: &mut Witch<BUF_SIZE, Hazel, HazelResponse>) {
        // witch.write(HazelResponse::Transition(State::Ready));
        witch.state.state = State::Ready;
    }
}
