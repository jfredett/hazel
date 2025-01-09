// use super::error::WitchError;
use async_trait::async_trait;

#[async_trait]
pub trait MessageFor<W> where Self: Send {
    async fn run(&self, actor: &mut W); // -> Result<(), WitchError<E>>;
}
