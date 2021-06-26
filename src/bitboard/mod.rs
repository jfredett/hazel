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
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(u64);

mod creation;
mod bitops;
mod util;
mod arbitrary;
mod debug;
mod shifts;

use crate::constants::conversion_tables::*;

pub use shifts::*;

impl Bitboard {
    /// True if the bitboard has no set bits.
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// assert!(b.is_empty());
    /// b.set(1,1);
    /// assert!(!b.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// True if the bitboard has all bits set.
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::full();
    /// assert!(b.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.0 == !0
    }

    /// Sets the bit at the given coordinates, indexes from 0 to 7.
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// assert!(!b.is_set(0,1));
    /// b.set(0,1);
    /// assert!(b.is_set(0,1));
    /// ```
    #[inline]
    pub fn set(&mut self, x: usize, y: usize) {
        self.set_by_index(Bitboard::coords_to_index(x,y));
    }

    /// Set a bit located at the given index
    #[inline]
    pub fn set_by_index(&mut self, idx: usize) {
        self.0 |= 1 << idx
    }

    /// Set a bit located at the given notation
    #[inline]
    pub fn set_by_notation(&mut self, notation: &str) {
        let (x,y) = Bitboard::notation_to_coords(notation);
        self.set(x,y);
    }

    /// Return a vector containing all the indices which are set
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// b.set_by_index(10);
    /// b.set_by_index(20);
    /// b.set_by_index(30);
    /// assert_eq!(b.all_set_indices(), vec![10,20,30]);
    /// ```
    pub fn all_set_indices(&self) -> Vec<usize> {
        let mut out = vec![];
        for i in 0..63 { 
            if self.is_index_set(i) {
                out.push(i);
            }
        }
        out
    }

    /// unsets the bit at the given coordinates
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// assert!(!b.is_set(0,1));
    /// b.set(0,1);
    /// assert!(b.is_set(0,1));
    /// b.unset(0,1);
    /// assert!(!b.is_set(0,1));
    /// ```
    pub fn unset(&mut self, rank: usize, file: usize) {
        self.unset_by_index(Bitboard::coords_to_index(rank,file));
    }
    
    /// unsets the bit at the given index
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// assert!(!b.is_index_set(43));
    /// b.set_by_index(43);
    /// assert!(b.is_index_set(43));
    /// b.unset_by_index(43);
    /// assert!(!b.is_index_set(43));
    /// ```
    pub fn unset_by_index(&mut self, idx: usize) {
        self.0 &= !(1 << idx)
    }

    /// unsets the bit at the given coordinates
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// assert!(!b.is_set(0,1));
    /// b.flip(0,1);
    /// assert!(b.is_set(0,1));
    /// b.flip(0,1);
    /// assert!(!b.is_set(0,1));
    /// ```
    pub fn flip(&mut self, rank: usize, file: usize) {
        self.0 ^= 1 << Bitboard::coords_to_index(rank,file);
    }

    /// True if the bit at the given notation is set
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// b.set_by_notation("d4");
    /// assert!(b.is_notation_set("d4"));
    /// ```
    #[inline]
    pub fn is_notation_set(&self, notation: &str) -> bool {
        let (rank, file) = Bitboard::notation_to_coords(notation);
        self.is_set(rank, file)
    }

    /// True if the given bit is set
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// b.set(1,1);
    /// assert!(b.is_set(1,1));
    /// assert!(!b.is_set(0,1));
    /// ```
    #[inline]
    pub fn is_set(&self, rank: usize, file: usize) -> bool {
        self.is_index_set(Bitboard::coords_to_index(rank,file))
    }

    /// True if the given bit is set
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// b.set(1,1);
    /// assert!(b.is_set(1,1));
    /// assert!(b.is_index_set(9));
    /// ```
    #[inline]
    pub fn is_index_set(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }

    /// Count the number of set squares
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// assert_eq!(b.count(), 0);
    /// b.set_by_notation("a1");
    /// b.set_by_notation("b1");
    /// b.set_by_notation("c1");
    /// assert_eq!(b.count(), 3);
    /// ```
    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

#[cfg(test)]
mod test {
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
            nonempty_board.set(1,1);
            assert!(!nonempty_board.is_empty());
        }
    }

    mod set_and_unset {
        use super::*;

        #[test]
        fn accurately_reports_if_square_is_set() {
            let mut board = Bitboard::empty();
            board.set(1,1);
            assert!(board.is_set(1,1));
        }

        #[test]
        fn accurately_reports_if_square_is_not_set() {
            let mut board = Bitboard::empty();
            board.set(1,1);
            assert!(!board.is_set(0,1));
        }

        #[test]
        fn is_idempotent_if_the_bit_is_already_set() {
            let mut board = Bitboard::empty();
            board.set(1,1);
            assert!(board.is_set(1,1));
            board.set(1,1);
            assert!(board.is_set(1,1));
        }

        #[test]
        fn is_inverse_to_unset() {
            let mut board = Bitboard::empty();
            board.set(1,1);
            assert!(board.is_set(1,1));
            board.unset(1,1);
            assert!(!board.is_set(1,1));
        }
    }

    mod flip {
        use super::*;

        #[quickcheck]
        fn setting_then_flipping_is_idempotent(x_i: usize, y_i: usize) -> bool {
            let x = x_i % 8; let y = y_i % 8;
            let mut board = Bitboard::empty();
            board.set(x,y);
            assert!(board.is_set(x,y));
            board.flip(x,y);
            assert!(!board.is_set(x,y));
            return board == Bitboard::empty();
        }

        #[quickcheck]
        fn double_flipping_is_idempotent(x_i: usize, y_i: usize) -> bool{
            let x = x_i % 8; let y = y_i % 8;
            let mut board = Bitboard::empty();
            board.flip(x,y);
            assert!(board.is_set(x,y));
            board.flip(x,y);
            assert!(!board.is_set(x,y));
            return board == Bitboard::empty();
        }
    }
}
