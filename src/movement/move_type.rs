bitflags! {
    pub struct MoveType: u16 {
        const CHECK = 0b100;
        const CAPTURE = 0b010;
        const ATTACK = 0b001;
        const SHORT_CASTLE = 0b110;
        const LONG_CASTLE = 0b011;
    }
}

impl MoveType {
    /// True if the metadata encodes a long castle
    #[inline(always)] pub fn is_long_castle(&self) -> bool { self.bits() == 0b011 }
    /// True if the metadata encodes a short castle
    #[inline(always)] pub fn is_short_castle(&self) -> bool { self.bits() == 0b110 }
    /// True if the metadata encodes a check
    #[inline(always)] pub fn is_check(&self) -> bool { self.bits() == 0b100 }
    /// True if the metadata encodes a capture
    #[inline(always)] pub fn is_capture(&self) -> bool { self.bits() == 0b010 }
    /// True if the metadata encodes an attack on a piece
    #[inline(always)] pub fn is_attack(&self) -> bool { self.bits() == 0b001 }
    /// True if the metadata is a quiet move
    #[inline(always)] pub fn is_quiet(&self) -> bool { self.bits() == 0b000 }
    
    // convenience constructors
    #[inline(always)] pub fn quiet()   -> MoveType { MoveType::from_bits(0).unwrap() }
    #[inline(always)] pub fn check()   -> MoveType { MoveType::CHECK }
    #[inline(always)] pub fn capture() -> MoveType { MoveType::CAPTURE }
    #[inline(always)] pub fn attack()  -> MoveType { MoveType::ATTACK }
    #[inline(always)] pub fn short_castle()  -> MoveType { MoveType::SHORT_CASTLE }
    #[inline(always)] pub fn long_castle()  -> MoveType { MoveType::LONG_CASTLE }
}