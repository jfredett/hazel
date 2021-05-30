use crate::bitboard::*;
use std::fmt::{Formatter, Result, Debug};

#[cfg(not(tarpaulin_include))]
impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for x in 0..8 {
            for y in 0..8 {
                if self.is_set(7-x,y) {
                    f.write_str("*")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        write!(f, "\n")
    }
}
