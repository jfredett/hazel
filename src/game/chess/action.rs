use std::fmt::{Debug, Formatter};

use super::reason::Reason;
use super::delim::Delim;

// TODO: Constrain these then fix the debug impl
#[derive(PartialEq, Clone)]
pub enum Action<T, S> where T: Clone + PartialEq, S: Clone + PartialEq {
    Halt(Reason),
    Make(T),
    Setup(S),
    Variation(Delim),
}

impl<T, S> Debug for Action<T, S> where T: Clone + PartialEq + Debug, S: Clone + PartialEq + Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Halt(egs) => write!(f, "EndGame({:?})", egs),
            Action::Variation(Delim::Start) => write!(f, "Variation(Start)"),
            Action::Variation(Delim::End) => write!(f, "Variation(End)"),
            Action::Setup(fen) => write!(f, "Setup({:?})", fen),
            Action::Make(mov) => write!(f, "Make({:?})", mov),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}

