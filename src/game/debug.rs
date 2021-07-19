use std::fmt::Debug;

use super::*;

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.position)?;
        writeln!(f, "{}, {}, {:?}", self.position.half_move_clock, self.position.full_move_clock, self.position.meta)?;
        writeln!(f)?;

        for (i, m) in self.played.iter().enumerate() {
            writeln!(f, "{}. {:?}", (i / 2) + 1, m)?;
        }
        
        writeln!(f)?;
        writeln!(f, "History")?;
        writeln!(f)?;
        for (i, m) in self.history.iter().enumerate() {
            writeln!(f, "{}. {:?}", i, m)?;
        }
        
        Ok(())
    }
}