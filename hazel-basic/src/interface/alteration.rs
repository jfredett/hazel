use std::fmt::{Debug, Display};
use quickcheck::{Arbitrary, Gen};

use crate::{color::Color, occupant::Occupant, piece::Piece, position_metadata::PositionMetadata, square::*};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Alteration {
    Place { square: Square, occupant: Occupant },
    Remove { square: Square, occupant: Occupant },
    Assert(PositionMetadata),
    Inform(PositionMetadata),
    #[default] Noop,
    Lit(u8),
    Turn,
    Clear,
}

impl Arbitrary for Alteration {
    fn arbitrary(g: &mut Gen) -> Alteration {
        let variant : usize = usize::arbitrary(g) % 6;

        let occupant = Occupant::Occupied(Piece::arbitrary(g), Color::arbitrary(g));

        match variant {
            0 => Self::Place { square: Square::arbitrary(g), occupant },
            1 => Self::Remove { square: Square::arbitrary(g), occupant },
            2 => Self::Assert(PositionMetadata::arbitrary(g)),
            3 => Self::Clear,
            4 => Self::Lit(u8::arbitrary(g)),
            5 => Self::Inform(PositionMetadata::arbitrary(g)),
            _ => { unreachable!(); }
        }
    }
}

impl Debug for Alteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Place { square, occupant } => write!(f, "Place {} @ {}", occupant, square),
            Self::Remove { square, occupant } => write!(f, "Remove {} @ {}", occupant, square),
            Self::Assert(metadata) => write!(f, "Assert <{:?}>", metadata),
            Self::Inform(metadata) => write!(f, "Inform <{:?}>", metadata),
            Self::Clear => write!(f, "Clear"),
            Self::Lit(byte) => write!(f, "Lit({:x})", byte),
            Self::Turn => write!(f, "Turn"),
            Self::Noop => write!(f, "Noop"),
        }
    }
}

impl Display for Alteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

    pub fn inform(meta: &PositionMetadata) -> Self {
        Self::Inform(*meta)
    }

    pub fn assert(meta: &PositionMetadata) -> Self {
        Self::Assert(*meta)
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
            Self::Assert(fact) => Self::Inform(*fact),
            Self::Inform(fact) => Self::Assert(*fact),
            _ => *self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::quickcheck;

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

    #[quickcheck]
    fn inverse_arb(sq: Square, occ: Occupant) -> bool {
        let place = Alteration::place(sq, occ);
        let remove = Alteration::remove(sq, occ);

        place.inverse() == remove && remove.inverse() == place
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
