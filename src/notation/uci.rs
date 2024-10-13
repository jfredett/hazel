use std::convert::TryFrom;
use crate::notation::square::*;
use crate::coup::rep::{Move, MoveType};
use crate::game::interface::Chess;

use super::MoveNotation;

/// Represents a move in UCI format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UCI {
    source: Square,
    target: Square,
    metadata: Option<MoveType>
}



impl UCI {

    /// Converts the UCI move to PGN format.
    /// TODO: Is this a From/To?
    pub fn to_pgn<C>(&self, context: C) -> String where C : Chess {
        todo!()
        /*
        let m = Move::from(self);
        m.to_pgn()
        */
    }
}

impl From<UCI> for Move {
    fn from(uci: UCI) -> Self {
        todo!()
        /*
        Move::from(uci.source, uci.target, uci.metadata)
        */
    }
}

impl From<Move> for UCI {
    fn from(mov: Move) -> Self {
        Self {
            source: mov.source_idx().try_into().unwrap(),
            target: mov.target_idx().try_into().unwrap(),
            metadata: Some(mov.move_metadata())
        }
    }
}

impl TryFrom<&str> for UCI {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err(());
        }

        let source = Square::try_from(value[0..2].as_bytes()).map_err(|_| ())?;
        let target = Square::try_from(value[2..4].as_bytes()).map_err(|_| ())?;

        Ok(Self {
            source,
            target,
            metadata: None
        })
    }
}

impl MoveNotation for UCI {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_pawn() {
        let uci = UCI::try_from("e2e4").unwrap();
        assert_eq!(uci.source, E2);
        assert_eq!(uci.target, E4);
        assert_eq!(uci.metadata, None);
    }

    #[test]
    fn invalid() {
        assert!(UCI::try_from("e2e").is_err());
        assert!(UCI::try_from("e2e44").is_err());
        assert!(UCI::try_from("e2e44").is_err());
        assert!(UCI::try_from("e2e4e").is_err());
    }

    #[ignore] // WIP
    #[test]
    fn promotion() {
        let uci = UCI::try_from("e7e8q").unwrap();
        assert_eq!(uci.source, E7);
        assert_eq!(uci.target, E8);
        assert_eq!(uci.metadata, None);
    }

    #[ignore] // WIP
    #[test]
    fn promotion_invalid() {
        assert!(UCI::try_from("e7e8").is_err());
        assert!(UCI::try_from("e7e8q1").is_err());
    }

    #[test]
    fn short_castling() {
        let uci = UCI::try_from("e1g1").unwrap();
        assert_eq!(uci.source, E1);
        assert_eq!(uci.target, G1);
    }

    #[test]
    fn long_castling() {
        let uci = UCI::try_from("e1c1").unwrap();
        assert_eq!(uci.source, E1);
        assert_eq!(uci.target, C1);
    }
}

