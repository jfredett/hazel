use tokio::sync::{mpsc, broadcast};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use std::sync::Arc;

struct Actor<S, R> {
    state: S,
    inbox: mpsc::Receiver<Box<dyn RunsOn<Actor<S, R>>>>,
    outbox: broadcast::Sender<R>,
}

impl<S, R> Actor<S, R> where S : Default {
    async fn new(inbox: mpsc::Receiver<Box<dyn RunsOn<Actor<S, R>>>>, outbox: broadcast::Sender<R>) -> Actor<S, R> {
        Actor {
            state: S::default(),
            inbox,
            outbox,
        }
    }

    async fn run(&mut self) {
        loop {
            let msg = self.inbox.recv().await.unwrap();
            msg.run(self);
        }
    }

    fn write(&mut self, v: R) {
        self.outbox.send(v);
    }
}

struct ActorHandle<S, R> {
    inbox: mpsc::Sender<Box<dyn RunsOn<Actor<S, R>>>>,
    // the issue is this isn't cloneable, which is correct, I really want the output side of it to
    outbox: Arc<Mutex<broadcast::Receiver<R>>>,
}

impl<S, R> Clone for ActorHandle<S, R> {
    fn clone(&self) -> Self {
        ActorHandle {
            inbox: self.inbox.clone(),
            outbox: self.outbox.clone(),
        }
    }
}

impl<S, R> ActorHandle<S, R> where 
    S : 'static + Default + Send,
    R : 'static + Clone + Send
{
    pub async fn new() -> ActorHandle<S, R> {
        let (inbox_tx, inbox_rx) = mpsc::channel(100);
        let (outbox_tx, outbox_rx) = broadcast::channel(100);

        tokio::spawn(async move {
            let mut actor_handle = Actor::<S, R>::new(inbox_rx, outbox_tx).await;
            actor_handle.run().await; 
        });

        ActorHandle {
            inbox: inbox_tx,
            outbox: Arc::new(Mutex::new(outbox_rx)),
        }
    }

    pub async fn send(&self, msg: Box<dyn RunsOn<Actor<S, R>>>) -> Result<(), mpsc::error::SendError<Box<dyn RunsOn<Actor<S, R>>>>> {
        self.inbox.send(msg).await
    }

    pub async fn read(&self) -> Result<R, broadcast::error::RecvError> {
        let mut ob = self.outbox.lock().await;
        ob.recv().await
    }
}

trait RunsOn<A> where Self: Send {
    fn run(&self, actor: &mut A);
}

struct HelloWorldMessage;

impl<S> RunsOn<Actor<S, String>> for HelloWorldMessage where S : Default {
    fn run(&self, actor: &mut Actor<S, String>) {
        actor.write("Hello World!".to_string());
    }
}

#[tokio::main]
async fn main() {
    println!("Actor Test");
    let handle = ActorHandle::<(), String>::new().await;

    let echo_actor = handle.clone();
    tokio::spawn(async move {
        while let Ok(msg) = echo_actor.read().await {
            println!("Received: {}", msg);
        }
    });

    loop {
        println!("Sending message");
        let _ = handle.send(Box::new(HelloWorldMessage)).await;
        let _ = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
