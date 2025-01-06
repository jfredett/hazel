use tokio::sync::broadcast::error::{RecvError, TryRecvError};
use tokio::sync::{mpsc, broadcast};
use tokio::task::JoinHandle;
use std::fmt::Debug;


// TODO: would love to separate Message type (M) from Response type (R)


struct Driver<const BUF_SIZE: usize, R, S>
where
        R : Send + Clone
{
    state: S,
    inbox: mpsc::Receiver<Box<dyn RunsOn<Self>>>,
    outbox: broadcast::Sender<R>,
}

impl<const BUF_SIZE: usize, R, S> Driver<BUF_SIZE, R, S>
where 
    R: Send + Clone + Debug + 'static,
    S: Send + Clone + Default + Debug + 'static
{
    pub async fn spawn() -> (mpsc::Sender<Box<dyn RunsOn<Self>>>, broadcast::Receiver<R>, JoinHandle<()>) {
        let (inbox_tx, inbox_rx) = mpsc::channel(32);
        let (outbox_tx, outbox_rx) = broadcast::channel(32);
        let mut driver : Driver<BUF_SIZE, R, S> = Driver { state: S::default(), inbox: inbox_rx, outbox: outbox_tx };
        let handle = tokio::spawn(async move { driver.run().await });
        (inbox_tx, outbox_rx, handle)
    }

    pub async fn run(&mut self) {
        loop {
            let msg = self.inbox.recv().await.unwrap();
            msg.run(self);
        }
    }
}


trait RunsOn<D> where Self: Send {
    fn run(&self, driver: &mut D);
}

#[derive(Clone, Debug)]
struct HelloWorldMessage;
#[derive(Clone, Debug)]
struct GoodbyeWorldMessage;

#[derive(Clone, Debug)]
struct AnotherMessage {
    pub message: String
}

#[derive(Clone, Debug)]
struct IncrementCounterMessage;

#[derive(Clone, Debug)]
struct DisplayStateMessage;


impl<const BUF_SIZE: usize> RunsOn<Driver<BUF_SIZE, String, usize>> for IncrementCounterMessage {
    fn run(&self, driver: &mut Driver<BUF_SIZE, String, usize>) {
        driver.state += 1;
    }
}


impl<const BUF_SIZE: usize, S> RunsOn<Driver<BUF_SIZE, String, S>> for DisplayStateMessage where S: Debug {
    fn run(&self, driver: &mut Driver<BUF_SIZE, String, S>) {
        driver.outbox.send(format!("State: {:?}", driver.state)).unwrap();
    }
}

impl<const BUF_SIZE: usize, S> RunsOn<Driver<BUF_SIZE, String, S>> for HelloWorldMessage {
    fn run(&self, driver: &mut Driver<BUF_SIZE, String, S>) {
        driver.outbox.send("Hello World!".to_string()).unwrap();
    }
}

impl<const BUF_SIZE: usize, S> RunsOn<Driver<BUF_SIZE, String, S>> for GoodbyeWorldMessage {
    fn run(&self, driver: &mut Driver<BUF_SIZE, String, S>) {
        driver.outbox.send("Goodbye World!".to_string()).unwrap();
    }
}

impl<const BUF_SIZE: usize, S> RunsOn<Driver<BUF_SIZE, String, S>> for AnotherMessage {
    fn run(&self, driver: &mut Driver<BUF_SIZE, String, S>) {
        driver.outbox.send(self.message.clone()).unwrap();
    }
}



#[tokio::main]
pub async fn main() {
    println!("Driver Example Stub Thing");

    let (tx, mut rx, handle) = Driver::<32, String, usize>::spawn().await;

    let echo = tokio::spawn(async move {
        let mut idx = 0;
        loop {
            match rx.try_recv() {
                Ok(msg) => {
                    idx += 1;
                    println!("{}: {}", idx, msg);
                }
                Err(TryRecvError::Lagged(lag)) => {
                    println!("Lagged by {}, catching up", lag);
                    for _ in 0..lag {
                        if let Ok(msg) = rx.try_recv() {
                            println!("CATCH: {}", msg);
                        }
                    }
                }
                Err(TryRecvError::Empty) => { println!("No messages in queue"); }
                Err(e) => { panic!("Error receiving message: {:?}", e); },
            }

            println!("State of Queue: {:?}", rx.len());
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let writer = tokio::spawn(async move {
        loop {
            tx.send(Box::new(HelloWorldMessage)).await.unwrap();
            tx.send(Box::new(GoodbyeWorldMessage)).await.unwrap();
            tx.send(Box::new(AnotherMessage { message: "Another Message".to_string() })).await.unwrap();
            tx.send(Box::new(IncrementCounterMessage)).await.unwrap();
            tx.send(Box::new(DisplayStateMessage)).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    tokio::try_join!(echo, writer, handle).unwrap();
}
