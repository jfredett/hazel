use crate::bitboard::*;
use std::fmt::{Formatter, Result, Debug};

#[cfg(not(tarpaulin_include))]
impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("\n")?;
        for y in 0..8 {
            write!(f, " {}", 8 - y)?;
            for x in 0..8 {
                if self.is_set(x,7-y) {
                    write!(f, " *")?;
                } else {
                    write!(f, " .")?;
                }
            }
            f.write_str("\n")?;
        }
        write!(f, "   a b c d e f g h")?;
        write!(f, "\n")
    }
}
