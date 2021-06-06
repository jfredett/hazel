use super::*;
use std::fmt::{Formatter, Result, Debug};
use crate::constants::*;

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({}) -> {} ({})", INDEX_TO_NOTATION[self.source_idx() as usize], self.source_idx(), INDEX_TO_NOTATION[self.target_idx() as usize], self.target_idx())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn displays_as_intended() {
        let m = Move::from_notation("d2", "d4", Either::Left(MoveType::quiet()));
        let debug_out = format!("{:?}", m);
        assert_eq!(debug_out, "d2 (11) -> d4 (27)");
    }
}