/// Standard 64 bit bitboards
///
/// By convention: a1 = (0,0) = bit 0, h8 = (7,7) = bit 63
#[derive(Hash, PartialEq, Eq)]
pub struct Bitboard(u64);

mod bitops;
mod debug;


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


// traits to implement
// Binary (formats to a binary string)
// LowerHex
// UpperHex (similar, format as hex)
// Octal (similar, format as octal
//
// ^-- those maybe derivable?
//
// Ops::BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shifts?
//
// Probably want to split this into a mod.rs and folder.

impl Bitboard {
    // # Creation
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
        self.0 |= 1 << (y * 8) + x;
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
        self.0 &= !(1 << (y * 8) + x);
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
        self.0 ^= 1 << (y * 8) + x;
    }


    // #flip(x,y)
    //
    // # Setwise/Bitwise operations
    //
    // AND/Intersect
    // OR/Union
    // XOR, NOT
    //
    // # Shift, Rotate, and Wrap
    //
    // #shift(DIRECTION) where DIRECTION is an enum
    // #wrap(DIRECTION) where DIRECTION is an enum
    // #rotate_cw() rotates 90 clockwise
    // #rotate_acw(), #rotate_ccw() rotates 90 counterclockwise
    //
    //


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
        (self.0 & 1 << (y * 8) + x) != 0
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
