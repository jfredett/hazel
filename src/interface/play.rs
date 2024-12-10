use crate::{coup::rep::Move, game::action::Action, notation::ben::BEN};

pub trait Play where Self: Clone {
    // TODO: This should be a traitbound for the `Coup` trait, of which `Move` would be an
    // implementor.
    type Coup = Move;
    // TODO: Similarly, this should be a traitbound for the `BoardRep` trait, of which `BEN` would
    // be an implementor.
    type BoardRep = BEN;
    // FIXME: I don't love this.
    type Action = Action<Self::Coup, Self::BoardRep>;
    type Metadata: Clone;

    fn apply(&self, action: &Self::Action) -> Self;

    fn metadata(&self) -> Self::Metadata;

    fn apply_mut(&mut self, action: &Self::Action) -> &mut Self {
        *self = self.apply(action);
        self
    }
}

pub trait Unplay where Self: Clone + Play {
    fn unapply(&self, action: &Self::Action) -> Self;

    fn unapply_mut(&mut self, action: &Self::Action) -> &mut Self {
        *self = self.unapply(action);
        self
    }
}
