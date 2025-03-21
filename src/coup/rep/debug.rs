use super::*;
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
                "{} ({:02}) -> {} ({:02}) ({}{}) [0o{:06o}]",
                self.source(),
                self.source_idx(),
                self.target(),
                self.target_idx(),
                self.move_metadata().decode(),
                if self.is_promotion() { ", P" } else { "" },
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
    fn display_displays_as_intended() {
        let m = Move::from_notation("d2", "d4", MoveType::QUIET);
        let debug_out = format!("{}", m);
        assert_eq!(debug_out, "d2 (11) -> d4 (27) (QUIET) [0o026660]");
    }

    #[test]
    fn debug_displays_as_intended() {
        let m = Move::from_notation("d2", "d4", MoveType::QUIET);
        let debug_out = format!("{:?}", m);
        assert_eq!(debug_out, "d2 (11) -> d4 (27) (QUIET) [0o026660]");
    }
}
