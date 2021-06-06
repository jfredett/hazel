bitflags! {
    pub struct MoveType: u16 {
        const CHECK = 0b100;
        const CAPTURE = 0b010;
        const ATTACK = 0b001;
    }
}

impl MoveType {
    /// True if the metadata encodes a check
    #[inline(always)] pub fn is_check(&self) -> bool { self.contains(MoveType::CHECK) }
    /// True if the metadata encodes a capture
    #[inline(always)] pub fn is_capture(&self) -> bool { self.contains(MoveType::CAPTURE) }
    /// True if the metadata encodes an attack on a piece
    #[inline(always)] pub fn is_attack(&self) -> bool { self.contains(MoveType::ATTACK) }
    /// True if the metadata is a quiet move
    #[inline(always)] pub fn is_quiet(&self) -> bool { self.bits() == 0 }
    
    // convenience constructors
    #[inline(always)] pub fn quiet()   -> MoveType { MoveType::from_bits(0).unwrap() }
    #[inline(always)] pub fn check()   -> MoveType { MoveType::CHECK }
    #[inline(always)] pub fn capture() -> MoveType { MoveType::CAPTURE }
    #[inline(always)] pub fn attack()  -> MoveType { MoveType::ATTACK }
}