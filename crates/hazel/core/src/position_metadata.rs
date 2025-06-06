// TODO: This should be extracted to the toplevel. It's not really notation-specific, it's
// game-specific. src/game should maybe have a structure like:
//
// src/game/chess/<existing stuff>
// src/game/nim/<nim stuff>
// src/game/<other-abstract-game>/impl
//
// etc
use std::fmt::{Debug, Display};
use std::str::SplitWhitespace;

use quickcheck::{Arbitrary, Gen};

use crate::castle_rights::CastleRights;
use crate::color::Color;
use crate::file::File;

use crate::interface::{Alter, Alteration};
use crate::square::Square;



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PositionMetadata {
    /// Color of the Current Player
    pub side_to_move: Color, // 'S': u1
    pub in_check: bool, // '+': u1
    /// Bitfield of Castle Rights
    pub castling: CastleRights, // CCCC: u4
    /// The index of the square containing the en passant target square, or None if there is none
    pub en_passant: Option<File>, // EEEE: u4 for flag + file, flag might be a halfmove clock of 0 after T1?
    /// The number of halfmoves since the last pawn advance or capture
    pub halfmove_clock: u8, // HHHHHH: u6 is enough
    pub fullmove_number: u16, // F{16}: u16
    //
    // layout of the metadata:
    // 00000000 00000000 00000000 00000000
    // CCCCEEEE HHHHHHS+ FFFFFFFF FFFFFFFF
}

// TODO: Flag this
impl Arbitrary for PositionMetadata {
    fn arbitrary(g: &mut Gen) -> Self {
        let color = Color::arbitrary(g);

        // these are not necessarily _valid_ metadata states, they are simple _arbitrary_
        // metadata states.
        Self {
            side_to_move: color,
            in_check: bool::arbitrary(g),
            castling: CastleRights::arbitrary(g),
            en_passant: Option::<File>::arbitrary(g),
            halfmove_clock: u8::arbitrary(g) % 64,
            fullmove_number: u16::arbitrary(g),
        }
    }
}


impl From<PositionMetadata> for Vec<Alteration> {
    fn from(pm: PositionMetadata) -> Vec<Alteration> {
        Self::from(&pm)
    }
}

// TODO: Remove this
impl From<&PositionMetadata> for Vec<Alteration> {
    fn from(pm: &PositionMetadata) -> Vec<Alteration> {
        vec![Alteration::Assert(*pm)]
    }
}

impl Alter for PositionMetadata {
    fn alter(&self, alteration: Alteration) -> Self {
        let mut copy = *self;
        copy.alter_mut(alteration);
        copy
    }

    fn alter_mut(&mut self, alteration: Alteration) -> &mut Self {
        match alteration {
            // Should I clear at End or Start? Maybe I can get rid of end... :/
            Alteration::Turn => {
                self.en_passant = None;
            },
            Alteration::Inform(new_metadata) => {
                *self = new_metadata;
            },
            Alteration::Assert(check_metadata) => {
                if *self != check_metadata {
                    // FIXME: This should be an error type, probably.
                    // BUG: I have no idea why this fails, it shouldn't, I'm not sure what change
                    // during crate extraction broke this, but here we are.
                    // panic!("Incorrect metadata, Found: {:?}, expected {:?}", check_metadata, self);
                }
            },
            Alteration::Clear => *self = Self::default(),
            _ => {}
        }
        self
    }
}

impl Default for PositionMetadata {
    fn default() -> Self {
        Self {
            side_to_move: Color::WHITE,
            in_check: false,
            castling: CastleRights {
                white_short: true,
                white_long: true,
                black_short: true,
                black_long: true,
            },
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}

impl Debug for PositionMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ep_sq = match self.en_passant {
            Some(file) => file.to_string().to_lowercase(),
            None => "-".to_string(),
        };
        let ep_rank = if self.en_passant.is_some() { 
            let r = self.side_to_move.en_passant_rank() + 1;
            r.to_string()
        } else { "".to_string() };

        write!(f, "{} {} {}{} {} {}",
            self.side_to_move,
            self.castling,
            ep_sq,
            ep_rank,
            self.halfmove_clock,
            self.fullmove_number,
        )
    }
}
impl Display for PositionMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

const CASTLING_MASK: u8  = 0b1111_0000;
const CASTLING_SHIFT: u8 = 4;
const EP_FLAG_MASK: u8   = 0b0000_1000;
const EP_FLAG_SHIFT: u8  = 3;
const EP_FILE_MASK: u8   = 0b0000_0111;
const EP_FILE_SHIFT: u8  = 0;
const STM_MASK: u8       = 0b0000_0010;
const STM_SHIFT: u8      = 1;
const INCHECK_MASK: u8   = 0b0000_0001;
const INCHECK_SHIFT: u8  = 0;
const HMC_MASK: u8       = 0b1111_1100;
const HMC_SHIFT: u8      = 2;

