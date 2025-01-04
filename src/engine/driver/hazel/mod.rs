// Actual implementation of Hazel as a chess engine


use std::{collections::HashMap, fmt::Display, path::Path};

use tokio::sync::{broadcast, mpsc};
use tracing::*;

use crate::{coup::rep::Move, engine::uci::UCIMessage, game::{action::Action, variation::Variation}, notation::{ben::BEN, fen::FEN, pgn::PGN, uci::UCI}};

pub use crate::engine::Engine;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum State {
    #[default] Idle,
    Ready,
    Pondering,
    Quitting,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = match self {
            State::Idle => "Idle",
            State::Ready => "Ready",
            State::Pondering => "Pondering",
            State::Quitting => "Quitting",
        };
        write!(f, "{}", state)
    }
}

const BUF_SIZE: usize = 256;

// TODO: Rename this to Hazel
pub struct Driver {
    state: State,
    game: Variation,
    // this should be a smarter type
    options: HashMap<String, Option<String>>,
    // This should be an actual queue, not a vec. It should be of finite length, and new enqueues
    // should block if the queue is full. It will generally be empty. The queue should optionally
    // keep a longer log of messages it has already processed, for debugging.
    queue_rx: mpsc::Sender<WitchLang>,
    queue_tx: mpsc::Receiver<WitchLang>,
    output_rx: broadcast::Sender<String>,
    output_tx: broadcast::Receiver<String>,
}

