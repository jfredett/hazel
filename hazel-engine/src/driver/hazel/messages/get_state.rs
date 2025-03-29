use async_trait::async_trait;

use witch::{MessageFor, Witch};
use crate::driver::{Hazel, HazelResponse};

pub struct GetState;

#[async_trait]
impl<const BUF_SIZE: usize> MessageFor<Witch<BUF_SIZE, Hazel, HazelResponse>> for GetState {
    async fn run(&self, witch: &mut Witch<BUF_SIZE, Hazel, HazelResponse>) {
        witch.write(HazelResponse::Debug(witch.state.clone()));
    }
}
