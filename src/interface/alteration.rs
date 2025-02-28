use std::fmt::{Debug, Display};


use crate::constants::File;
use crate::game::castle_rights::CastleRights;
use crate::game::position_metadata::PositionMetadata;
use crate::types::{Color, Occupant};
use crate::notation::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Alteration {
    Place { square: Square, occupant: Occupant },
    Remove { square: Square, occupant: Occupant },
    Assert(MetadataAssertion),
    Lit(u8),
    Clear,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MetadataAssertion {
    CastleRights(CastleRights),
    EnPassant(File),
    StartTurn(Color),
    FiftyMoveCount(u8),
    FullMoveCount(u16),

}

#[cfg(test)]
impl quickcheck::Arbitrary for MetadataAssertion {
    fn arbitrary(g: &mut quickcheck::Gen) -> MetadataAssertion {
        let variant = usize::arbitrary(g) % 5;
        match variant {
            0 => { MetadataAssertion::CastleRights(CastleRights::arbitrary(g)) },
            1 => { MetadataAssertion::EnPassant(File::arbitrary(g)) },
            2 => { MetadataAssertion::StartTurn(Color::arbitrary(g)) },
            3 => { MetadataAssertion::FiftyMoveCount(u8::arbitrary(g) % 50) },
            4 => { MetadataAssertion::FullMoveCount(u16::arbitrary(g)) },
            _ => { unreachable!(); }
        }
    }
}



impl Debug for Alteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Place { square, occupant } => write!(f, "Place {} @ {}", occupant, square),
            Self::Remove { square, occupant } => write!(f, "Remove {} @ {}", occupant, square),
            // it'd be ideal if this dropped a flag with _how to change_ the metadata, not just a
            // copy of the metadata.
            Self::Assert(metadata) => write!(f, "Assert <{:?}>", metadata),
            Self::Clear => write!(f, "Clear"),
            Self::Lit(byte) => write!(f, "Lit({:x})", byte)
        }
    }
}

impl Display for Alteration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Place { square, occupant } => write!(f, "Place {} @ {}", occupant, square),
            Self::Remove { square, occupant } => write!(f, "Remove {} @ {}", occupant, square),
            Self::Assert(metadata) => write!(f, "Assert <{:?}>", metadata),
            Self::Clear => write!(f, "Clear"),
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
