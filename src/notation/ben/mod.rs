//! Binary FEN Notation
//!
//! 64, 4 bit nibbles encoding piece and color, placed in order from A1 to H8.
//! Followed by all relevant positional information in 64/2 + 4 = 36 bytes.
//!
//! This could probably be smaller if I phrased it as a bitfield with some kind of 'skip' byte, but
//! I think I can make that optimization silently later.
//!
//! BEN is going to be the main way I embed a whole context into whatever needs a whole context,
//! and I expect that means there will be a lot of BENs floating around, so every byte counts.
//!
//! I am certain there exists and I am sure I have seen something like this on the internet before,
//! but this seemed the most natural way to do it to me, whether that's because I'm a genius or
//! because I've seen it before, I don't know, but I'm very likely not a genius.

use super::{fen::{PositionMetadata, FEN}, Square};
use crate::{board::{Alter, Alteration, PieceBoard, Query}, types::{Color, Occupant}};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BEN {
    position: [u8; 32],
    metadata: PositionMetadata
}

impl Query for BEN {
    fn get(&self, square: impl Into<Square>) -> Occupant {
        let sq : usize = square.into().index();
        let byte_index = sq / 2;
        let occupant_nibble = if sq % 2 == 0 {
            self.position[byte_index] >> 4
        } else {
            self.position[byte_index] & 0b00001111
        };
        Occupant::from(occupant_nibble)
    }
}

impl Alter for BEN {
    fn alter(&self, alter: Alteration) -> Self {
        let mut ben = self.clone();
        ben.alter_mut(alter);
        ben
    }

    fn alter_mut(&mut self, alter: Alteration) -> &mut Self {
        match alter {
            Alteration::Place { square, occupant } => {
                let sq : usize = square.index();
                let byte_index = sq / 2;
                let nibble : u8 = occupant.into();
                if sq % 2 == 0 {
                    self.position[byte_index] = (nibble << 4) | (self.position[byte_index] & 0b00001111);
                } else {
                    self.position[byte_index] = nibble | (self.position[byte_index] & 0b11110000);
                }
            },
            Alteration::Remove { square, .. } => {
                let sq : usize = square.index();
                let byte_index = sq / 2;
                if sq % 2 == 0 {
                    self.position[byte_index] = self.position[byte_index] & 0b00001111;
                } else {
                    self.position[byte_index] = self.position[byte_index] & 0b11110000;
                }
            },
            Alteration::Clear => { self.position = [0; 32]; },
            _ => { }
        }

        self
    }
}


impl BEN {
    pub fn new() -> Self {
        Self {
            position: [0; 32],
            metadata: PositionMetadata::default()
        }
    }

    pub fn with_metadata(metadata: PositionMetadata) -> Self {
        Self {
            position: [0; 32],
            metadata
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

impl From<FEN> for BEN {
    fn from(fen: FEN) -> Self {
        let mut ben = BEN::new();
        let mut idx = 0;
        let mut squares = Square::by_rank_and_file();

        while !squares.is_done() {
            let lower_square = squares.next().unwrap();
            let upper_square = squares.next().unwrap();

            let lower_occupant : u8 = fen.get(lower_square).into();
            let upper_occupant : u8 = fen.get(upper_square).into();

            ben.position[idx] = (lower_occupant << 4) | upper_occupant;
            idx += 1;
        }

        ben.metadata = fen.metadata();

        ben
    }
}

impl From<BEN> for FEN {
    fn from(ben: BEN) -> Self {
        let mut pb = PieceBoard::default();
        let mut idx = 0;
        let mut squares = Square::by_rank_and_file();

        while !squares.is_done() {
            let lower_square = squares.next().unwrap();
            let upper_square = squares.next().unwrap();

            let lower_occupant = ben.position[idx] >> 4;
            let upper_occupant = ben.position[idx] & 0b00001111;

            pb.alter_mut(Alteration::place(lower_square, lower_occupant.into()));
            pb.alter_mut(Alteration::place(upper_square, upper_occupant.into()));

            idx += 1;
        }

        let mut fen : FEN = pb.into();
        fen.set_metadata(ben.metadata);
        fen
    }
}
