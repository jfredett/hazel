use std::fmt::{Debug, Display};

use crate::game::position_metadata::PositionMetadata;
use crate::types::Occupant;
use crate::notation::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Alteration {
    Place { square: Square, occupant: Occupant },
    Remove { square: Square, occupant: Occupant },
    Assert(PositionMetadata),
    StartTurn,
    Lit(u8),
    Clear,
}

impl Debug for Alteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Place { square, occupant } => write!(f, "Place {} @ {}", occupant, square),
            Self::Remove { square, occupant } => write!(f, "Remove {} @ {}", occupant, square),
            Self::Assert(metadata) => write!(f, "Assert {:?}", metadata),
            Self::Clear => write!(f, "Clear"),
            Self::StartTurn => write!(f, "StartTurn"),
            Self::Lit(byte) => write!(f, "Lit({:x})", byte)
        }
    }
}

impl Display for Alteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Place { square, occupant } => write!(f, "Place {} @ {}", occupant, square),
            Self::Remove { square, occupant } => write!(f, "Remove {} @ {}", occupant, square),
            Self::Assert(metadata) => write!(f, "Assert {:?}", metadata),
            Self::Clear => write!(f, "Clear"),
            Self::StartTurn => write!(f, "StartTurn"),
            Self::Lit(byte) => write!(f, "Lit({:x})", byte)
        }
    }
}

impl Alteration {
    pub fn place(square: Square, occupant: Occupant) -> Self {
        Self::Place { square, occupant }
    }

    pub fn remove(square: Square, occupant: Occupant) -> Self {
        Self::Remove { square, occupant }
    }

    pub fn tag(byte: u8) -> Self {
        Self::Lit(byte)
    }

    pub fn lit(bytes: &[u8]) -> Vec<Self> {
        bytes.iter().map(|byte| Self::Lit(*byte)).collect()
    }

    pub fn clear() -> Self {
        Self::Clear
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Place { square, occupant } => Self::Remove { square: *square, occupant: *occupant },
            Self::Remove { square, occupant } => Self::Place { square: *square, occupant: *occupant },
            _ => *self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_display_is_correct() {
        let alteration = Alteration::place(A1, Occupant::black_king());
        assert_eq!(format!("{:?}", alteration), "Place k @ a1");

        let alteration = Alteration::remove(A1, Occupant::black_king());
        assert_eq!(format!("{:?}", alteration), "Remove k @ a1");
    }

    #[test]
    fn clear() {
        let alteration = Alteration::clear();
        assert_eq!(alteration, Alteration::Clear);
    }

    #[test]
    fn place() {
        let alteration = Alteration::place(A1, Occupant::black_king());
        assert_eq!(alteration, Alteration::Place { square: A1, occupant: Occupant::black_king() });
    }

    #[test]
    fn remove() {
        let alteration = Alteration::remove(A1, Occupant::black_king());
        assert_eq!(alteration, Alteration::Remove { square: A1, occupant: Occupant::black_king() });
    }

    #[test]
    fn inverse() {
        let place = Alteration::place(A1, Occupant::black_king());
        let remove = Alteration::remove(A1, Occupant::black_king());
        assert_eq!(place.inverse(), remove);
        assert_eq!(remove.inverse(), place);
    }

    #[test]
    fn tag() {
        let alteration = Alteration::tag(0x01);
        assert_eq!(alteration, Alteration::Lit(0x01));
    }

    #[test]
    fn lit() {
        let alterations = Alteration::lit(&[0x01, 0x02, 0x03]);
        assert_eq!(alterations, vec![Alteration::Lit(0x01), Alteration::Lit(0x02), Alteration::Lit(0x03)]);
    }
}
