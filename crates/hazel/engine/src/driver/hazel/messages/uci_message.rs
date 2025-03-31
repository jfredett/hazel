use async_trait::async_trait;

use hazel_parser::uci::UCI;
use hazel_core::ben::BEN;
use witch::{MessageFor, Witch};
use hazel_representation::game::position::Position;
use crate::uci::UCIMessage;
use crate::driver::hazel::{Hazel, HazelResponse};

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
                // TODO: push position onto the variation in place (creating a variation if necessary),
                // end the game as an abort.

                // FIXME: This is all kinda wrong now.
                //
                // if witch.state.position.is_some() {
                //     let pos = witch.state.position.clone().unwrap();
                //     let init = pos.initial;
                //     witch.state.game.setup(init);
                //     for m in pos.moves.iter() {
                //         witch.state.game.make(*m);
                //     }
                //     // TODO: Calculate the endgame if it's a checkmate, otherwise it's an abort
                //     // for now, just going to say it's an abort.
                //     witch.state.game.halt(Reason::Aborted);
                //     witch.state.game.commit();
                // }

                // witch.state.position = None;
            },
            UCIMessage::Position(fen, moves) => {
                let moves = moves.iter().map(|m| UCI::try_from(m).unwrap().into()).collect();
                let ben = BEN::new(fen);

                witch.state.position = Some(Position::with_moves(ben, moves));
            },
            UCIMessage::Go(_) => {
                // for now, we will just statically 'search' by replying with a 'bestmove' based on
                // a random move, right now stubbing in a null move with no ponder
                witch.write(HazelResponse::UCIResponse(UCIMessage::Info(vec!["Status".to_string(), "WIP".to_string()])));
                witch.write(HazelResponse::UCIResponse(UCIMessage::BestMove("0000".to_string(), None)));
            },
            _ => {
                tracing::error!("Unsupported UCI Message: {:?}", self);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod uci_messages {
        use crate::driver::hazel::GetState;
        use witch::WitchHandle;
        use hazel_core::constants::START_POSITION_FEN;

        use super::*;

        #[tokio::test]
        async fn uci() {
            let w : WitchHandle<10, Hazel, HazelResponse> = WitchHandle::new().await;

            w.send(Box::new(UCIMessage::UCI)).await;
            let result = w.read().await;

            assert_eq!(result, Some(HazelResponse::UCIResponse(UCIMessage::ID("hazel".to_string(), "0.1".to_string()))));
        }

        #[tokio::test]
        async fn is_ready() {
            let w : WitchHandle<10, Hazel, HazelResponse> = WitchHandle::new().await;

            w.send(Box::new(UCIMessage::IsReady)).await;
            let result = w.read().await;

            assert_eq!(result, Some(HazelResponse::UCIResponse(UCIMessage::ReadyOk)));
        }

        #[tokio::test]
        async fn set_option() {
            let w : WitchHandle<10, Hazel, HazelResponse> = WitchHandle::new().await;

            w.send(Box::new(UCIMessage::SetOption("name".to_string(), Some("value".to_string())))).await;
            w.send(Box::new(GetState)).await;
            if let Some(HazelResponse::Debug(result)) = w.read().await {
                assert_eq!(result.options.get("name"), Some(&Some("value".to_string())));
            } else {
                panic!("Expected Debug response");
            }
        }

        // FIXME: This I think is not working as I refactor `Position`
        // #[tokio::test]
        // async fn uci_new_game() {
        //     let w : WitchHandle<10, Hazel, HazelResponse> = WitchHandle::new().await;

        //     w.send(Box::new(UCIMessage::Position(START_POSITION_FEN.to_string(), vec![]))).await;
        //     w.send(Box::new(UCIMessage::UCINewGame)).await;
        //     w.send(Box::new(GetState)).await;
        //     if let Some(HazelResponse::Debug(result)) = w.read().await {
        //         assert_eq!(result.position, None);
        //     } else {
        //         panic!("Expected Debug response");
        //     }
        // }

        #[tokio::test]
        async fn position() {
            let w : WitchHandle<10, Hazel, HazelResponse> = WitchHandle::new().await;

            w.send(Box::new(UCIMessage::Position(START_POSITION_FEN.to_string(), vec![]))).await;
            w.send(Box::new(GetState)).await;
            if let Some(HazelResponse::Debug(result)) = w.read().await {
                assert_eq!(result.position.unwrap().initial, BEN::new(START_POSITION_FEN));
            } else {
                panic!("Expected Debug response");
            }
        }
    }
}
