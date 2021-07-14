bitflags! {
    pub struct MoveType: u16 {
        const QUIET                 = 0b0000;
        const DOUBLE_PAWN           = 0b0001;
        const SHORT_CASTLE          = 0b0010;
        const LONG_CASTLE           = 0b0011;
        const CAPTURE               = 0b0100;
        const EP_CAPTURE            = 0b0101;
        const UNUSED_1              = 0b0110;
        const UNUSED_2              = 0b0111;
        const PROMOTION             = 0b1000;
        const PROMOTION_CAPTURE     = 0b1100;
    }
}

impl MoveType {
    #[inline(always)] pub fn is_long_castle(&self)  -> bool { self.contains(MoveType::LONG_CASTLE) }
    #[inline(always)] pub fn is_short_castle(&self) -> bool { self.contains(MoveType::SHORT_CASTLE) }
    #[inline(always)] pub fn is_capture(&self)      -> bool { self.contains(MoveType::CAPTURE) }
    #[inline(always)] pub fn is_quiet(&self)        -> bool { self.contains(MoveType::QUIET) }
    #[inline(always)] pub fn is_promotion(&self)    -> bool { self.contains(MoveType::PROMOTION) }
    
    // convenience constructors
    #[inline(always)] pub fn quiet()        -> MoveType { MoveType::QUIET }
    #[inline(always)] pub fn capture()      -> MoveType { MoveType::CAPTURE }
    #[inline(always)] pub fn short_castle() -> MoveType { MoveType::SHORT_CASTLE }
    #[inline(always)] pub fn long_castle()  -> MoveType { MoveType::LONG_CASTLE }
}