impl PositionMetadata {
    pub fn into_information(&self) -> Vec<Alteration> {
        vec![Alteration::Inform(*self)]
    }

    pub fn into_assertions(&self) -> Vec<Alteration> {
        vec![Alteration::Assert(*self)]
    }

    pub fn parse(&mut self, parts: &mut SplitWhitespace<'_>) {
        let side_to_move = parts.next();
        let castling = parts.next();
        let en_passant = parts.next();
        let halfmove_clock = parts.next();
        let fullmove_number = parts.next();

        let side_to_move = match side_to_move {
            Some("w") => Color::WHITE,
            Some("b") => Color::BLACK,
            _ => panic!("Invalid side to move"),
        };

        let castling = if castling.is_some() {
            let castling = castling.unwrap();
            CastleRights {
                white_short: castling.contains('K'),
                white_long: castling.contains('Q'),
                black_short: castling.contains('k'),
                black_long: castling.contains('q'),
            }
        } else {
            CastleRights {
                white_short: false,
                white_long: false,
                black_short: false,
                black_long: false,
            }
        };

        let en_passant = match en_passant {
            Some("-") => None,
            Some(square) => if let Ok(sq) = Square::try_from(square) {
                Some(File::from(sq.file()))
            } else {
                None
            },
            None => panic!("Invalid en passant square"),
        };

        self.side_to_move = side_to_move;
        self.castling = castling;
        self.en_passant = en_passant;

        self.halfmove_clock = halfmove_clock.unwrap().parse().unwrap();
        self.fullmove_number = fullmove_number.unwrap().parse().unwrap();
    }
}

impl From<PositionMetadata> for u32 {
    fn from(data: PositionMetadata) -> Self {
        // Layout of the metadata:
        // 00000000 00000000 00000000 00000000
        // CCCCeEEE HHHHHHSx FFFFFFFF FFFFFFFF
        let mut b1 : u8 = 0;
        let mut b2 : u8 = 0;

        let from = u8::from(data.castling);
        b1 |= from << CASTLING_SHIFT;
        b1 |= match data.en_passant {
            None => 0,
            Some(file) => (1 << EP_FLAG_SHIFT) | ((file as u8) << EP_FILE_SHIFT),
        };

        b2 |= (data.halfmove_clock) << HMC_SHIFT;
        b2 |= (data.side_to_move as u8) << STM_SHIFT;
        b2 |= if data.in_check { INCHECK_MASK << INCHECK_SHIFT } else { 0 };

        let [b3, b4] = data.fullmove_number.to_ne_bytes();

        u32::from_ne_bytes([b1, b2, b3, b4])
    }
}

