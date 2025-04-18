use super::*;


/// Used for easily making bitboards from a list of set squares, use as follows:
///
/// ```
/// # use hazel_bitboard::{bitboard, bitboard::Bitboard};
/// # use hazel_core::square::*;
///
/// let bb = bitboard!("d4", "c6");
/// assert!(bb.is_set(D4));
/// assert!(bb.is_set(C6));
/// ```
#[macro_export]
macro_rules! bitboard {
    () => (
        Bitboard::empty()
    );
    ($n:expr $(, $ns:expr)*) => (
        Bitboard::from($n) | bitboard!($($ns),*)
    );
}

impl Bitboard {
    /// Creates an empty bitboard
    ///
    /// ```
    /// # use hazel_bitboard::bitboard::Bitboard;
    /// let b = Bitboard::empty();
    /// assert!(b.is_empty());
    /// ```
    pub const fn empty() -> Bitboard {
        Bitboard(0)
    }

    /// Creates a bitboard with all bits set
    ///
    /// ```
    /// # use hazel_bitboard::bitboard::Bitboard;
    /// let b = Bitboard::full();
    /// assert!(b.is_full());
    /// ```
    pub fn full() -> Bitboard {
        !Bitboard::empty()
    }

    pub fn from_index(index: usize) -> Bitboard {
        Bitboard(1 << index)
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let b = Bitboard::empty();
        assert_eq!(b.0, 0);
    }

    #[test]
    fn from() {
        let b = Bitboard::from(A1);
        assert_eq!(b.0, 1);

        let b = Bitboard::from(H8);
        assert_eq!(b.0, 1 << 63);
    }

    #[test]
    fn new() {
        let b = Bitboard::new(0x1234_5678_9ABC_DEF0u64);
        assert_eq!(b.0, 0x1234_5678_9ABC_DEF0);
    }

    #[test]
    fn full() {
        let b = Bitboard::full();
        assert_eq!(b.0, 0xFFFF_FFFF_FFFF_FFFF);
    }

    #[test]
    fn from_square_notation() {
        let b = Bitboard::from(E4);
        assert_eq!(b.0, 1 << E4.index() as u64);
    }
}
