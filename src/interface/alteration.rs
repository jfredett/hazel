use std::fmt::{Debug, Display};


use crate::constants::File;
use crate::coup::rep::MoveType;
use crate::game::castle_rights::CastleRights;
use crate::game::position_metadata::PositionMetadata;
use crate::types::{Color, Occupant};
use crate::notation::*;

// NOTE: It's interesting to think about commutativity amongst these alterations. In particular if
// I'm trying to build a final representation of something and I want to vectorize that. It would
// be beneficial in some sense to be able to 'sum up' all the alterations, cancelling out whichever
// ones can be canceled, by commuting them around if needed. Turns don't matter if I only care
// about some specific sum of alterations.
//
// I guess what I'm saying is this feels like a monoid, maybe even something group-adjacent, but
// lacking an explicit NOOP instruction/identity.
//
// Assert/Inform have a less commutative structure though, so opposite to place/remove, which
// ultimately cancel each other out, e.g.:
//
// place P @ d4
// remove P @ d2
//
// results in a boardstate equivalent to:
//
// remove P @ d2
// place P @ d4
//
// and further
//
// place P @ d4
// remove P @ d2
// place P @ d5
// remove P @ d4
//
// is exactly equivalent to:
//
// remove P @ d2
// place P @ d5
//
// which you can find by commutting and cancelling like:
//
// remove P @ d2
// place P @ d4
// remove P @ d4
// place P @ d5
//
// Assert/Inform, however, require ordering, since informs have to follow a set of valid asserts.

#[derive(Clone, Copy, PartialEq)]
pub enum Alteration {
    Place { square: Square, occupant: Occupant },
    Remove { square: Square, occupant: Occupant },
    Assert(MetadataAssertion),
    Inform(MetadataAssertion),
    InitialMetadata(PositionMetadata),
    Lit(u8),
    Turn,
    End,
    Clear,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MetadataAssertion {
    CastleRights(CastleRights),
    EnPassant(File),
    SideToMove(Color),
    InCheck,
    MoveType(MoveType),
    FiftyMoveCount(u8),
    FullMoveCount(u16),
}


#[cfg(test)]
impl quickcheck::Arbitrary for MetadataAssertion {
    fn arbitrary(g: &mut quickcheck::Gen) -> MetadataAssertion {
        let variant = usize::arbitrary(g) % 7;
        match variant {
            0 => { MetadataAssertion::CastleRights(CastleRights::arbitrary(g)) },
            1 => { MetadataAssertion::EnPassant(File::arbitrary(g)) },
            2 => { MetadataAssertion::InCheck },
            3 => { MetadataAssertion::SideToMove(Color::arbitrary(g)) },
            4 => { MetadataAssertion::FiftyMoveCount(u8::arbitrary(g) % 50) },
            5 => { MetadataAssertion::FullMoveCount(u16::arbitrary(g)) },
            6 => { MetadataAssertion::MoveType(MoveType::arbitrary(g)) },
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
            Self::InitialMetadata(metadata) => write!(f, "InitialMetadata <{:?}>", metadata),
            Self::Clear => write!(f, "Clear"),
            Self::Lit(byte) => write!(f, "Lit({:x})", byte),
            Self::Turn => write!(f, "Turn"),
            Self::End => write!(f, "End"),
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
            // TODO: Inverse for metadata assertions/informations are needed
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
