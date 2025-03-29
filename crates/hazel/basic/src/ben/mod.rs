//! Binary Encoding Notation
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

use crate::{color::Color, interface::{Alter, Alteration, Query}, occupant::Occupant, piece::Piece, position_metadata::PositionMetadata, square::Square};

use std::fmt::{Debug, Formatter};

#[derive(Default, PartialEq, Clone, Copy)]
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

    fn try_metadata(&self) -> Option<PositionMetadata> {
        Some(self.metadata)
    }
}

impl Alter for BEN {
    fn alter(&self, alter: Alteration) -> Self {
        let mut ben = *self;
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
                    self.position[byte_index] &= 0b00001111;
                } else {
                    self.position[byte_index] &= 0b11110000;
                }
            },
            Alteration::Clear => { self.position = [0; 32]; },
            Alteration::Assert(_) | Alteration::Inform(_) => { self.metadata.alter_mut(alter); },
            _ => { }
        }

        self
    }
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

impl std::fmt::Display for BEN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::interface::query::to_fen_position(self))
    }
}


impl BEN {
    pub fn new(pos: &str) -> Self {
        let alterations = Self::compile(pos);
        let mut ret = Self::empty();
        for alter in alterations {
            ret.alter_mut(alter);
        }
        ret
    }

    pub fn to_alterations(&self) -> impl Iterator<Item = Alteration> {
        crate::interface::query::to_alterations(self)
    }

    // TODO: Move this to it's own function, it should produce a _Log_ of alteratons
    // TODO: Nom.
    fn compile(fen: &str) -> impl Iterator<Item = Alteration> {
        let mut alterations = vec![];
        let mut cursor = Square::by_rank_and_file();
        cursor.downward();
        let mut chunks = fen.split_whitespace();

        let configuration = chunks.next().expect("Invalid position configuration");

        for c in configuration.chars() {
            if cursor.is_done() { break; }

            match c {
                '1'..='8' => {
                    let skip = c.to_digit(10).unwrap() as usize;
                    for _ in 0..skip { cursor.next(); }
                }
                '/' => {
                    continue;
                }
                c => {
                    let color = if c.is_uppercase() { Color::WHITE } else { Color::BLACK };
                    let piece = match c.to_ascii_lowercase() {
                        'p' => Piece::Pawn,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'r' => Piece::Rook,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => {
                            continue;
                        },
                    };
                    let occupant = Occupant::Occupied(piece, color);
                    alterations.push(Alteration::Place { square: cursor.current_square(), occupant } );

                    cursor.next();
                }
            }
        }

        let mut metadata = PositionMetadata::default();
        metadata.parse(&mut chunks);
        let metadata_alterations : Vec<Alteration> = metadata.into_information();
        alterations.extend(metadata_alterations);

        alterations.into_iter()
    }

    pub fn start_position() -> Self {
        Self::new(crate::constants::START_POSITION_FEN)
    }

    pub fn empty() -> Self {
        Self {
            position: [0; 32],
            metadata: PositionMetadata::default()
        }
    }

    // FIXME: This feels like a bug. Probably where-ever I use this is a bug.
    pub fn with_default_metadata(fen: &str) -> Self {
        let mut ret = Self::new(fen);
        ret.set_metadata(PositionMetadata::default());
        ret
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
    use super::*;
    use crate::{file::File, square::*};
    use crate::castle_rights::CastleRights;

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

    // #[quickcheck]
    // fn get_mut(sq: Square, piece: Piece, color: Color) {
    //     let mut ben = BEN::new();
    //     let ben_sq = ben.get_mut(sq);
    //     ben_sq = Occupant::white_pawn().into();
    //     assert_eq!(ben.get(sq), Occupant::white_pawn().into());
    // }


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


