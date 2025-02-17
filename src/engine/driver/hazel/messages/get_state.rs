use async_trait::async_trait;
use tracing::error;

use crate::{engine::driver::{Hazel, HazelResponse}, types::witch::{MessageFor, Witch}};

pub struct GetState;

#[async_trait]
impl<const BUF_SIZE: usize> MessageFor<Witch<BUF_SIZE, Hazel, HazelResponse>> for GetState {
    async fn run(&self, witch: &mut Witch<BUF_SIZE, Hazel, HazelResponse>) {
        witch.write(HazelResponse::Debug(witch.state.clone()));
    }
}
