
// Note the lack of sign, that's handled in the #shift and #shift_mut methods
//                                               N  NE E  SE S SW  W NW
pub const DIRECTION_INDEX_OFFSETS: [usize; 8] = [8, 9, 1, 7, 8, 9, 1, 7];

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

pub const DIRECTIONS : [Direction; 8] = [
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];