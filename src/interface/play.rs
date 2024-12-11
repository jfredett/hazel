use crate::{coup::rep::Move, game::action::Action, notation::ben::BEN};

pub trait Play where Self: Clone {
    type Metadata: Clone;

    fn apply(&self, action: &Action<Move, BEN>) -> Self;

    fn metadata(&self) -> Self::Metadata;

    fn apply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self {
        *self = self.apply(action);
        self
    }
}

pub trait Unplay where Self: Clone + Play {
    fn unapply(&self, action: &Action<Move, BEN>) -> Self;

    fn unapply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self {
        *self = self.unapply(action);
        self
    }
}
