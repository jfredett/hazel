use tokio::sync::{mpsc, broadcast};
use tokio::sync::Mutex;
use tracing::error;
use std::sync::Arc;

use super::{MessageForWitch, Witch, WitchError};

#[derive(Clone)]
pub struct WitchHandle<const BUF_SIZE: usize, S, R>
where S: 'static + Clone + Send + Default,
      R: 'static + Clone + Send
{
    inbox: mpsc::Sender<MessageForWitch<BUF_SIZE, S, R>>,
    // the issue is this isn't cloneable, which is correct, I really want the output side of it to
    outbox: Arc<Mutex<broadcast::Receiver<R>>>,
}

impl<const BUF_SIZE: usize, S, R> WitchHandle<BUF_SIZE, S, R> where
    S : 'static + Clone + Send + Default,
    R : 'static + Clone + Send
{
    pub async fn new() -> WitchHandle<BUF_SIZE, S, R> {
        let (inbox_tx, inbox_rx) = mpsc::channel(100);
        let (outbox_tx, outbox_rx) = broadcast::channel(100);

        tokio::spawn(async move {
            let mut actor_handle = Witch::<BUF_SIZE, S, R>::new(inbox_rx, outbox_tx).await;
            actor_handle.run().await;
        });

        WitchHandle {
            inbox: inbox_tx,
            outbox: Arc::new(Mutex::new(outbox_rx)),
        }
    }

    pub async fn with_initialization_message(msg: MessageForWitch<BUF_SIZE, S, R>) -> WitchHandle<BUF_SIZE, S, R> {
        let handle = WitchHandle::new().await;
        handle.send(msg).await;
        handle
    }

    pub async fn send(&self, msg: MessageForWitch<BUF_SIZE, S, R>) {
        match self.inbox.send(msg).await {
            Ok(_) => {},
            Err(e) => {
                error!("Error sending message to WitchHandle: {:?}", e);
            }
        }
    }

    /// Nonblocking read of the output channel.
    pub async fn read(&self) -> Option<R> {
        self.outbox.lock().await
            .recv().await
            .ok()
    }

}
