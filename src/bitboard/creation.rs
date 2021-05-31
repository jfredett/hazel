use super::*;

impl Bitboard {
    /// Creates an empty bitboard
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let b = Bitboard::empty();
    /// assert!(b.is_empty());
    /// ```
    pub fn empty() -> Bitboard {
        return Bitboard { 0: 0 }
    }

    /// Creates a bitboard from the given u64
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let u = 0xC113_55B1_7B0A_2D55;
    /// let b = Bitboard::from(u);
    /// assert!(!b.is_empty());
    /// ```
    pub fn from(b: u64) -> Bitboard {
        return Bitboard { 0: b }
    }

    /// Creates a bitboard with all bits set
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let b = Bitboard::full();
    /// assert!(b.is_full());
    /// ```
    pub fn full() -> Bitboard {
        return !Bitboard::empty()
    }
}
