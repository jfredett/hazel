use hazel_core::piece::Piece;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MoveType {
    QUIET = 0b0000,
    DOUBLE_PAWN = 0b0001,
    SHORT_CASTLE = 0b0010,
    LONG_CASTLE = 0b0011,
    CAPTURE = 0b0100,
    EP_CAPTURE = 0b0101,
    NULLMOVE = 0b0110,
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

#[cfg(test)]
impl quickcheck::Arbitrary for MoveType {
    fn arbitrary(g: &mut quickcheck::Gen) -> MoveType {
        let u = u16::arbitrary(g) & 0b1111;
        MoveType::new(u)
    }
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
            MoveType::NULLMOVE => "NULLMOVE",
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

    pub fn to_uci(&self) -> &'static str {
        match self {
            MoveType::PROMOTION_KNIGHT => "n",
            MoveType::PROMOTION_BISHOP => "b",
            MoveType::PROMOTION_ROOK => "r",
            MoveType::PROMOTION_QUEEN => "q",
            MoveType::PROMOTION_CAPTURE_KNIGHT => "n",
            MoveType::PROMOTION_CAPTURE_BISHOP => "b",
            MoveType::PROMOTION_CAPTURE_ROOK => "r",
            MoveType::PROMOTION_CAPTURE_QUEEN => "q",
            _ => "",
        }
    }

    pub fn is_null(self) -> bool {
        self == MoveType::NULLMOVE
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
            0b0110 => MoveType::NULLMOVE,
            0b0111 => MoveType::UCI_AMBIGUOUS,
            0b1000 => MoveType::PROMOTION_KNIGHT,
            0b1001 => MoveType::PROMOTION_BISHOP,
            0b1010 => MoveType::PROMOTION_ROOK,
            0b1011 => MoveType::PROMOTION_QUEEN,
            0b1100 => MoveType::PROMOTION_CAPTURE_KNIGHT,
            0b1101 => MoveType::PROMOTION_CAPTURE_BISHOP,
            0b1110 => MoveType::PROMOTION_CAPTURE_ROOK,
            0b1111 => MoveType::PROMOTION_CAPTURE_QUEEN,
            _ => unreachable!(),
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

    #[inline(always)]
    pub fn null_move() -> MoveType {
        MoveType::NULLMOVE
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn is_null_works() {
        assert!(MoveType::null_move().is_null());
        assert!(!MoveType::quiet().is_null());
    }

    #[test]
    pub fn is_long_castle_works() {
        assert!(MoveType::long_castle().is_long_castle());
        assert!(!MoveType::short_castle().is_long_castle());
    }

    #[test]
    pub fn is_short_castle_works() {
        assert!(MoveType::short_castle().is_short_castle());
        assert!(!MoveType::long_castle().is_short_castle());
    }

    #[test]
    pub fn is_capture_works() {
        assert!(MoveType::capture().is_capture());
        assert!(!MoveType::quiet().is_capture());
    }

    #[test]
    pub fn is_quiet_works() {
        assert!(MoveType::quiet().is_quiet());
        assert!(!MoveType::capture().is_quiet());
    }

    mod promotion_piece {
        use super::*;

        #[test]
        pub fn promotion_piece_works() {
            assert_eq!(MoveType::PROMOTION_KNIGHT.promotion_piece(), Some(Piece::Knight));
            assert_eq!(MoveType::PROMOTION_BISHOP.promotion_piece(), Some(Piece::Bishop));
            assert_eq!(MoveType::PROMOTION_ROOK.promotion_piece(), Some(Piece::Rook));
            assert_eq!(MoveType::PROMOTION_QUEEN.promotion_piece(), Some(Piece::Queen));
            assert_eq!(MoveType::PROMOTION_CAPTURE_KNIGHT.promotion_piece(), Some(Piece::Knight));
            assert_eq!(MoveType::PROMOTION_CAPTURE_BISHOP.promotion_piece(), Some(Piece::Bishop));
            assert_eq!(MoveType::PROMOTION_CAPTURE_ROOK.promotion_piece(), Some(Piece::Rook));
            assert_eq!(MoveType::PROMOTION_CAPTURE_QUEEN.promotion_piece(), Some(Piece::Queen));
        }

        #[test]
        pub fn promotion_piece_none() {
            assert_eq!(MoveType::QUIET.promotion_piece(), None);
            assert_eq!(MoveType::CAPTURE.promotion_piece(), None);
            assert_eq!(MoveType::SHORT_CASTLE.promotion_piece(), None);
            assert_eq!(MoveType::LONG_CASTLE.promotion_piece(), None);
            assert_eq!(MoveType::NULLMOVE.promotion_piece(), None);
            assert_eq!(MoveType::UCI_AMBIGUOUS.promotion_piece(), None);
        }
    }

    mod uci {
        use super::*;


        #[test]
        pub fn to_uci_works() {
            assert_eq!(MoveType::PROMOTION_KNIGHT.to_uci(), "n");
            assert_eq!(MoveType::PROMOTION_BISHOP.to_uci(), "b");
            assert_eq!(MoveType::PROMOTION_ROOK.to_uci(), "r");
            assert_eq!(MoveType::PROMOTION_QUEEN.to_uci(), "q");
            assert_eq!(MoveType::PROMOTION_CAPTURE_KNIGHT.to_uci(), "n");
            assert_eq!(MoveType::PROMOTION_CAPTURE_BISHOP.to_uci(), "b");
            assert_eq!(MoveType::PROMOTION_CAPTURE_ROOK.to_uci(), "r");
            assert_eq!(MoveType::PROMOTION_CAPTURE_QUEEN.to_uci(), "q");
        }
    }

        #[test]
        pub fn new_works() {
            assert_eq!(MoveType::new(0b0000), MoveType::QUIET);
            assert_eq!(MoveType::new(0b0001), MoveType::DOUBLE_PAWN);
            assert_eq!(MoveType::new(0b0010), MoveType::SHORT_CASTLE);
            assert_eq!(MoveType::new(0b0011), MoveType::LONG_CASTLE);
            assert_eq!(MoveType::new(0b0100), MoveType::CAPTURE);
            assert_eq!(MoveType::new(0b0101), MoveType::EP_CAPTURE);
            assert_eq!(MoveType::new(0b0110), MoveType::NULLMOVE);
            assert_eq!(MoveType::new(0b0111), MoveType::UCI_AMBIGUOUS);
            assert_eq!(MoveType::new(0b1000), MoveType::PROMOTION_KNIGHT);
            assert_eq!(MoveType::new(0b1001), MoveType::PROMOTION_BISHOP);
            assert_eq!(MoveType::new(0b1010), MoveType::PROMOTION_ROOK);
            assert_eq!(MoveType::new(0b1011), MoveType::PROMOTION_QUEEN);
            assert_eq!(MoveType::new(0b1100), MoveType::PROMOTION_CAPTURE_KNIGHT);
            assert_eq!(MoveType::new(0b1101), MoveType::PROMOTION_CAPTURE_BISHOP);
            assert_eq!(MoveType::new(0b1110), MoveType::PROMOTION_CAPTURE_ROOK);
            assert_eq!(MoveType::new(0b1111), MoveType::PROMOTION_CAPTURE_QUEEN);
        }

    mod decode {
        use super::*;

        #[test]
        pub fn decode_works() {
            assert_eq!(MoveType::QUIET.decode(), "QUIET");
            assert_eq!(MoveType::DOUBLE_PAWN.decode(), "DOUBLE_PAWN");
            assert_eq!(MoveType::SHORT_CASTLE.decode(), "SHORT_CASTLE");
            assert_eq!(MoveType::LONG_CASTLE.decode(), "LONG_CASTLE");
            assert_eq!(MoveType::CAPTURE.decode(), "CAPTURE");
            assert_eq!(MoveType::EP_CAPTURE.decode(), "EP_CAPTURE");
            assert_eq!(MoveType::NULLMOVE.decode(), "NULLMOVE");
            assert_eq!(MoveType::UCI_AMBIGUOUS.decode(), "UCI_AMBIGUOUS");
            assert_eq!(MoveType::PROMOTION_KNIGHT.decode(), "PROMOTION_KNIGHT");
            assert_eq!(MoveType::PROMOTION_BISHOP.decode(), "PROMOTION_BISHOP");
            assert_eq!(MoveType::PROMOTION_ROOK.decode(), "PROMOTION_ROOK");
            assert_eq!(MoveType::PROMOTION_QUEEN.decode(), "PROMOTION_QUEEN");
            assert_eq!(MoveType::PROMOTION_CAPTURE_KNIGHT.decode(), "PROMOTION_CAPTURE_KNIGHT");
            assert_eq!(MoveType::PROMOTION_CAPTURE_BISHOP.decode(), "PROMOTION_CAPTURE_BISHOP");
            assert_eq!(MoveType::PROMOTION_CAPTURE_ROOK.decode(), "PROMOTION_CAPTURE_ROOK");
            assert_eq!(MoveType::PROMOTION_CAPTURE_QUEEN.decode(), "PROMOTION_CAPTURE_QUEEN");
        }
    }
}
