use super::*;

/// Used for easily making bitboards from a list of set squares, use as follows:
///
/// ```
/// #[macro_use] extern crate hazel;
/// use hazel::bitboard::Bitboard;
/// let bb = bitboard!("d4", "c6");
/// assert!(bb.is_notation_set("d4"));
/// assert!(bb.is_notation_set("c6"));
/// ```
#[macro_export] macro_rules! bitboard {
    () => (
        Bitboard::empty()
    );
    ($n:expr $(, $ns:expr)*) => (
        Bitboard::from_notation($n) | bitboard!($($ns),*)
    );
}

impl Bitboard {
    /// Creates an empty bitboard
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let b = Bitboard::empty();
    /// assert!(b.is_empty());
    /// ```
    pub fn empty() -> Bitboard {
        Bitboard { 0: 0 }
    }

    /// Creates a bitboard from the given u64
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let u = 0xC113_55B1_7B0A_2D55;
    /// let b = Bitboard::from(u);
    /// assert!(!b.is_empty());
    /// ```
    pub fn from(b: u64) -> Bitboard {
        Bitboard { 0: b }
    }

    /// Creates a bitboard from the given u64
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let b = Bitboard::from_notation("e4");
    /// assert!(b.is_notation_set("e4"));
    /// ```
    pub fn from_notation(n: &str) -> Bitboard {
        let mut b = Bitboard::empty();
        b.set_by_notation(n);
        b
    }

    /// Creates a bitboard with all bits set
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let b = Bitboard::full();
    /// assert!(b.is_full());
    /// ```
    pub fn full() -> Bitboard {
        !Bitboard::empty()
    }
}
