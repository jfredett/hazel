//! Binary Encoding Notation
//!
//! 64, 4 bit nibbles encoding piece and color, placed in order from A1 to H8.
//! Followed by all relevant positional information in 64/2 + 4 = 36 bytes.
//!
//! 4 additional bytes are needed for PositionMetadata
use crate::{color::Color, square::Square};

use std::fmt::{Debug, Formatter};

use crate::position_metadata::PositionMetadata;

mod alter;
mod compile;
mod display;
mod query;

#[derive(Default, PartialEq, Clone, Copy)]
pub struct BEN {
    position: [u8; 32],
    metadata: PositionMetadata
}

impl Debug for BEN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("0x");
        for byte in self.position.iter() {
            s.push_str(&format!("{:02x}", byte));
        }
        s.push(':');
        s.push_str(&format!("{}", self.metadata));

        write!(f, "{}", s)
    }
}


impl BEN {
    pub fn empty() -> Self {
        Self {
            position: [0; 32],
            metadata: PositionMetadata::default()
        }
    }

    pub fn metadata(&self) -> PositionMetadata {
        self.metadata
    }

    pub fn set_metadata(&mut self, metadata: PositionMetadata) {
        self.metadata = metadata;
    }

    pub fn get_mut(&mut self, square: impl Into<Square>) -> &mut u8 {
        let sq : usize = square.into().index();
        let byte_index = sq / 2;
        &mut self.position[byte_index]
    }

    pub fn side_to_move(&self) -> Color {
        self.metadata.side_to_move
    }
}


#[cfg(test)]
mod tests {
    use crate::{castle_rights::CastleRights, interface::{Alter, Alteration, Query}};

    use super::*;
    use crate::{ben::BEN, file::File, occupant::Occupant, square::*};

    #[quickcheck]
    fn alter_mut(square: Square, occupant: Occupant) {
        let mut ben = BEN::empty();

        assert!(ben.get(square) == Occupant::Empty);
        ben.alter_mut(Alteration::place(square, occupant));
        assert!(ben.get(square) == occupant);
        ben.alter_mut(Alteration::remove(square, occupant));
        assert!(ben.get(square) == Occupant::Empty);
    }

    #[test]
    fn alter() {
        let ben = BEN::empty();
        let ben = ben.alter(Alteration::Place { square: A1, occupant: Occupant::white_pawn() });
        let ben = ben.alter(Alteration::Place { square: H8, occupant: Occupant::black_king() });
        let ben = ben.alter(Alteration::Place { square: H1, occupant: Occupant::white_queen() });
        let ben = ben.alter(Alteration::Place { square: A8, occupant: Occupant::black_knight() });

        assert_eq!(ben.get(A1), Occupant::white_pawn());
        assert_eq!(ben.get(H8), Occupant::black_king());
        assert_eq!(ben.get(H1), Occupant::white_queen());
        assert_eq!(ben.get(A8), Occupant::black_knight());

        let ben = ben.alter(Alteration::Remove { square: A1, occupant: Occupant::white_pawn() });
        let ben = ben.alter(Alteration::Remove { square: H8, occupant: Occupant::black_king() });
        let ben = ben.alter(Alteration::Remove { square: H1, occupant: Occupant::white_queen() });
        let ben = ben.alter(Alteration::Remove { square: A8, occupant: Occupant::black_knight() });

        assert_eq!(ben.get(A1), Occupant::Empty);
        assert_eq!(ben.get(H8), Occupant::Empty);
        assert_eq!(ben.get(H1), Occupant::Empty);
        assert_eq!(ben.get(A8), Occupant::Empty);
    }

    #[test]
    fn metadata() {
        let mut ben = BEN::empty();
        assert_eq!(ben.metadata(), PositionMetadata::default());

        let metadata = PositionMetadata {
            side_to_move: Color::BLACK,
            in_check: false,
            castling: CastleRights::default(),
            en_passant: Some(File::A),
            halfmove_clock: 0,
            fullmove_number: 1
        };

        ben.set_metadata(metadata);
        assert_eq!(ben.metadata(), metadata);
    }
}


