use crate::types::Color;

mod constants;
mod display_debug;
mod iterator;
mod from_into;
mod movements;


pub use constants::*;
pub use iterator::*;


/// Represents a single square by it's index rooted at a1 = 0, h8 = 63
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Square(usize);

impl Square {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn set_rank(&mut self, rank: usize) -> Self {
        self.0 = rank * 8 + self.file();
        *self
    }

    pub const fn set_file(&mut self, file: usize) -> Self {
        self.0 = self.rank() * 8 + file;
        *self
    }

    pub const fn index(&self) -> usize {
        self.0
    }

    pub const fn file(&self) -> usize {
        self.0 % 8
    }

    pub const fn rank(&self) -> usize {
        self.0 / 8
    }

    pub const fn backrank_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => self.rank() == 0,
            Color::BLACK => self.rank() == 7,
        }
    }

    pub const fn backrank(&self) -> bool {
        self.rank() == 0 || self.rank() == 7
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::{Arbitrary, Gen};

    impl Arbitrary for Square {
        fn arbitrary(g: &mut Gen) -> Self {
            Square(usize::arbitrary(g) % 64)
        }
    }

    #[test]
    fn rank_is_correct() {
        assert_eq!(A1.rank(), 0);
        assert_eq!(A2.rank(), 1);
        assert_eq!(A3.rank(), 2);
        assert_eq!(A4.rank(), 3);
        assert_eq!(A5.rank(), 4);
        assert_eq!(A6.rank(), 5);
        assert_eq!(A7.rank(), 6);
        assert_eq!(A8.rank(), 7);

        assert_eq!(H1.rank(), 0);
    }

    #[test]
    fn file_is_correct() {
        assert_eq!(A1.file(), 0);
        assert_eq!(B1.file(), 1);
        assert_eq!(C1.file(), 2);
        assert_eq!(D1.file(), 3);
        assert_eq!(E1.file(), 4);
        assert_eq!(F1.file(), 5);
        assert_eq!(G1.file(), 6);
        assert_eq!(H1.file(), 7);

        assert_eq!(H8.file(), 7);
    }

}
