use crate::constants::Piece;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MoveType {
    QUIET = 0b0000,
    DOUBLE_PAWN = 0b0001,
    SHORT_CASTLE = 0b0010,
    LONG_CASTLE = 0b0011,
    CAPTURE = 0b0100,
    EP_CAPTURE = 0b0101,
    UNUSED_1 = 0b0110,   // NOTE: unused at the moment
    ///
    /// UCI sends moves in a simplified long algebraic notation that simply specifies:
    ///
    /// 1. source square
    /// 2. target square
    /// 3. promotion piece (if any)
    ///
    /// For Hazel, more is expected of the move metadata, but calculating it requires knowing the
    /// game state. This value is used to indicate that the move metadata is ambiguous and needs to
    /// be calculated.
    /// 
    UCI_AMBIGUOUS = 0b0111,
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
    pub fn decode(self) -> &'static str {
        match self {
            MoveType::QUIET => "QUIET",
            MoveType::DOUBLE_PAWN => "DOUBLE_PAWN",
            MoveType::SHORT_CASTLE => "SHORT_CASTLE",
            MoveType::LONG_CASTLE => "LONG_CASTLE",
            MoveType::CAPTURE => "CAPTURE",
            MoveType::EP_CAPTURE => "EP_CAPTURE",
            MoveType::UNUSED_1 => "UNUSED_1",
            MoveType::UCI_AMBIGUOUS => "UCI_AMBIGUOUS",
            MoveType::PROMOTION_KNIGHT => "PROMOTION_KNIGHT",
            MoveType::PROMOTION_BISHOP => "PROMOTION_BISHOP",
            MoveType::PROMOTION_ROOK => "PROMOTION_ROOK",
            MoveType::PROMOTION_QUEEN => "PROMOTION_QUEEN",
            MoveType::PROMOTION_CAPTURE_KNIGHT => "PROMOTION_CAPTURE_KNIGHT",
            MoveType::PROMOTION_CAPTURE_BISHOP => "PROMOTION_CAPTURE_BISHOP",
            MoveType::PROMOTION_CAPTURE_ROOK => "PROMOTION_CAPTURE_ROOK",
            MoveType::PROMOTION_CAPTURE_QUEEN => "PROMOTION_CAPTURE_QUEEN",
        }
    }


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
            // NOTE: 
            0b0111 => MoveType::UCI_AMBIGUOUS,
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

    #[inline]
    pub fn from_uci(uci: &str) -> MoveType {
        match uci {
            "q" => MoveType::PROMOTION_QUEEN,
            "r" => MoveType::PROMOTION_ROOK,
            "b" => MoveType::PROMOTION_BISHOP,
            "n" => MoveType::PROMOTION_KNIGHT,
            "" => MoveType::UCI_AMBIGUOUS,
            _ => unimplemented!(),
        }
    }

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

    pub fn is_ambiguous(self) -> bool {
        self == MoveType::UCI_AMBIGUOUS
    }
}
