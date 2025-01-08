use tokio::sync::{mpsc, broadcast};

#[derive(Clone, Debug)]
pub enum WitchError<T> {
    SendError(mpsc::error::SendError<T>),
    RecvError(broadcast::error::RecvError),
}
