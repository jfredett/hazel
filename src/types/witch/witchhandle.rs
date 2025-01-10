use tokio::sync::{mpsc, broadcast};
use tokio::sync::Mutex;
use tracing::error;
use std::sync::Arc;

use super::{MessageForWitch, Witch};

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

        let sase = inbox_tx.clone();
        tokio::spawn(async move {
            let mut actor_handle = Witch::<BUF_SIZE, S, R>::new(inbox_rx, sase, outbox_tx).await;
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

    /// NOTE: Technically this duplicates Witch#send, but I think it is a correct, instrinsic duplication.
    ///
    /// Because `WitchHandle` does not have direct access to the `Witch`, it can send
    /// messages over it's channel. Since `Witch` has it's own channel over which it can send
    /// messages, it can, nearly identically, send messages to itself, which is handy for keeping
    /// all the logic in the messages themselves.
    ///
    /// However, these two approaches are, in essence, exactly the same except for the specific
    /// inbox being used, they _appear_ to duplicate. Indeed, we could extract this to a 'dumb
    /// send' function that replaces this, but tbh, I think the duplication is _fine_. It's less
    /// confusing, one fewer indirection, and ultimately the cost of the WET is better than the
    /// cost of the DRY.
    pub async fn send(&self, msg: MessageForWitch<BUF_SIZE, S, R>) {
        match self.inbox.send(msg).await {
            Ok(_) => {},
            Err(e) => {
                error!("Error sending message to WitchHandle: {:?}", e);
            }
        }
    }

    /// Blocking read of the output channel.
    pub async fn read(&self) -> Option<R> {
        self.outbox.lock().await
            .recv().await
            .ok()
    }
}
