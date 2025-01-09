use std::collections::HashMap;
use std::io;

use async_trait::async_trait;
use tracing::*;

use crate::engine::uci::UCIMessage;
use crate::game::variation::Variation;
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
    game: Variation,
    options: HashMap<String, Option<String>>
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
            _ => {
                panic!("Unsupported Message: {:?}", self);
            }
        }
    }
}
