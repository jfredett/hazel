// use super::error::WitchError;

pub trait MessageFor<W> where Self: Send {
    fn run(&self, actor: &mut W); // -> Result<(), WitchError<E>>;
}
