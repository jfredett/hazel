mod message_for;
mod witchhandle;
mod inner;

pub use inner::*;

pub type MessageForWitch<const BUF_SIZE: usize, S, R> = Box<dyn message_for::MessageFor<Witch<BUF_SIZE, S, R>>>;

