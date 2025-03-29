#![feature(assert_matches)]
pub mod uci;
pub mod driver;

// Spec that Engine adapters must implement to be included in the Hazel UI.
//
// NOTE: `allow(async_fn_in_trait)` is noted to be alright if I don't plan to use this outside my
// library, and I don't. I also don't like the type salad that is the alternative. #justforfun
pub trait Engine<T> {
    /// Take a raw &str, convert it into the message type, return a series of response messages of
    /// the same type
    #[allow(async_fn_in_trait)]
    async fn exec_message(&mut self, message: &str) -> Vec<T>;

    /// Take a message type, return a series of response messages of the same type
    #[allow(async_fn_in_trait)]
    async fn exec(&mut self, message: &T) -> Vec<T>;
}
