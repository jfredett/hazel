use crate::board::interface::{Alter, Query};


pub trait Play where Self: Alter + Query + Clone {
    type Rule;
    type Metadata;

    fn apply(&self, rule: Self::Rule) -> Self;
    fn unwind(&self) -> Self;

    fn metadata(&self) -> Self::Metadata;

    fn apply_mut(&mut self, rule: Self::Rule) -> &mut Self {
        *self = self.apply(rule);
        self
    }
    fn unwind_mut(&mut self) -> &mut Self {
        *self = self.unwind();
        self
    }

}
