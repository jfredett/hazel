use crate::interface::alteration::Alteration;


/// implementing Alter states that the implementor can apply and reverse alterations to the board.
/// An alteration is defined by the Alteration enum.
pub trait Alter where Self: Sized {
    // TODO: Eventually this should probably return a result type.
    fn alter(&self, mov: Alteration) -> Self;

    // TODO: Eventually this should probably return a result type.
    fn alter_mut(&mut self, mov: Alteration) -> &mut Self;
}

// // TODO: Use this instead of the `compile` methods all over
// pub trait IntoAlter {
//     fn into_alter(self) -> Vec<Alteration>;
// }
//
// All Query are IntoAlter, except they're missing Metadata. This IntoAlter doesn't quite capture what I want it to mean, that it is a vector that ensures there is a metadata set.


pub fn setup<A>(alterations: impl Iterator<Item = Alteration>) -> A where A : Alter + Default {
    let mut ret = A::default();
    for alter in alterations {
        ret.alter_mut(alter);
    }
    ret
}

