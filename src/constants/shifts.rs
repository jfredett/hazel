// Note the lack of sign, that's handled in the #shift and #shift_mut methods
//                                               N  NE E  SE S SW  W NW
pub const DIRECTION_INDEX_OFFSETS: [usize; 8] = [8, 9, 1, 7, 8, 9, 1, 7];

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
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
    pub fn index_shift(self, idx: usize) -> usize {
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
mod tests {}
