use crate::constants::Piece;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MoveType {
    QUIET = 0b0000,
    DOUBLE_PAWN = 0b0001, // NOTE: unused at the moment, according to CPW, usually this indicates a double-pawn move
    SHORT_CASTLE = 0b0010,
    LONG_CASTLE = 0b0011,
    CAPTURE = 0b0100,
    EP_CAPTURE = 0b0101,
    UNUSED_1 = 0b0110,   // NOTE: unused at the moment
    UNUSED_2 = 0b0111,   // NOTE: unused at the moment
    PROMOTION_KNIGHT = 0b1000,
    PROMOTION_BISHOP = 0b1001,
    PROMOTION_ROOK = 0b1010,
    PROMOTION_QUEEN = 0b1011,
    PROMOTION_CAPTURE_KNIGHT = 0b1100,
    PROMOTION_CAPTURE_BISHOP = 0b1101,
    PROMOTION_CAPTURE_ROOK = 0b1110,
    PROMOTION_CAPTURE_QUEEN = 0b1111,
}

impl MoveType {
    pub fn new(bits: u16) -> MoveType {
        // NOTE: This may not be necessary? I think I mask this on the way in.
        match bits & 0b1111u16 {
            0b0000 => MoveType::QUIET,
            0b0001 => MoveType::DOUBLE_PAWN,
            0b0010 => MoveType::SHORT_CASTLE,
            0b0011 => MoveType::LONG_CASTLE,
            0b0100 => MoveType::CAPTURE,
            0b0101 => MoveType::EP_CAPTURE,
            0b0110 => MoveType::UNUSED_1,
            0b0111 => MoveType::UNUSED_2,
            0b1000 => MoveType::PROMOTION_KNIGHT,
            0b1001 => MoveType::PROMOTION_BISHOP,
            0b1010 => MoveType::PROMOTION_ROOK,
            0b1011 => MoveType::PROMOTION_QUEEN,
            0b1100 => MoveType::PROMOTION_CAPTURE_KNIGHT,
            0b1101 => MoveType::PROMOTION_CAPTURE_BISHOP,
            0b1110 => MoveType::PROMOTION_CAPTURE_ROOK,
            0b1111 => MoveType::PROMOTION_CAPTURE_QUEEN,
            _ => unimplemented!(),
        }
    }

    const PROMOTION_MASK: u16 = 0b1000;
    const CAPTURE_MASK: u16 = 0b0100;
    const EP_MASK: u16 = 0b0101;

    #[inline(always)]
    pub fn is_long_castle(self) -> bool {
        self == MoveType::LONG_CASTLE
    }
    #[inline(always)]
    pub fn is_short_castle(self) -> bool {
        self == MoveType::SHORT_CASTLE
    }
    #[inline(always)]
    pub fn is_capture(self) -> bool {
        (self as u16 & Self::CAPTURE_MASK) != 0
    }
    #[inline(always)]
    pub fn is_quiet(self) -> bool {
        self == MoveType::QUIET
    }
    #[inline(always)]
    pub fn is_promotion(self) -> bool {
        (self as u16 & Self::PROMOTION_MASK) != 0
    }

    #[inline(always)]
    pub fn is_en_passant(self) -> bool {
        (self as u16 & Self::EP_MASK) != 0
    }

    // convenience constructors
    #[inline(always)]
    pub fn quiet() -> MoveType {
        MoveType::QUIET
    }
    #[inline(always)]
    pub fn capture() -> MoveType {
        MoveType::CAPTURE
    }
    #[inline(always)]
    pub fn short_castle() -> MoveType {
        MoveType::SHORT_CASTLE
    }
    #[inline(always)]
    pub fn long_castle() -> MoveType {
        MoveType::LONG_CASTLE
    }

    pub fn promotion_piece(&self) -> Option<Piece> {
        // TODO: It may be faster to mask-and-cast the bits, they're arranged such that they correspond to the piece enum.
        // This is the KISS version
        match self {
            MoveType::PROMOTION_KNIGHT => Some(Piece::Knight),
            MoveType::PROMOTION_BISHOP => Some(Piece::Bishop),
            MoveType::PROMOTION_ROOK => Some(Piece::Rook),
            MoveType::PROMOTION_QUEEN => Some(Piece::Queen),
            MoveType::PROMOTION_CAPTURE_KNIGHT => Some(Piece::Knight),
            MoveType::PROMOTION_CAPTURE_BISHOP => Some(Piece::Bishop),
            MoveType::PROMOTION_CAPTURE_ROOK => Some(Piece::Rook),
            MoveType::PROMOTION_CAPTURE_QUEEN => Some(Piece::Queen),
            _ => None,
        }
    }
}
