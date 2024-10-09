use serde::{Deserialize, Serialize};

// Note the lack of sign, that's handled in the ,#shift and #shift_mut methods
//                                               N  NE E  SE S SW  W NW
pub const DIRECTION_INDEX_OFFSETS: [usize; 8] = [8, 9, 1, 7, 8, 9, 1, 7];

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Direction {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

impl Direction {
    /// shifts an index in the direction
    pub const fn index_shift(self, idx: usize) -> usize {
        match self {
            Direction::N => idx + 8,
            Direction::NE => idx + 9,
            Direction::E => idx + 1,
            Direction::SE => idx - 7,
            Direction::S => idx - 8,
            Direction::SW => idx - 9,
            Direction::W => idx - 1,
            Direction::NW => idx + 7,
        }
    }
}

pub const DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::Bitboard;
    use crate::constants::NOTATION_TO_INDEX;

    #[test]
    fn shift_north() {
        let mut b = NOTATION_TO_INDEX("e4");
        b = Direction::N.index_shift(b);
        assert_eq!(b, NOTATION_TO_INDEX("e5"));
    }

    #[test]
    fn shift_north_east() {
        let mut b = NOTATION_TO_INDEX("e4");
        b = Direction::NE.index_shift(b);
        assert_eq!(b, NOTATION_TO_INDEX("f5"));
    }

    #[test]
    fn shift_east() {
        let mut b = NOTATION_TO_INDEX("e4");
        b = Direction::E.index_shift(b);
        assert_eq!(b, NOTATION_TO_INDEX("f4"));
    }

    #[test]
    fn shift_south_east() {
        let mut b = NOTATION_TO_INDEX("e4");
        b = Direction::SE.index_shift(b);
        assert_eq!(b, NOTATION_TO_INDEX("f3"));
    }

    #[test]
    fn shift_south() {
        let mut b = NOTATION_TO_INDEX("e4");
        b = Direction::S.index_shift(b);
        assert_eq!(b, NOTATION_TO_INDEX("e3"));
    }

    #[test]
    fn shift_south_west() {
        let mut b = NOTATION_TO_INDEX("e4");
        b = Direction::SW.index_shift(b);
        assert_eq!(b, NOTATION_TO_INDEX("d3"));
    }

    #[test]
    fn shift_west() {
        let mut b = NOTATION_TO_INDEX("e4");
        b = Direction::W.index_shift(b);
        assert_eq!(b, NOTATION_TO_INDEX("d4"));
    }

    #[test]
    fn shift_north_west() {
        let mut b = NOTATION_TO_INDEX("e4");
        b = Direction::NW.index_shift(b);
        assert_eq!(b, NOTATION_TO_INDEX("d5"));
    }

    #[test]
    fn shift_by_shifts_by_given_amount() {
        let mut b = Bitboard::empty();
        b.set_by_notation("d4"); // Put a piece on d4.
        assert!(b.is_notation_set("d4")); // Put a piece on d4.

        let bb_after_shift = b.shift_by(Direction::N, 2);

        assert!(bb_after_shift.is_notation_set("d6"));

        assert!(!bb_after_shift.is_notation_set("d4"));
        assert!(!bb_after_shift.is_notation_set("d5"));
    }


}
