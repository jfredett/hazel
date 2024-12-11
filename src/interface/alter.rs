use crate::interface::alteration::Alteration;


/// implementing Alter states that the implementor can apply and reverse alterations to the board.
/// An alteration is defined by the Alteration enum.
pub trait Alter where Self: Sized {
    fn alter(&self, mov: Alteration) -> Self;

    fn alter_mut(&mut self, mov: Alteration) -> &mut Self {
        *self = self.alter(mov);
        self
    }
}

// // TODO: Use this instead of the `compile` methods all over
// pub trait IntoAlter {
//     fn into_alter(self) -> Vec<Alteration>;
// }
//
//
// All Query are IntoAlter, except they're missing Metadata. This IntoAlter doesn't quite capture what I want it to mean, that it is a vector that ensures there is a metadata set.