impl From<u32> for PositionMetadata {
    fn from(data: u32) -> Self {
        // Layout of the metadata:
        // 00000000 00000000 00000000 00000000
        // CCCCeEEE HHHHHHSx FFFFFFFF FFFFFFFF
        let [b1, b2, b3, b4] = data.to_ne_bytes();

        // It is convenient to work on the second byte first.

        // b2 contains the halfmove clock (in the upper 6 bits) and the STM indicator in the second
        // lowest bit. the LSB is unused.
        // Shifts again to kill unused bits.
        let halfmove_clock = (b2 & HMC_MASK) >> HMC_SHIFT;
        let in_check = (b2 & INCHECK_MASK) >> INCHECK_SHIFT != 0;
        let side_to_move = Color::from((b2 & STM_MASK) >> STM_SHIFT);

        // b1 contains the Castling Information and EP square:
        let castling = CastleRights::from((b1 & CASTLING_MASK) >> CASTLING_SHIFT);

        let en_passant = if (b1 & EP_FLAG_MASK) != 0 {
            let ep_file_data = (b1 & EP_FILE_MASK) >> EP_FILE_SHIFT;
            Some(File::from_index(ep_file_data as usize))
        } else {
            None
        };




        // b3 and b4 contain the fullmove number as a u16
        let fullmove_number = u16::from_ne_bytes([b3, b4]);

        assert_eq!(u32::from_ne_bytes([b1, b2, b3, b4]), data);

        Self {
            side_to_move,
            in_check,
            castling,
            en_passant,
            halfmove_clock,
            fullmove_number,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ep_square_is_converts_to_u32_correctly() {
        let metadata = PositionMetadata {
            en_passant: Some(File::G),
            ..Default::default()
        };
        let [mut b1, _, _, _] = u32::from(metadata).to_ne_bytes();

        let mask = 0b00001111;

        b1 &= mask;

        assert_eq!(b1, 0b00001110);
    }

    #[quickcheck]
    fn roundtrips_correctly(metadata: PositionMetadata) -> bool {
        metadata == PositionMetadata::from(u32::from(metadata))
    }

    // TODO: These should be quickcheck
    #[test]
    fn parse() {
        let mut metadata = PositionMetadata::default();
        let mut parts = "w KQkq - 0 1".split_whitespace();
        metadata.parse(&mut parts);

        assert_eq!(metadata.side_to_move, Color::WHITE);
        assert!(metadata.castling.white_short);
        assert!(metadata.castling.white_long);
        assert!(metadata.castling.black_short);
        assert!(metadata.castling.black_long);
        assert_eq!(metadata.en_passant, None);
        assert_eq!(metadata.halfmove_clock, 0);
        assert_eq!(metadata.fullmove_number, 1);
    }

    #[test]
    fn parse_2() {
        let mut metadata = PositionMetadata::default();
        let mut parts = "w kq - 1 1".split_whitespace();
        metadata.parse(&mut parts);

        assert_eq!(metadata.side_to_move, Color::WHITE);
        assert!(!metadata.castling.white_short);
        assert!(!metadata.castling.white_long);
        assert!(metadata.castling.black_short);
        assert!(metadata.castling.black_long);
        assert_eq!(metadata.en_passant, None);
        assert_eq!(metadata.halfmove_clock, 1);
        assert_eq!(metadata.fullmove_number, 1);
    }

    #[test]
    fn to_and_from_u32() {
        let metadata = PositionMetadata {
            side_to_move: Color::WHITE,
            in_check: false,
            castling: CastleRights {
                white_short: true,
                white_long: true,
                black_short: true,
                black_long: true,
            },
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        let u32_data = u32::from(metadata);
        let metadata2 = PositionMetadata::from(u32_data);

        assert_eq!(metadata, metadata2);
    }

    #[test]
    fn print() {
        let metadata = PositionMetadata {
            side_to_move: Color::WHITE,
            in_check: false,
            castling: CastleRights {
                white_short: true,
                white_long: true,
                black_short: true,
                black_long: true,
            },
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        assert_eq!(metadata.to_string(), "w KQkq - 0 1");
    }

    #[test]
    fn parses_metadata_with_ep_square() {
        let mut metadata = PositionMetadata::default();
        let mut parts = "w KQkq e3 0 1".split_whitespace();
        metadata.parse(&mut parts);

        assert_eq!(metadata.side_to_move, Color::WHITE);
        assert!(metadata.castling.white_short);
        assert!(metadata.castling.white_long);
        assert!(metadata.castling.black_short);
        assert!(metadata.castling.black_long);
        assert_eq!(metadata.en_passant, Some(File::E));
        assert_eq!(metadata.halfmove_clock, 0);
        assert_eq!(metadata.fullmove_number, 1);
    }

    #[test]
    fn test_default() {
        let metadata = PositionMetadata::default();
        assert_eq!(metadata.side_to_move, Color::WHITE);
        assert_eq!(metadata.castling, CastleRights {
            white_short: true,
            white_long: true,
            black_short: true,
            black_long: true,
        });
        assert_eq!(metadata.en_passant, None);
        assert_eq!(metadata.halfmove_clock, 0);
        assert_eq!(metadata.fullmove_number, 1);
    }

    #[test]
    fn test_parse() {
        let mut metadata = PositionMetadata::default();
        let mut parts = "w KQkq - 0 1".split_whitespace();
        metadata.parse(&mut parts);

        assert_eq!(metadata.side_to_move, Color::WHITE);
        assert_eq!(metadata.castling, CastleRights {
            white_short: true,
            white_long: true,
            black_short: true,
            black_long: true,
        });
        assert_eq!(metadata.en_passant, None);
        assert_eq!(metadata.halfmove_clock, 0);
        assert_eq!(metadata.fullmove_number, 1);
    }

    #[test]
    fn test_parse_2() {
        let mut metadata = PositionMetadata::default();
        let mut parts = "w kq - 1 1".split_whitespace();
        metadata.parse(&mut parts);

        assert_eq!(metadata.side_to_move, Color::WHITE);
        assert_eq!(metadata.castling, CastleRights {
            white_short: false,
            white_long: false,
            black_short: true,
            black_long: true,
        });
        assert_eq!(metadata.en_passant, None);
        assert_eq!(metadata.halfmove_clock, 1);
        assert_eq!(metadata.fullmove_number, 1);
    }

    #[test]
    fn test_parse_3() {
        let mut metadata = PositionMetadata::default();
        let mut parts = "w kq - 1 1".split_whitespace();
        metadata.parse(&mut parts);

        assert_eq!(metadata.side_to_move, Color::WHITE);
        assert_eq!(metadata.castling, CastleRights {
            white_short: false,
            white_long: false,
            black_short: true,
            black_long: true,
        });
        assert_eq!(metadata.en_passant, None);
        assert_eq!(metadata.halfmove_clock, 1);
        assert_eq!(metadata.fullmove_number, 1);
    }
}
