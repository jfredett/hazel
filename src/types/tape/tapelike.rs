use std::range::Range;

use super::{cursor::Cursor, familiar::Familiar};

// Covers all the IO operations on the tape, without an explicit read/write head being maintained.
pub trait Tapelike {
    type Item;

    fn length(&self) -> usize;

    fn read_address(&self, address: usize) -> Option<&Self::Item>;
    fn read_range(&self, range: impl Into<Range<usize>>) -> &[Option<Self::Item>];
    fn write_address(&mut self, address: usize, data: &Self::Item);
    fn write_range(&mut self, start: usize, data: &[Self::Item]);
}
