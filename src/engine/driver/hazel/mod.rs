use std::collections::HashMap;

use async_trait::async_trait;
use tracing::error;

use crate::constants::START_POSITION_FEN;
use crate::coup::rep::Move;
use crate::engine::uci::UCIMessage;
use crate::game::reason::Reason;
use crate::game::variation::Variation;
use crate::notation::ben::BEN;
use crate::notation::uci::UCI;
use crate::types::witch::{MessageFor, Witch, WitchHandle};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum State {
    #[default] Idle,
    Ready,
    Pondering,
    Quitting,
}

#[derive(Default, Clone, Debug)]
pub struct Hazel {
    state: State,
    position: Option<Position>,
    game: Variation,
    options: HashMap<String, Option<String>>
}

#[derive(Debug, Clone)]
pub struct Position {
    initial: BEN,
    moves: Vec<Move>,
}

impl Position {
    fn new(fen: impl Into<BEN>, moves: Vec<Move>) -> Self {
        Self { initial: fen.into(), moves }
    }
}

impl Hazel {
    pub fn is_ready(&self) -> bool {
        self.state == State::Ready
    }
}

#[derive(Clone, Debug, Default)]
pub enum HazelResponse {
    #[default] Silence,
    Transition(State),
    UCIResponse(UCIMessage),
}

pub type WitchHazel<const BUF_SIZE: usize> = WitchHandle<BUF_SIZE, Hazel, HazelResponse>;

impl<const BUF_SIZE: usize> WitchHazel<BUF_SIZE> {
    pub async fn write_uci(&self, msg: UCIMessage) {
        self.send(Box::new(msg)).await;
    }
}

pub struct HazelInitialization; /* {
    // this is where config file loading can go?
} */

#[async_trait]
impl<const BUF_SIZE: usize> MessageFor<Witch<BUF_SIZE, Hazel, HazelResponse>> for HazelInitialization {
    async fn run(&self, witch: &mut Witch<BUF_SIZE, Hazel, HazelResponse>) {
        // witch.write(HazelResponse::Transition(State::Ready));
        witch.state.state = State::Ready;
    }
}

#[async_trait]
impl<const BUF_SIZE: usize> MessageFor<Witch<BUF_SIZE, Hazel, HazelResponse>> for UCIMessage {
    // NOTE: At least from some light testing with stockfish, bad commands are ignored entirely.
    // I've chosen to log them to STDERR
    async fn run(&self, witch: &mut Witch<BUF_SIZE, Hazel, HazelResponse>) {
        // To implement these, there are some UCI commands that are more like 'queries', I need to
        // treat them as such, these are long-term things that should not block further message
        // processing, but _should_ block further _UCI_ message processing.
        //
        // I need a way to allocate some kind of space for this in the actor dynamically*, but just
        // to store messages in, so I guess I can have a 'queue' subsystem of Hazel messages that
        // allocate queues by name or something?
        //
        // Alternatively I could do the simple thing and make Witches UCI-aware.
        //
        // Maybe a system where you messages:
        //
        // Requeue(QueueName, Message)
        // StartQueueProcessor(async fn, QueueName)
        // HaltQueueProcessor(QueueName)
        //
        // Requeuing pulls it off the main queue and sends it to the alternate queue by name, the
        // start/halt messages start and stop a dynamically provided queue processor task, this
        // would replace the `run` command in Witch with a trait object (similar to how messages
        // work)?
        // 
        match self {
            UCIMessage::UCI => {
                witch.write(HazelResponse::UCIResponse(UCIMessage::ID("hazel".to_string(), "0.1".to_string())));
            },
            UCIMessage::IsReady => {
                witch.write(HazelResponse::UCIResponse(UCIMessage::ReadyOk));
            },
            UCIMessage::SetOption(name, value) => {
                witch.state.options.insert(name.clone(), value.clone());
            },
            UCIMessage::UCINewGame => {
                // push position onto the variation in place (creating a variation if necessary),
                // end the game as an abort.

                if witch.state.position.is_some() {

                    let pos = witch.state.position.clone().unwrap();
                    let init = pos.initial;
                    witch.state.game.setup(init);
                    for m in pos.moves.iter() {
                        witch.state.game.make(*m);
                    }
                } else {
                    witch.state.game.setup(BEN::new(START_POSITION_FEN));
                }
            },
            UCIMessage::Position(fen, moves) => {
                let moves = moves.iter().map(|m| UCI::try_from(m).unwrap().into()).collect();
                let ben = BEN::new(fen);
                witch.state.position = Some(Position::new(ben, moves));
            },
            UCIMessage::Go(_) => {
                // for now, we will just statically 'search' by replying with a 'bestmove' based on
                // a random move, right now stubbing in a null move with no ponder
                witch.write(HazelResponse::UCIResponse(UCIMessage::Info(vec!["Status".to_string(), "WIP".to_string()])));
                witch.write(HazelResponse::UCIResponse(UCIMessage::BestMove("0000".to_string(), None)));
            },
            _ => {
                error!("Unsupported UCI Message: {:?}", self);
            }
        }
    }
}
