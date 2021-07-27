use std::fmt::Debug;

use super::*;

impl Debug for Game {
    #[cfg(test)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.position)?;
        writeln!(f, "{:?}", self.position.meta)?;
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
    
    #[cfg(not(test))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.position)?;
        writeln!(f, "{:?}", self.position.meta)?;
        writeln!(f)?;

        for (i, m) in self.played.iter().enumerate() {
            writeln!(f, "{}. {:?}", (i / 2) + 1, m)?;
        }
        
        Ok(())
    }
}