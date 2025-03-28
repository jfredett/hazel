mod error;
mod message_for;
mod witchhandle;
mod inner;

pub use inner::*;
pub use error::*;
pub use witchhandle::*;
pub use message_for::*;

pub type MessageForWitch<const BUF_SIZE: usize, S, R> = Box<dyn message_for::MessageFor<Witch<BUF_SIZE, S, R>>>;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::broadcast;

    // The simplest kind of message is a 0-length struct that tells the Witch to simply do a thing.
    mod simple_struct_messages {
        use super::*;

        struct TestMessage;
        #[async_trait::async_trait]
        impl MessageFor<Witch<10, i32, i32>> for TestMessage {
            async fn run(&self, witch: &mut Witch<10, i32, i32>) {
                witch.write(10);
            }
        }

        #[tokio::test]
        async fn test_witch() {
            // NOTE: this should _maybe_ be a method on Witch proper? or a module function?
            let w = WitchHandle::<10, i32, i32>::new().await;

            // NOTE: Perhaps there should be a method on message-for that takes a message and wraps
            // it in a box, if only to hide the abstraction a little.
            w.send(Box::new(TestMessage)).await;

            assert_eq!(w.read().await, Some(10));
        }
    }

    // Messages can contain arbitrary internal state, implement arbitrary internal methods, and for
    // the subset of Witches where the type of messages is also the type of responses, could be
    // sent _back_ with updated internal state.
    mod stateful_messages {
        //TODO
    }

    // Additionally, messages can update the _internal state of the Witch itself_.
    mod state_updating_messages {
        use super::*;

        struct StateUpdatingMessage;
        #[async_trait::async_trait]
        impl MessageFor<Witch<10, i32, i32>> for StateUpdatingMessage {
            async fn run(&self, witch: &mut Witch<10, i32, i32>) {
                witch.state = 10;
            }
        }

        struct ReadStateMessage;
        #[async_trait::async_trait]
        impl MessageFor<Witch<10, i32, i32>> for ReadStateMessage {
            async fn run(&self, witch: &mut Witch<10, i32, i32>) {
                witch.write(witch.state);
            }
        }

        #[tokio::test]
        async fn test_witch() {
            let w = WitchHandle::<10, i32, i32>::new().await;

            w.send(Box::new(StateUpdatingMessage)).await;
            w.send(Box::new(ReadStateMessage)).await;

            assert_eq!(w.read().await, Some(10));
            // assert_eq!(w.read().await, Err(broadcast::error::RecvError::Closed));
        }

    }

    // Struct/Enum, not special. If it makes sense to enumify it, that's fine, just implement
    // MessageFor
    mod enum_messages {
        use super::*;

        enum Command {
            Incr,
            Decr,
            GetState
        }

        #[async_trait::async_trait]
        impl MessageFor<Witch<10, i32, i32>> for Command {
            async fn run(&self, witch: &mut Witch<10, i32, i32>) {
                match self {
                    Command::Incr => witch.state += 1,
                    Command::Decr => witch.state -= 1,
                    Command::GetState => witch.write(witch.state),
                }
            }
        }

        struct ExampleInitMessage;
        #[async_trait::async_trait]
        impl MessageFor<Witch<10, i32, i32>> for ExampleInitMessage {
            async fn run(&self, witch: &mut Witch<10, i32, i32>) {
                witch.state = 100;
            }
        }

        #[tokio::test]
        async fn test_state_initialization() {
            let w = WitchHandle::<10, i32, i32>::with_initialization_message(Box::new(ExampleInitMessage)).await;

            w.send(Box::new(Command::GetState)).await;
            assert_eq!(w.read().await, Some(100));

            w.send(Box::new(Command::Incr)).await;
            w.send(Box::new(Command::Incr)).await;
            w.send(Box::new(Command::Decr)).await;
            w.send(Box::new(Command::GetState)).await;
            assert_eq!(w.read().await, Some(101));
        }

    }
}
