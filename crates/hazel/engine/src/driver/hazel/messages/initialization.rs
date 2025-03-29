use async_trait::async_trait;

use crate::{driver::{Hazel, HazelResponse, State}};
use witch::{MessageFor, Witch};

pub struct Initialization; /* {
    // this is where config file loading can go?
} */

#[async_trait]
impl<const BUF_SIZE: usize> MessageFor<Witch<BUF_SIZE, Hazel, HazelResponse>> for Initialization {
    async fn run(&self, witch: &mut Witch<BUF_SIZE, Hazel, HazelResponse>) {
        // witch.write(HazelResponse::Transition(State::Ready));
        witch.state.state = State::Ready;
    }
}
