use crate::{notation::SquareNotation, types::Color};

mod constants;
mod display_debug;
mod iterator;
mod from_into;


pub use constants::*;
pub use display_debug::*;
pub use iterator::*;
pub use from_into::*;


/// Represents a single square by it's index rooted at a1 = 0, h8 = 63
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Square(usize);

impl SquareNotation for Square { }

impl Square {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub(crate) const fn _index(&self) -> usize {
        self.0
    }

    pub(crate) const fn _file(&self) -> usize {
        self.0 % 8
    }

    pub(crate) const fn _rank(&self) -> usize {
        self.0 / 8
    }

    pub(crate) const fn coords(&self) -> (usize, usize) {
        (self._rank(), self._file())
    }

    pub const fn backrank_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => self._rank() == 0,
            Color::BLACK => self._rank() == 7,
        }
    }

    pub const fn backrank(&self) -> bool {
        self._rank() == 0 || self._rank() == 7
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_is_correct() {
        assert_eq!(A1._rank(), 0);
        assert_eq!(A2._rank(), 1);
        assert_eq!(A3._rank(), 2);
        assert_eq!(A4._rank(), 3);
        assert_eq!(A5._rank(), 4);
        assert_eq!(A6._rank(), 5);
        assert_eq!(A7._rank(), 6);
        assert_eq!(A8._rank(), 7);

        assert_eq!(H1._rank(), 0);
    }

    #[test]
    fn file_is_correct() {
        assert_eq!(A1._file(), 0);
        assert_eq!(B1._file(), 1);
        assert_eq!(C1._file(), 2);
        assert_eq!(D1._file(), 3);
        assert_eq!(E1._file(), 4);
        assert_eq!(F1._file(), 5);
        assert_eq!(G1._file(), 6);
        assert_eq!(H1._file(), 7);

        assert_eq!(H8._file(), 7);
    }
}
