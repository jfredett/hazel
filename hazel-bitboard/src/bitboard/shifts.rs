
use hazel_basic::direction::{Direction, DIRECTION_INDEX_OFFSETS};
use hazel_basic::file::{NOT_A_FILE, NOT_H_FILE};
use crate::bitboard::Bitboard;

impl Bitboard {
    #[inline]
    pub const fn shift(&self, d: Direction) -> Bitboard {
        let mut new_b = *self; // new_b is a copy of self

        new_b.shift_mut(d);
        new_b
    }


    #[rustfmt::skip]
    pub const fn shift_mut(&mut self, d : Direction) {
        let offset = DIRECTION_INDEX_OFFSETS[d as usize];
        match d {
            Direction::N    => { self.0 <<= offset               },
            Direction::S    => { self.0 >>= offset               },
            Direction::E    => { self.0 = (self.0 << offset) & NOT_A_FILE },
            Direction::NE   => { self.0 = (self.0 << offset) & NOT_A_FILE },
            Direction::SE   => { self.0 = (self.0 >> offset) & NOT_A_FILE },
            Direction::W    => { self.0 = (self.0 >> offset) & NOT_H_FILE },
            Direction::SW   => { self.0 = (self.0 >> offset) & NOT_H_FILE },
            Direction::NW   => { self.0 = (self.0 << offset) & NOT_H_FILE }
        }
    }

    pub fn shift_by(&self, d: Direction, amt: usize) -> Bitboard {
        let mut out = *self;
        for _ in 0..amt {
            out.shift_mut(d);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use hazel_basic::square::*;

    use super::*;

    #[test]
    fn shift_by_shifts_by_given_amount() {
        let mut b = Bitboard::empty();
        b.set(D4); // Put a piece on d4.
        assert!(b.is_set(D4)); // Put a piece on d4.

        let bb_after_shift = b.shift_by(Direction::N, 2);

        assert!(bb_after_shift.is_set(D6));

        assert!(!bb_after_shift.is_set(D4));
        assert!(!bb_after_shift.is_set(D5));
    }

    #[test]
    fn slide_moves_pieces_appropriately() {
        let mut b = Bitboard::empty();
        b.set(D4); // Put a piece on d4.
        assert!(b.is_set(D4)); // Put a piece on d4.

        b.shift_mut(Direction::N);
        assert!(!b.is_set(D4));
        assert!(b.is_set(D5));

        b.shift_mut(Direction::NE);
        assert!(!b.is_set(D5));
        assert!(b.is_set(E6));

        b.shift_mut(Direction::E);
        assert!(!b.is_set(E6));
        assert!(b.is_set(F6));

        b.shift_mut(Direction::SE);
        assert!(!b.is_set(F6));
        assert!(b.is_set(G5));

        b.shift_mut(Direction::S);
        assert!(!b.is_set(G5));
        assert!(b.is_set(G4));

        b.shift_mut(Direction::SW);
        assert!(!b.is_set(G4));
        assert!(b.is_set(F3));

        b.shift_mut(Direction::W);
        assert!(!b.is_set(F3));
        assert!(b.is_set(E3));

        b.shift_mut(Direction::NW);
        assert!(!b.is_set(E3));
        assert!(b.is_set(D4));
    }

    #[test]
    fn sliding_off_the_edge_removes_bit() {
        // shifting off the top right of the board
        for d in &[ Direction::N, Direction::NE, Direction::E, Direction::SE, Direction::NW, ] {
            let mut b = Bitboard::empty();
            b.set(H8);
            b.shift_mut(*d);
            assert!(b.is_empty());
        }

        // shifting off the bottom left of the board
        for d in &[Direction::S, Direction::SW, Direction::W, Direction::NW, Direction::SE] {
            let mut b = Bitboard::empty();
            b.set(A1);
            b.shift_mut(*d);
            assert!(b.is_empty());
        }
    }

    #[test]
    fn sliding_multiple_bits_works() {
        let mut b = Bitboard::empty();
        b.set(G4);
        b.set(B5);
        b.shift_mut(Direction::NE);

        assert!(b.is_set(H5));
        assert!(b.is_set(C6));
    }
}
