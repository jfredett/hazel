use async_trait::async_trait;

use witch::{MessageFor, Witch};
use crate::driver::hazel::{Hazel, HazelResponse};

pub struct GetPosition;

#[async_trait]
impl<const BUF_SIZE: usize> MessageFor<Witch<BUF_SIZE, Hazel, HazelResponse>> for GetPosition {
    async fn run(&self, witch: &mut Witch<BUF_SIZE, Hazel, HazelResponse>) {
        witch.write(HazelResponse::Position(witch.state.position.clone()));
    }
}
