use std::fmt::Display;
use std::str::SplitWhitespace;

use tracing::debug;

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

const STM_MASK: u32 = 0b1;
const STM_SHIFT: usize = 0;
const CASTLING_MASK: u32 = 0b1111;
const CASTLING_SHIFT: usize = 1;
const EP_FLAG_MASK: u32 = 0b0001;
const EP_FILE_MASK: u32 = 0b1110;
const EP_SHIFT: usize = 5;
const HMC_MASK: u32 = 0b111111;
const HMC_SHIFT: usize = 9;
const FMN_SHIFT: usize = 16;

impl PositionMetadata {
    pub fn parse(&mut self, parts: &mut SplitWhitespace<'_>) {
        let side_to_move = parts.next();
        let castling = parts.next();
        let en_passant = parts.next();
        let halfmove_clock = parts.next();
        let fullmove_number = parts.next();

        debug!("Side to move: {:?}", side_to_move);
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



    pub fn compile(&self) -> Vec<u8> {
        u32::from(*self).to_ne_bytes().into()
    }
}

impl From<PositionMetadata> for u32 {
    fn from(data: PositionMetadata) -> Self {
        let mut ret : u32 = 0;

        ret |= (data.side_to_move as u32) << STM_SHIFT;
        ret |= u32::from(data.castling) << CASTLING_SHIFT;
        ret |= (data.halfmove_clock as u32) << HMC_SHIFT;
        ret |= (data.fullmove_number as u32) << FMN_SHIFT;

        match data.en_passant {
            None => {} // no need to do anything
            Some(sq) => {
                let file = File::from_index(sq.file());
                ret |= 1 << EP_SHIFT;
                ret |= (file as u32) << (EP_SHIFT + 1);
            }
        }

        ret
    }

}

impl From<u32> for PositionMetadata {
    fn from(data: u32) -> Self {
        let side_to_move = (data >> STM_SHIFT) & STM_MASK;
        let castling = (data >> CASTLING_SHIFT) & CASTLING_MASK;
        let ep_flag = (data >> EP_SHIFT) & EP_FLAG_MASK;
        let ep_flag_bit = (data >> (EP_SHIFT + 1)) & EP_FILE_MASK;
        let ep_file = File::from_index(ep_flag_bit as usize);
        let hmc : u8 = ((data >> HMC_SHIFT) & HMC_MASK) as u8;
        let fmn : u16 = (data >> FMN_SHIFT) as u16;

        let side_to_move = match side_to_move {
            0 => Color::WHITE,
            1 => Color::BLACK,
            _ => panic!("Invalid side to move"),
        };

        let en_passant = (ep_flag != 0).then(|| Square::new(ep_file as usize));

        let castling = CastleRights {
            white_short: castling & 0b1000 != 0,
            white_long: castling & 0b0100 != 0,
            black_short: castling & 0b0010 != 0,
            black_long: castling & 0b0001 != 0,
        };


        Self {
            side_to_move,
            castling,
            en_passant,
            halfmove_clock: hmc,
            fullmove_number: fmn,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

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
}
