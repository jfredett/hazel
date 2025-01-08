use tokio::sync::{mpsc, broadcast};

// TODO: I'd love to unify all the errors from Witch down to a single type, but that is not
// straightforward.

#[derive(Clone, Debug)]
pub enum WitchError<T> {
    SendError(mpsc::error::SendError<T>),
    RecvError(broadcast::error::RecvError),
}
