use crate::notation::*;


/// Standard 64 bit bitboards
///
/// By convention, each square is assigned indices as follows
///
/// hex:
///
/// 8| 38 39 3A 3B 3C 3D 3E 3F
/// 7| 30 31 32 33 34 35 36 37
/// 6| 28 29 2A 2B 2C 2D 2E 2F
/// 5| 20 21 22 23 24 25 26 27
/// 4| 18 19 1A 1B 1C 1D 1E 1F
/// 3| 10 11 12 13 14 15 16 17
/// 2| 8  9  A  B  C  D  E  F
/// 1| 0  1  2  3  4  5  6  7
///  ------------------------
///    a  b  c  d  e  f  g  h
///
/// decimal:
///
/// 8| 56 57 58 59 60 61 62 63
/// 7| 48 49 50 51 52 53 54 55
/// 6| 40 41 42 43 44 45 46 47
/// 5| 32 33 34 35 36 37 38 39
/// 4| 24 25 26 27 28 29 30 31
/// 3| 16 17 18 19 20 21 22 23
/// 2| 8  9  10 11 12 13 14 15
/// 1| 0  1  2  3  4  5  6  7
///  ------------------------
///    a  b  c  d  e  f  g  h
///
/// octal;
///
/// 8| 70 71 72 73 74 75 76 77
/// 7| 60 61 62 63 64 65 66 67
/// 6| 50 51 52 53 54 55 56 57
/// 5| 40 41 42 43 44 45 46 47
/// 4| 30 31 32 33 34 35 36 37
/// 3| 20 21 22 23 24 25 26 27
/// 2| 10 11 12 13 14 15 16 17
/// 1| 00 01 02 03 04 05 06 07
///  ------------------------
///    a  b  c  d  e  f  g  h
///
/// Octal is the most handy representation for understanding where things are located, but the
/// literature usually uses decimal.
#[derive(Hash, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Bitboard(u64);

mod arbitrary;
mod bitops;
mod creation;
mod debug;
mod from_into;
mod intrinsics;
mod iterator;
mod shifts;
mod util;

impl Bitboard {
    /// True if the bitboard has no set bits.
    ///
    /// ```
    /// # use hazel::types::Bitboard;
    /// # use hazel::notation::*;
    /// let mut b = Bitboard::empty();
    /// assert!(b.is_empty());
    /// b.set(A1);
    /// assert!(!b.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// True if the bitboard has any set bits.
    ///
    /// ```
    /// # use hazel::types::Bitboard;
    /// # use hazel::notation::*;
    /// let mut b = Bitboard::empty();
    /// assert!(b.is_empty());
    /// b.set(A1);
    /// assert!(b.is_nonempty());
    /// ```
    pub fn is_nonempty(&self) -> bool {
        !self.is_empty()
    }

