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
/// 1| 0  1  2  3  4  5  6  7
///  ------------------------
///    a  b  c  d  e  f  g  h
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Bitboard(u64);

mod creation;
mod bitops;
mod util;
mod arbitrary;
mod debug;
mod shifts;
mod constants;

pub use constants::*;



// NOTE: Should make a bunch of arrays with all the bitboards for a piece at a given location, so
// all the queen-boards w/ the squares they attack, etc. Can basically create all these bitboard
// libraries and have the Bitboard object just look up the right things. Each library would only
// have ~64 entries each. We'd need two per piece -- plus a couple extra for the color-distinct
// moves.
//
// They'd fall into "Attack Boards" -- showing all squares the piece could attack on a blank board,
// and "Move Boards" -- showing all squares they could attack on a blank board.
//
// We'd want some way to incorporate blocking as well I suppose the algorithm would be something
// like:
//
//
// Take the position's bitboard, AND it with the pieces bitboard, if we see any values still high
// then we know there is a block there, and we can then calculate that those pieces are attacked,
// and any 'behind' those pieces are not. Only matters for a few pieces (bishops, rooks, and
// queens). Also need to account for piece color in that.
//


// Shifts?
//
// Probably want to split this into a mod.rs and folder.

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
    pub fn set(&mut self, x: usize, y: usize) {
        self.set_by_index(Bitboard::coords_to_index(x,y));
    }

    /// Set a bit located at the given index
    #[inline]
    pub fn set_by_index(&mut self, i: usize) {
        self.0 |= 1 << i
    }

    pub fn set_by_notation(&mut self, notation: &str) {
        let (x,y) = Bitboard::notation_to_coords(notation);
        self.set(x,y);
    }

    /// unsets the bit at the given coordinates
    ///
    /// ```
    /// # use hazel::bitboard::Bitboard;
    /// let mut b = Bitboard::empty();
    /// assert!(!b.is_set(0,1));
    /// b.set(0,1);
    /// assert!(b.is_set(0,1));
    /// ```
    pub fn unset(&mut self, x: usize, y: usize) {
        self.0 &= !(1 << Bitboard::coords_to_index(x,y));
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
    pub fn flip(&mut self, x: usize, y: usize) {
        self.0 ^= 1 << Bitboard::coords_to_index(x,y);
    }

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
    pub fn is_set(&self, x: usize, y: usize) -> bool {
        (self.0 & (1 << Bitboard::coords_to_index(x,y))) != 0
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
    pub fn is_index_set(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }


    // # Shift, Rotate, and Wrap
    //
    // #shift(DIRECTION) where DIRECTION is an enum
    // #wrap(DIRECTION) where DIRECTION is an enum
    // #rotate_cw() rotates 90 clockwise
    // #rotate_acw(), #rotate_ccw() rotates 90 counterclockwise
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