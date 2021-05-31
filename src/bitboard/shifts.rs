use super::*;


static A_FILE : u64 = 0x0101010101010101;
static H_FILE : u64 = 0x0808080808080808;
static NOT_A_FILE : u64 = !A_FILE;
static NOT_H_FILE : u64 = !H_FILE;


#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    NW =  7 , N =  8 , NE =  9 ,
    W  = -1 ,           E =  1 ,
    SW = -9 , S = -8 , SE = -7
}


impl Bitboard {
    //pub fn shift_north(&self) -> Bitboard { Bitboard::from(self.0 << 8) }
    //pub fn shift_south(&self) -> Bitboard { Bitboard::from(self.0 >> 8) }

    #[inline]
    pub fn shift(&self, d : Direction) -> Bitboard {
        let mut new_b = *self; // new_b is a copy of self

        new_b.shift_mut(d);
        return new_b;
    }

    pub fn shift_mut(&mut self, d : Direction) {
        let shift_value = d as isize;

        if shift_value > 0 {
            self.0 = (self.0 << shift_value.abs()) & NOT_A_FILE;
        } else {
            self.0 = (self.0 >> shift_value.abs()) & NOT_A_FILE;
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
        assert!(b.is_index_set(27));
        dbg!(b);
        b.shift_mut(Direction::N);
        dbg!(b);
        assert!(b.is_notation_set("d5"));

    }

    // slide test for each direction
    // slide test with stuff falling off the edge for each direction

}
