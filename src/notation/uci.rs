use std::convert::TryFrom;
use crate::notation::square::*;
use crate::coup::rep::{Move, MoveType};
use crate::types::Piece;

use super::MoveNotation;

/// Represents a move in UCI format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UCI {
    source: Square,
    target: Square,
    promotion_piece: Option<Piece>,
    metadata: MoveType
}

impl From<UCI> for Move {
    fn from(uci: UCI) -> Self {
        Move::new(
            uci.source,
            uci.target,
            uci.metadata
        )
    }
}

impl From<Move> for UCI {
    fn from(mov: Move) -> Self {
        let promotion_piece = mov.move_metadata().promotion_piece();
        Self {
            source: mov.source(),
            target: mov.target(),
            promotion_piece,
            metadata: mov.move_metadata()
        }
    }
}

impl TryFrom<String> for UCI {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for UCI {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for UCI {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 4 || value.len() > 5 {
            return Err(());
        }

        let source = Square::try_from(value[0..2].as_bytes()).map_err(|_| ())?;
        let target = Square::try_from(value[2..4].as_bytes()).map_err(|_| ())?;

        let promotion_piece = match value.get(4..5) {
            Some("q") => Some(Piece::Queen),
            Some("r") => Some(Piece::Rook),
            Some("b") => Some(Piece::Bishop),
            Some("n") => Some(Piece::Knight),
            _ => {
                if value.len() == 5 {
                    return Err(()) // if we have 5 characters, we _must_ have a promotion
                } else {
                    None
                }
            }
        };

        Ok(Self {
            source,
            target,
            promotion_piece,
            metadata: MoveType::UCI_AMBIGUOUS
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
        assert_eq!(uci.promotion_piece, None);
        assert_eq!(uci.metadata, MoveType::UCI_AMBIGUOUS);
    }

    #[test]
    fn invalid() {
        assert!(UCI::try_from("e2e").is_err());
        assert!(UCI::try_from("e2e44").is_err());
        assert!(UCI::try_from("e2e44").is_err());
        assert!(UCI::try_from("e2e4e").is_err());
    }

    #[test]
    fn promotion() {
        let uci = UCI::try_from("e7e8q").unwrap();
        assert_eq!(uci.source, E7);
        assert_eq!(uci.target, E8);
        assert_eq!(uci.promotion_piece, Some(Piece::Queen));  
        assert_eq!(uci.metadata, MoveType::UCI_AMBIGUOUS);
    }

    #[test]
    fn promotion_invalid() {
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