    /// True if the bitboard has all bits set.
    ///
    /// ```
    /// # use hazel::types::Bitboard;
    /// let mut b = Bitboard::full();
    /// assert!(b.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.0 == !0
    }

    /// Sets the bit at the given coordinates, indexes from 0 to 7.
    ///
    /// ```
    /// # use hazel::types::Bitboard;
    /// # use hazel::notation::*;
    /// let mut b = Bitboard::empty();
    /// assert!(!b.is_set(A1));
    /// b.set(A1);
    /// assert!(b.is_set(A1));
    /// ```
    #[inline]
    pub fn set(&mut self, square: impl Into<Square>) {
        self.0 |= 1 << square.into().index();
    }

    /// Return a vector containing all the indices which are set
    ///
    /// TODO: Replace this with an iterator that progressively pops the least set bit, should be quicker.
    ///
    /// ```
    /// # use hazel::types::Bitboard;
    /// # use hazel::notation::*;
    /// let mut b = Bitboard::empty();
    /// b.set(A1);
    /// b.set(A2);
    /// b.set(A3);
    /// assert_eq!(b.all_set_indices(), vec![A1.index(),A2.index(),A3.index()]);
    /// ```
    ///
    /// TODO: Replace return value with `Square` objects
    pub fn all_set_indices(&self) -> Vec<usize> {
        self.into_iter().collect()
    }

    /// unsets the bit at the given coordinates
    ///
    /// ```
    /// # use hazel::types::Bitboard;
    /// # use hazel::notation::*;
    /// let mut b = Bitboard::empty();
    /// assert!(!b.is_set(A1));
    /// b.set(A1);
    /// assert!(b.is_set(A1));
    /// b.unset(A1);
    /// assert!(!b.is_set(A1));
    /// ```
    pub fn unset(&mut self, square: impl Into<Square>) {
        self.0 &= !(1 << square.into().index());
    }

    /// Logically 'moves' a piece from the 'from' square to the 'to' square
    pub fn move_piece(&mut self, from: usize, to: usize) {
        self.unset(Square::new(from));
        self.set(Square::new(to));
    }

    /// unsets the bit at the given coordinates
    /// ```
    /// # use hazel::types::Bitboard;
    /// # use hazel::notation::*;
    /// let mut b = Bitboard::empty();
    /// assert!(!b.is_set(A1));
    /// b.flip(A1);
    /// assert!(b.is_set(A1));
    /// b.flip(A1);
    /// assert!(!b.is_set(A1));
    /// ```
    pub fn flip(&mut self, square: impl Into<Square>) {
        self.0 ^= 1 << square.into().index();
    }

    /// True if the given bit is set
    ///
    /// ```
    /// # use hazel::types::Bitboard;
    /// # use hazel::notation::*;
    /// let mut b = Bitboard::empty();
    /// b.set(A2);
    /// assert!(b.is_set(A2));
    /// assert!(!b.is_set(A1));
    /// ```
    #[inline]
    pub fn is_set(&self, square: impl Into<Square>) -> bool {
        self.0 & (1 << square.into().index()) != 0
    }

    /// Count the number of set squares
    ///
    /// ```
    /// # use hazel::types::Bitboard;
    /// # use hazel::notation::*;
    /// let mut b = Bitboard::empty();
    /// assert_eq!(b.count(), 0);
    /// b.set(A1);
    /// b.set(B1);
    /// b.set(C1);
    /// assert_eq!(b.count(), 3);
    /// ```
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_empty {
        use super::*;

        #[test]
        fn accurately_reports_if_board_is_empty() {
            let empty_board = Bitboard::empty();
            assert!(empty_board.is_empty());
        }

        #[test]
        fn accurately_reports_if_board_is_not_empty() {
            let mut nonempty_board = Bitboard::empty();
            nonempty_board.set(A1);
            assert!(!nonempty_board.is_empty());
        }
    }

    mod set_and_unset {
        use super::*;

        #[test]
        fn accurately_reports_if_square_is_set() {
            let mut board = Bitboard::empty();
            assert!(!board.is_set(A1));
            board.set(A1);
            assert!(board.is_set(A1));
        }

        #[test]
        fn accurately_reports_if_square_is_not_set() {
            let mut board = Bitboard::empty();
            assert!(!board.is_set(A2));
            board.set(A1);
            assert!(!board.is_set(A2));
        }

        #[test]
        fn is_idempotent_if_the_bit_is_already_set() {
            let mut board = Bitboard::empty();
            assert!(!board.is_set(A1));
            board.set(A1);
            assert!(board.is_set(A1));
            board.set(A1);
            assert!(board.is_set(A1));
        }

        #[test]
        fn is_inverse_to_unset() {
            let mut board = Bitboard::empty();
            assert!(!board.is_set(A1));
            board.set(A1);
            assert!(board.is_set(A1));
            board.unset(A1);
            assert!(!board.is_set(A1));
        }
    }

    mod is_full {
        use super::*;

        #[test]
        fn accurately_reports_if_board_is_full() {
            let full_board = Bitboard::full();
            assert!(full_board.is_full());
        }
    }

    mod flip {
        use super::*;

        #[quickcheck]
        fn setting_then_flipping_is_idempotent(x_i: usize, y_i: usize) -> bool {
            let x = x_i % 8;
            let y = y_i % 8;
            let mut board = Bitboard::empty();
            let s = Square::from((x, y));

            assert!(!board.is_set(s));
            board.set(s);
            assert!(board.is_set(s));
            board.flip(s);
            assert!(!board.is_set(s));
            board == Bitboard::empty()
        }

        #[quickcheck]
        fn double_flipping_is_idempotent(x_i: usize, y_i: usize) -> bool {
            let x = x_i % 8;
            let y = y_i % 8;
            let mut board = Bitboard::empty();
            let s = Square::from((x, y));

            assert!(!board.is_set(s));
            board.flip(s);
            assert!(board.is_set(s));
            board.flip(s);
            assert!(!board.is_set(s));

            board == Bitboard::empty()
        }
    }
}
