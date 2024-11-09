use std::fmt::Display;
use std::str::SplitWhitespace;

use crate::board::Query;
use crate::constants::File;
use crate::coup::rep::Move;
use crate::notation::fen::castle_rights::CastleRights;
use crate::notation::*;
use crate::types::{Color, Occupant, Piece};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionMetadata {
    /// Color of the Current Player
    pub side_to_move: Color, // u1
    /// Bitfield of Castle Rights
    pub castling: CastleRights, // u4
    /// The index of the square containing the en passant target square, or None if there is none
    pub en_passant: Option<Square>, // u4 for flag + file, flag might be a halfmove clock of 0 after T1?
    /// The number of halfmoves since the last pawn advance or capture
    pub halfmove_clock: u8, // u6 is enough
    pub fullmove_number: u16, // u16
    //
    // layout of the metadata:
    // 00000000 00000000 00000000 00000000
    // CCCCEEEE HHHHHHSx FFFFFFFF FFFFFFFF
}


impl Default for PositionMetadata {
    fn default() -> Self {
        Self {
            side_to_move: Color::WHITE,
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
                let sq = Square::try_from(square);
                match sq {
                    Ok(sq) => Some(sq),
                    Err(_) => None,
                }
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
            self.side_to_move = Color::BLACK;
        } else {
            self.side_to_move = Color::WHITE;
        }

        // rely on the color of the piece being moved, rather than reasoning about the side-to-move
        // or delaying it till the end.
        let Occupant::Occupied(piece, color) = board.get(mov.source()) else { panic!("Move has no source piece"); };


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
                if mov.is_double_pawn_push_for(color) {
                    self.en_passant = match color {
                        Color::BLACK => mov.target().up(),
                        Color::WHITE => mov.target().down(),
                    }
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
            Some(sq) => (1 << EP_FLAG_SHIFT) | ((sq.file() as u8) << EP_FILE_SHIFT),
        };
        


        b2 |= (data.halfmove_clock as u8) << HMC_SHIFT;
        b2 |= (data.side_to_move as u8) << STM_SHIFT;


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
        let side_to_move = Color::from((b2 & STM_MASK) >> STM_SHIFT);

        // b1 contains the Castling Information and EP square:
        // magic numbers are just shifting off the unused portions.
        let castling = CastleRights::from((b1 & CASTLING_MASK) >> CASTLING_SHIFT);

        let en_passant = if (b1 & EP_FLAG_MASK) != 0 {
            let ep_file_data = (b1 & EP_FILE_MASK) >> EP_FILE_SHIFT;
            let ep_file = File::from_index(ep_file_data as usize);

            Some(match side_to_move {
                // color is the _side to move_, so the EP square would be on the opposite side if
                // it exists
                Color::WHITE => A6.set_file(ep_file as usize),
                Color::BLACK => A3.set_file(ep_file as usize),
            })
        } else {
            None
        };


        // b3 and b4 contain the fullmove number as a u16
        let fullmove_number = u16::from_ne_bytes([b3, b4]);

        assert_eq!(u32::from_ne_bytes([b1, b2, b3, b4]), data);

        Self {
            side_to_move,
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

    impl Arbitrary for PositionMetadata {
        fn arbitrary(g: &mut Gen) -> Self {
            let should_ep = bool::arbitrary(g);
            let color = Color::arbitrary(g);
            let ep_square = if should_ep {
                let file = File::arbitrary(g);
                
                let sq = if color == Color::WHITE {
                    A6.set_file(file as usize)
                } else {
                    A3.set_file(file as usize)
                };

                Some(sq)
            } else {
                None
            };

            Self {
                side_to_move: color,
                castling: CastleRights::arbitrary(g),
                en_passant: ep_square,
                halfmove_clock: u8::arbitrary(g) % 64,
                fullmove_number: u16::arbitrary(g),
            }
        }
    }

    #[quickcheck]
    #[tracing_test::traced_test]
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
        assert_eq!(metadata.en_passant, Some(E3));
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