impl Default for Driver {
    fn default() -> Self {
        let (output_rx, output_tx) = broadcast::channel(BUF_SIZE);
        let (queue_rx, queue_tx) = mpsc::channel(BUF_SIZE);

        Driver {
            queue_rx,
            queue_tx,
            output_rx,
            output_tx,
            state: State::default(),
            options: HashMap::new(),
            game: Variation::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WitchLang {
    // Transition into a new state, triggering any state exit/entry actions as appropriate.
    Transition(State),
    // This should maybe be bytes->bytes or something? Ideally we could marshall these guys back to
    // a real type, but I don't really capture type information anywhere, it would have to be
    // guesses. I suppose for UCI we would just wrap it in a UCI tag, but for Hazel level it could
    // be typed-tagged?
    Set(String, String),
    // play a move on the current variation
    Play(Action<Move, BEN>),
    // Execute the given Query on the most recently sought familiar.
    Query(String),
    // Calculate a familiar which identifies a specified state given by the string
    // Save a familiar to the menagerie
    // Load a familiar from the menagerie
}


impl Driver {
    pub fn new() -> Driver {
        let (output_rx, output_tx) = broadcast::channel(BUF_SIZE);
        let (queue_rx, queue_tx) = mpsc::channel(BUF_SIZE);


        Driver {
            queue_rx,
            queue_tx,
            output_rx,
            output_tx,
            state: State::default(),
            options: HashMap::new(),
            game: Variation::default(),
        }
    }

    pub async fn spawn() -> tokio::task::JoinHandle<()> {
        tokio::spawn(async {
            let mut d = Driver::new();
            d.run().await;
        })
    }

    pub async fn run(&mut self) {
        info!("Starting up...");
        self.transition(State::Ready);
        info!("In Main Loop");
        loop {
            info!("Checking queue...");
            while let Some(m) = self.queue_tx.recv().await {
                debug!("Processing message: {:?}", m);
                self.process(&m).await;
            }

            info!("Queue empty, sleeping...");

            // FIXME: 50ms sleep just for now, this is maybe unnecessary?
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }

    pub fn debug(&self) -> bool {
        if let Some(flag) = self.options.get("debug").unwrap_or(&None) {
            flag == "true"
        } else {
            false
        }
    }

    pub fn load_pgn_file<P: AsRef<Path>>(&mut self, path: P) {
        let pgn = PGN::load(path).expect("Failed to load PGN file");
        for pair in pgn.tags() {
            self.options.insert(pair.name.clone(), Some(pair.value.clone()));
        }
    }

    fn respond(&mut self, message: UCIMessage) {
        self.write(message.to_string());
    }

    fn is_ready(&self) -> bool {
        self.state == State::Ready
    }

    fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        // FIXME: this is a stupid way to do things.
        self.enqueue(WitchLang::Set(key.into(), value.into()))
    }

    fn write(&mut self, message: impl Into<String>) {
        self.output_rx.send(message.into());
    }

    fn enqueue(&mut self, item: WitchLang) {
        self.queue_rx.send(item);
    }

    pub async fn process_uci(&mut self, m: UCIMessage) {
        match m {
            // 
            UCIMessage::IsReady => {
                if self.is_ready() {
                    self.respond(UCIMessage::ReadyOk);
                } else if self.debug() {
                    self.respond(UCIMessage::Info(vec![self.state.to_string()]));
                }
            },
            // Tell the engine to identify itself, in principle this could be used to set a 'UCI
            // Mode', but we just always understand UCI and assume
            UCIMessage::UCI => {
                self.respond(UCIMessage::ID("Hazel".to_string(), "0.1".to_string()));
            },
            // Set the debug flag
            UCIMessage::Debug(flag) => {
                self.set("debug", flag.to_string());
            },
            // This sets the given option to the given value. It converts everything down to a
            // string, this is not ideal, but I don't want to make this more complicated until
            // I have a good reason to.
            UCIMessage::SetOption(name, value) => {
                self.wait_until(|d| d.is_ready()).await;
                self.set(name.clone(), value.clone().map_or("".to_string(), |v| v));
            },
            // This instructs the engine that a new game is coming, however, if we get a `position`
            // command without receiving a `ucinewgame` command first, then we should never expect 
            // another `ucinewgame` command again. The spec says we should respond with an `readyok`
            // at the end of this, so Hazel just resets the variation and then responds with
            // `readyok`
            UCIMessage::UCINewGame => {
                self.game = Variation::default();
                self.transition(State::Ready);
            },
            UCIMessage::Position(fen, moves) => {
                // FIXME: This is not particularly ergonomic, probably an issue of genericizing
                // `Action`
                let setup_action : Action<Move, BEN> = Action::Setup(FEN::new(&fen).into());

                self.enqueue(WitchLang::Play(setup_action));

                for m_str in moves {
                    let uci_mov = UCI::try_from(m_str).expect("Invalid UCI Move");
                    let move_action = Action::Make(uci_mov.into());
                    self.enqueue(WitchLang::Play(move_action));
                }

                self.transition(State::Ready);
            },
            UCIMessage::Go(_) => {
                // do this only if we get the ponder subcommand
//                self.transition(State::Pondering);
            },
            UCIMessage::Stop => {
                self.transition(State::Idle);
            },
            UCIMessage::PonderHit => {
                // Raise the ponder-hit flag, we probably won't be using it for a while.
                todo!();
            },
            UCIMessage::Quit => {
                self.transition(State::Quitting);
            },
            _ => {
                error!("Unexpected message: {:?}", m);
                panic!("Failing due to unexpected UCI message");
            }
        };
    }

    async fn listen(&mut self) -> broadcast::Receiver<String> {
        // get a handle to the receiver stream
        self.output_rx.subscribe()
    }

    async fn process(&self, m: &WitchLang) {
        // if we're pondering, we need to make sure to run/monitor the search thread(s)
        // if we're idling, we should stop those search thread(s)
        // if we're ready, we just process messages
        todo!()
    }

    fn transition(&mut self, state: State) {
        self.state = state;
    }

    async fn wait_until(&self, test: impl Fn(&Self) -> bool) {
        while !test(self) {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    }
}



//
//
// Ideal API is something like:
//
// let mut driver = Driver::new();
//
// driver.send(UCIMessage::IsReady).await;
// driver.wait_for(|m| m.is_ready()).await; // blocks current thread until the guard is true
// // should take a list
// driver.send([ WitchLang::Play(Action::Setup(FEN::start_position().into()))]).await;
// driver.send([ WitchLang::Play(Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN)), WitchLang::Query("eval") ]).await;
//
// alternatively, it could be that each message is a type unto itself, it packages a method to
// perform a machine update, and we invert the process thing a bit, something like:
//
// impl Supports<Foo> for Driver {
//   async fn process(&mut self, foo: Foo) {
//     foo.execute(self);
//   }
// }
//
// impl ExecuteOn<Driver> for Foo {
//  fn execute(&self, driver: &mut Driver) {
//      let ret = do_stuff_to(driver);
//      self.return_addr.send(ret);
//  }
// }
//
// struct Foo {
//    return_addr: oneshot_channel
// }
//
//
// Granularity is up to the implementor, if I want to use a type-per-message, cool, if I want it to
// be an enum, also fine. This also makes it so that a message can include a response address, held
// by the sender, so something like the `return_addr` can be used to return an arbitrary calculated
// state out of the engine. 
//
// Then the UI can abstractly depend on something that `Supports<UCIEngineMessage>`, the
// implementation of that would reply with `UCIUIMessage`s. `Mask` can implement that as a
// passthrough to it's underlying process, thus stockfish/other engines should 'just work'. I can
// then also implement `Supports<Vec<T>> for T where T : Supports<T>`, etc.
//
//
//
//
// // in another thread
//
// driver.listen(|message| {
//      // handle a message here, the driver will block when the queue is empty, and is halted when
//      // the listen loop can be called multiple times, and each listener gets a copy of the
//      // output line.
//
// }).await;



// I want to redesign this a bit. This is where the tokio injects. Engine will be a async trait
// that does what it does now, just with async methods.
//
// Driver will have a message queue, Engine will parse UCI commands down to Hazel commands, which
// will then drive the engine. On start the engine will hand back a handle to the caller which it
// will send messages on, that handle can also be used to send messages to the engine.
//
// The driver will maintain a state, options hash, and variation. When a new game is encountered,
// the previous game is flushed (and optionally saved to a pgn file by option). Incoming messages
// are passed to handlers, which are a guard+action function pair. The guard will look for a
// particular message (maybe a message pattern?).
//
// Past messages should be kept in a rotating log, so that handlers can look for log _patterns_ and
// not simply a single message. This can be useful for scanning ahead and preparing/scheduling
// work.
//
// The UI will hold this open, and then manage communication from the UI to the engine via the comm
// handle.
//


/*
// TODO: This should be async, let's not fuck around
impl Engine<UCIMessage> for Driver {
    /// This method simplifies testing by allowing the driver to be fed a string
    /// which is then parsed by the UCI implementation. This exercises both sides of the UCI
    /// implementation. Since Driver doesn't handle the UCI stream directly, we know we'll
    /// always be listening to our dialect of UCI anyway.
    async fn exec_message(&mut self, message: &str) {
        self.exec(&UCIMessage::parse(message)).await;
    }

    async fn exec(&mut self, message: &UCIMessage) {
        info!("Executing UCI instruction: {:?}", &message);

        self.game.commit();

        match message {
            // GUI -> Engine
            UCIMessage::IsReady => {
                while !self.is_ready() {
                    // sleep
                }
                self.respond(UCIMessage::ReadyOk)
            }
            UCIMessage::UCI => {
                self.respond(UCIMessage::ID("Hazel".to_string(), "0.1".to_string()))
            }
            UCIMessage::Debug(flag) => {
                self.set("debug".to_string(), Some((*flag).to_string()))
            }
            UCIMessage::SetOption(name, value) => {
                self.set(name.clone(), value.clone())
            }
            UCIMessage::Register => { }
            UCIMessage::UCINewGame => {
                self.play(Action::Setup(FEN::start_position().into()));
                self.game.new_game();
                vec![]
            }
            UCIMessage::Position(fen, moves) => {
                self.game.setup(FEN::new(fen));

                for m_str in moves {
                    let m = UCI::try_from(m_str).expect("Invalid UCI Move");
                    self.game.make(m.into());
                }
                vec![]
            }
            UCIMessage::Go(_) => {
                /*
                let moves = self.game.moves();
                // select one at random
                let m = moves[0].clone();
                self.game.make(m);
                */
                vec![]
            }
            UCIMessage::Stop => {
                vec![]
            }
            UCIMessage::PonderHit => {
                vec![]
            }
            UCIMessage::Quit => {
                vec![]
            }
            // Engine -> GUI
            UCIMessage::ID(_,_) => vec![],
            UCIMessage::ReadyOk => vec![],
            UCIMessage::BestMove(_, _) => vec![],
            UCIMessage::CopyProtection => vec![],
            UCIMessage::Registration => vec![],
            UCIMessage::Info(_) => vec![],
            UCIMessage::Option(_) => vec![],
            _ => {
                error!("Unexpected message: {:?}", message);
                panic!("Unexpected message");
            }
        };
    }
}
*/


