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
            Action::Halt(egs) => write!(f, "Halt({:?})", egs),
            Action::Variation(Delim::Start) => write!(f, "Variation(Start)"),
            Action::Variation(Delim::End) => write!(f, "Variation(End)"),
            Action::Setup(fen) => write!(f, "Setup({:?})", fen),
            Action::Make(mov) => write!(f, "Make({:?})", mov),
        }
    }
}

#[cfg(test)]
mod tests {
    use ben::BEN;

    use super::*;
    use crate::{constants::START_POSITION_FEN, coup::rep::{Move, MoveType}, notation::*};

    #[test]
    fn debug_formats_correctly() {
        let action : Action<Move, BEN> = Action::Halt(Reason::Stalemate);
        assert_eq!(format!("{:?}", action), "Halt(Stalemate)");

        let action : Action<Move, BEN> = Action::Make(Move::new(A1, A2, MoveType::QUIET));
        assert_eq!(format!("{:?}", action), "Make(a1 (00) -> a2 (08) (QUIET) [0o000200])");

        let action : Action<Move, BEN> = Action::Setup(BEN::new(START_POSITION_FEN));
        assert_eq!(format!("{:?}", action), "Setup(0x42356324111111110000000000000000000000000000000077777777a89bc98a)");

        let action : Action<Move, BEN> = Action::Variation(Delim::Start);
        assert_eq!(format!("{:?}", action), "Variation(Start)");

        let action : Action<Move, BEN> = Action::Variation(Delim::End);
        assert_eq!(format!("{:?}", action), "Variation(End)");
    }
}

