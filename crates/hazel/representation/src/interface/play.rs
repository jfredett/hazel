use hazel_core::ben::BEN;

use crate::{coup::rep::Move, game::action::Action};

pub trait Play where Self: Clone {
    type Metadata: Clone;

    fn apply(&self, action: &Action<Move, BEN>) -> Self;

    fn metadata(&self) -> Self::Metadata;

    fn apply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self;
}