#[cfg(test)]
mod tests {
    use ben::BEN;

    use super::*;
    use crate::coup::rep::{Move, MoveType};
    use crate::game::action::Action;
    use crate::notation::*;

    impl Driver {
        pub fn log(&self) -> Vec<Action<Move, BEN>> {
            self.game.log()
        }
    }

    use crate::constants::{POS2_KIWIPETE_FEN, START_POSITION_FEN};

    /*
    #[tokio::test]
    async fn driver_parses_isready() {
        let mut driver = Driver::default();
        driver.process_uci(UCIMessage::IsReady).await;
        let response = driver.listen().await.recv().await.unwrap();
        assert_eq!(response, "readyok");
    }


    // TODO: Rewrite
    #[tokio::test]
    async fn driver_parses_uci() {
        let mut driver = Driver::default();
        let response = driver.exec(&UCIMessage::UCI).await;
        assert_eq!(response, vec![UCIMessage::ID("Hazel".to_string(), "0.1".to_string())]);
    }

    #[tokio::test]
    async fn driver_parses_debug() {
        let mut driver = Driver::default();
        assert!(!driver.debug());
        let response = driver.exec(&UCIMessage::Debug(true)).await;
        assert_eq!(response, vec![]);
        assert!(driver.debug())
    }

    #[tokio::test]
    async fn driver_sets_up_start_position() {
        let mut driver = Driver::default();
        let response = driver.exec_message("position startpos moves").await;
        assert_eq!(response, vec![]);
        assert_eq!(driver.game.log(), vec![
            Action::Setup(FEN::start_position().into())
        ]);
        assert_eq!(driver.game.current_position(), FEN::new(START_POSITION_FEN));
    }

    #[tokio::test]
    async fn driver_sets_up_arbitrary_position() {
        let mut driver = Driver::default();

        let response = driver.exec_message(&format!("position fen {} moves", POS2_KIWIPETE_FEN)).await;
        assert_eq!(response, vec![]);
        assert_eq!(driver.game.log(), vec![
            Action::Setup(FEN::new(POS2_KIWIPETE_FEN).into())
        ]);
        assert_eq!(driver.game.current_position(), FEN::new(POS2_KIWIPETE_FEN));
    }

    #[tokio::test]
    async fn driver_plays_moves_specified_by_position() {
        let mut driver = Driver::default();
        let response = driver.exec_message(&format!("position fen {} moves e2e4 e7e5", START_POSITION_FEN)).await;
        assert_eq!(response, vec![]);
        assert_eq!(driver.game.log(), vec![
            Action::Setup(FEN::new(START_POSITION_FEN).into()),
            Action::Make(Move::new(E2, E4, MoveType::UCI_AMBIGUOUS)),
            Action::Make(Move::new(E7, E5, MoveType::UCI_AMBIGUOUS))
        ]);
        assert_eq!(driver.game.current_position(), FEN::new("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2"));
    }
    */
}


