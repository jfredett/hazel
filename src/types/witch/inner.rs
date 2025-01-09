use tokio::sync::{mpsc, broadcast};

use super::MessageForWitch;

pub struct Witch<const BUF_SIZE : usize, S, R>

where S : Default + Send + Clone + 'static,
      R : Send + Clone + 'static
{
    pub state: S,
    inbox: mpsc::Receiver<MessageForWitch<BUF_SIZE, S, R>>,
    outbox: broadcast::Sender<R>,
}

impl<const BUF_SIZE : usize, S, R> Witch<BUF_SIZE, S, R> 
where S : Default + Send + Clone + 'static,
      R : Send + Clone + 'static 
{
    pub(super) async fn new(inbox: mpsc::Receiver<MessageForWitch<BUF_SIZE, S, R>>, outbox: broadcast::Sender<R>) -> Witch<BUF_SIZE, S, R> {
        Witch {
            state: S::default(),
            inbox,
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
}
