use tokio::sync::{mpsc, broadcast};
use tokio::sync::Mutex;
use std::sync::Arc;

use super::message_for::MessageFor;
use super::{Witch, MessageForWitch};

struct WitchHandle<const BUF_SIZE: usize, S, R> 
where S: 'static + Clone + Send + Default,
      R: 'static + Clone + Send
{
    inbox: mpsc::Sender<MessageForWitch<BUF_SIZE, S, R>>,
    // the issue is this isn't cloneable, which is correct, I really want the output side of it to
    outbox: Arc<Mutex<broadcast::Receiver<R>>>,
}

impl<const BUF_SIZE: usize, S, R> Clone for WitchHandle<BUF_SIZE, S, R> 
where S: 'static + Clone + Send + Default,
      R: 'static + Clone + Send
{
    fn clone(&self) -> Self {
        WitchHandle {
            inbox: self.inbox.clone(),
            outbox: self.outbox.clone(),
        }
    }
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

    pub async fn send(&self, msg: Box<dyn MessageFor<Witch<BUF_SIZE, S, R>>>) -> Result<(), mpsc::error::SendError<Box<dyn MessageFor<Witch<BUF_SIZE, S, R>>>>> {
        self.inbox.send(msg).await
    }

    pub async fn read(&self) -> Result<R, broadcast::error::RecvError> {
        let mut ob = self.outbox.lock().await;
        ob.recv().await
    }
}
