use dynamic_array::SmallArray;

// Covers all the IO operations on the tape, without an explicit read/write head being maintained.
pub trait Tapelike where Self: Sized {
    type Item;

    fn length(&self) -> usize;
    fn writehead(&self) -> usize;

    fn read_address(&self, address: usize) -> Self::Item;
    fn read_range(&self, start: usize, end: usize) -> SmallArray<Self::Item>;
    fn write_address(&mut self, address: usize, data: &Self::Item);
    fn write_range(&mut self, start: usize, data: &[Self::Item]);
}
