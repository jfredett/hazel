use super::*;

static NOT_A_FILE : u64 = 0xfefefefefefefefe;
static NOT_H_FILE : u64 = 0x7f7f7f7f7f7f7f7f;

// Note the lack of sign, that's handled below in the #shift and #shift_mut
// methods
//                                            N  NE E  SE S SW  W NW
static DIRECTION_INDEX_OFFSETS: [usize; 8] = [8, 9, 1, 7, 8, 9, 1, 7];

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    N  = 0,
    NE = 1,
    E  = 2,
    SE = 3,
    S  = 4,
    SW = 5,
    W  = 6,
    NW = 7
}


impl Bitboard {
    #[inline]
    pub fn shift(&self, d : Direction) -> Bitboard {
        let mut new_b = *self; // new_b is a copy of self

        new_b.shift_mut(d);
        return new_b;
    }

    pub fn shift_mut(&mut self, d : Direction) {
        let offset = DIRECTION_INDEX_OFFSETS[d as usize];
        match d {
            Direction::N    => { self.0 =  self.0 << offset               },
            Direction::S    => { self.0 =  self.0 >> offset               },
            Direction::E    => { self.0 = (self.0 << offset) & NOT_A_FILE },
            Direction::NE   => { self.0 = (self.0 << offset) & NOT_A_FILE },
            Direction::SE   => { self.0 = (self.0 >> offset) & NOT_A_FILE },
            Direction::W    => { self.0 = (self.0 >> offset) & NOT_H_FILE },
            Direction::SW   => { self.0 = (self.0 >> offset) & NOT_H_FILE },
            Direction::NW   => { self.0 = (self.0 << offset) & NOT_H_FILE }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slide_moves_pieces_appropriately() {
        let mut b = Bitboard::empty();
        b.set_by_notation("d4"); // Put a piece on d4.
        assert!(b.is_notation_set("d4")); // Put a piece on d4.

        b.shift_mut(Direction::N);
        assert!(!b.is_notation_set("d4"));
        assert!(b.is_notation_set("d5"));

        b.shift_mut(Direction::NE);
        assert!(!b.is_notation_set("d5"));
        assert!(b.is_notation_set("e6"));

        b.shift_mut(Direction::E);
        assert!(!b.is_notation_set("e6"));
        assert!(b.is_notation_set("f6"));

        b.shift_mut(Direction::SE);
        assert!(!b.is_notation_set("f6"));
        assert!(b.is_notation_set("g5"));

        b.shift_mut(Direction::S);
        assert!(!b.is_notation_set("g5"));
        assert!(b.is_notation_set("g4"));

        b.shift_mut(Direction::SW);
        assert!(!b.is_notation_set("g4"));
        assert!(b.is_notation_set("f3"));

        b.shift_mut(Direction::W);
        assert!(!b.is_notation_set("f3"));
        assert!(b.is_notation_set("e3"));

        b.shift_mut(Direction::NW);
        assert!(!b.is_notation_set("e3"));
        assert!(b.is_notation_set("d4"));
    }

    #[test]
    fn sliding_off_the_edge_removes_bit() {
        let mut b = Bitboard::empty();
        b.set_by_notation("h4");
        b.shift_mut(Direction::E);
        assert!(b.is_empty());
    }

    #[test]
    fn sliding_multiple_bits_works() {
        let mut b = Bitboard::empty();
        b.set_by_notation("g4");
        b.set_by_notation("b5");
        b.shift_mut(Direction::NE);

        assert!(b.is_notation_set("h5"));
        assert!(b.is_notation_set("c6"));
    }
}
