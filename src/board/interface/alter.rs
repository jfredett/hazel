use crate::board::interface::alteration::Alteration;


/// implementing Alter states that the implementor can apply and reverse alterations to the board.
/// An alteration is defined by the Alteration enum.
pub trait Alter where Self: Sized {
    fn alter(&self, mov: Alteration) -> Self;

    fn alter_mut(&mut self, mov: Alteration) -> &mut Self {
        *self = self.alter(mov);
        self
    }
}

