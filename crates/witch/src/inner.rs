use tokio::sync::{mpsc, broadcast};

use super::MessageForWitch;

pub struct Witch<const BUF_SIZE : usize, S, R>

where S : Default + Send + Clone + 'static,
      R : Send + Clone + 'static
{
    /// Internal State
    pub state: S,
    /// Incoming messages
    inbox: mpsc::Receiver<MessageForWitch<BUF_SIZE, S, R>>,
    /// Self-addressed Stamped Envelope -- so I can send messages back to myself.
    sase: mpsc::Sender<MessageForWitch<BUF_SIZE, S, R>>,
    /// Outgoing messages
    outbox: broadcast::Sender<R>,
}

impl<const BUF_SIZE : usize, S, R> Witch<BUF_SIZE, S, R> 
where S : Default + Send + Clone + 'static,
      R : Send + Clone + 'static 
{
    pub(super) async fn new(
        inbox: mpsc::Receiver<MessageForWitch<BUF_SIZE, S, R>>,
        sase: mpsc::Sender<MessageForWitch<BUF_SIZE, S, R>>,
        outbox: broadcast::Sender<R>
    ) -> Witch<BUF_SIZE, S, R> {
        Witch {
            state: S::default(),
            inbox,
            sase,
            outbox,
        }
    }

    pub(super) async fn run(&mut self) {
        loop {
            let msg = self.inbox.recv().await.unwrap();
            msg.run(self).await;
        }
    }

    pub fn write(&mut self, v: R) {
        let _ = self.outbox.send(v);
    }

    // FIXME: Technically this duplicates WitchHandle#send, but IDK if I should rely on the extra
    // hop or just eat the cost of the duplication.
    pub async fn send(&self, msg: MessageForWitch<BUF_SIZE, S, R>) {
        match self.sase.send(msg).await {
            Ok(_) => {},
            Err(e) => {
                tracing::error!("Error sending message to WitchHandle: {:?}", e);
            }
        }
    }
}
