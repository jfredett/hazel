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

use crate::alteration::MetadataAssertion;
use crate::interface::Query;
use crate::constants::File;
use crate::coup::rep::Move;
use crate::game::chess::castle_rights::CastleRights;
use crate::query::display_board;
use crate::{notation::*, Alter, Alteration};
use crate::types::{Color, Occupant, Piece};

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

impl From<PositionMetadata> for Vec<Alteration> {
    fn from(pm: PositionMetadata) -> Vec<Alteration> {
        Self::from(&pm)
    }
}

impl From<&PositionMetadata> for Vec<Alteration> {
    fn from(pm: &PositionMetadata) -> Vec<Alteration> {
        let mut ret = vec![];
        ret.push(Alteration::Assert(MetadataAssertion::SideToMove(pm.side_to_move)));
        ret.push(Alteration::Assert(MetadataAssertion::CastleRights(pm.castling)));
        if let Some(ep) = pm.en_passant {
            ret.push(Alteration::Assert(MetadataAssertion::EnPassant(ep)));
        }
        ret.push(Alteration::Assert(MetadataAssertion::FiftyMoveCount(pm.halfmove_clock)));
        ret.push(Alteration::Assert(MetadataAssertion::FullMoveCount(pm.fullmove_number)));
        ret
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
            Alteration::InitialMetadata(metadata) => {
                *self = metadata;
            },
            // Should I clear at End or Start? Maybe I can get rid of end... :/
            Alteration::Turn => {
                self.en_passant = None;
            },
            Alteration::Assert(new_metadata) => {
                // TODO: this probably boils down to something that could be done with bit magic

                match new_metadata {
                    MetadataAssertion::SideToMove(color) => {
                        self.side_to_move = color;
                    },
                    MetadataAssertion::InCheck => self.in_check = true,
                    MetadataAssertion::CastleRights(rights) => self.castling = rights,
                    MetadataAssertion::EnPassant(file) => self.en_passant = Some(file),
                    MetadataAssertion::FullMoveCount(count) => self.fullmove_number = count,
                    MetadataAssertion::FiftyMoveCount(count) => self.halfmove_clock = count,
                    _ => { }
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
            Some(file) => file.to_string(),
            None => "-".to_string(),
        };

        write!(f, "{} {} {} {} {}",
            self.side_to_move,
            self.castling,
            ep_sq,
            self.halfmove_clock,
            self.fullmove_number,
        )
    }
}
impl Display for PositionMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ep_sq = match self.en_passant {
            Some(sq) => sq.to_string(),
            None => "-".to_string(),
        };

        write!(f, "{} {} {} {} {}",
            self.side_to_move,
            self.castling,
            ep_sq,
            self.halfmove_clock,
            self.fullmove_number,
        )
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
            Some(square) => {
                if let Ok(sq) = Square::try_from(square) {
                    Some(File::from(sq.file()))
                } else { None }
            },
            None => panic!("Invalid en passant square"),
        };

        self.side_to_move = side_to_move;
        self.castling = castling;
        self.en_passant = en_passant;

        self.halfmove_clock = halfmove_clock.unwrap().parse().unwrap();
        self.fullmove_number = fullmove_number.unwrap().parse().unwrap();
    }

    pub fn update(&mut self, mov: &Move, board: &impl Query) {
        // Clear the EP square, we'll re-set it if necessary later.
        self.en_passant = None;


        if self.side_to_move == Color::WHITE {
            self.fullmove_number += 1;
        }
        self.side_to_move = !self.side_to_move;

        // rely on the color of the piece being moved, rather than reasoning about the side-to-move
        // or delaying it till the end.

        let Occupant::Occupied(piece, color) = board.get(mov.source()) else { panic!("Move has no source piece: {:?}\n on: \n{}", mov, display_board(board)); };


        if mov.is_capture() || piece == Piece::Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        let source = mov.source();
        match piece {
            Piece::King => {
                match color  {
                    Color::WHITE => {
                        self.castling.white_short = false;
                        self.castling.white_long = false;
                    }
                    Color::BLACK => {
                        self.castling.black_short = false;
                        self.castling.black_long = false;
                    }
                }
            }
            Piece::Rook if source == H1 => { self.castling.white_short = false; }
            Piece::Rook if source == H8 => { self.castling.black_short = false; }
            Piece::Rook if source == A1 => { self.castling.white_long = false; }
            Piece::Rook if source == A8 => { self.castling.black_long = false; }
            Piece::Rook => {}
            Piece::Pawn => {
                self.en_passant = if mov.is_double_pawn_push_for(color) {
                    match mov.target().shift(color.pawn_direction()) {
                        Some(target) => { Some(File::from(target.file())) },
                        None => None
                    }
                } else {
                        None
                }
            }
            _ => {}
        }
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

    use quickcheck::{Arbitrary, Gen};
    use tracing::debug;

    impl Arbitrary for PositionMetadata {
        fn arbitrary(g: &mut Gen) -> Self {
            let should_ep = bool::arbitrary(g);
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
