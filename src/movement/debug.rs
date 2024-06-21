use super::*;
use crate::constants::INDEX_TO_NOTATION;
use std::fmt::{Debug, Display, Formatter, Result};

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.is_short_castle() {
            write!(f, "O-O")
        } else if self.is_long_castle() {
            write!(f, "O-O-O")
        } else {
            write!(
                f,
                "{} ({:02}) -> {} ({:02}) ({}0b{:04b}) [0o{:06o}]",
                INDEX_TO_NOTATION[self.source_idx() as usize],
                self.source_idx(),
                INDEX_TO_NOTATION[self.target_idx() as usize],
                self.target_idx(),
                if self.is_promotion() { "P, " } else { "" },
                self.move_metadata() as u16,
                self.0
            )
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn displays_as_intended() {
        let m = Move::from_notation("d2", "d4", MoveType::QUIET);
        let debug_out = format!("{:?}", m);
        assert_eq!(debug_out, "d2 (11) -> d4 (27) (0b0000) [0o026660]");
    }
}
