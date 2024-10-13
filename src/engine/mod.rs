pub mod uci;
pub mod driver;

// Spec that Engine adapters must implement to be included in the Hazel UI.
pub trait Engine<T> {
    /// Take a raw &str, convert it into the message type, return a series of response messages of
    /// the same type
    fn exec_message(&mut self, message: &str) -> Vec<T>;

    /// Take a message type, return a series of response messages of the same type
    fn exec(&mut self, message: T) -> Vec<T>;
}
