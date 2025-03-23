use dynamic_array::SmallArray;

// Covers all the IO operations on the tape, without an explicit read/write head being maintained.
pub trait Tapelike where Self: Sized {
    type Item;

    fn length(&self) -> usize;
    fn writehead(&self) -> usize;

    fn read_address(&self, address: usize) -> Self::Item;
    // BUG: I *hate* this. I should be able to return a reference to an arbitrary range of a tape,
    // instead I have to copy this stuff around, I know it's possible, I don't know how to do it.
    fn read_range(&self, start: usize, end: usize) -> SmallArray<Self::Item>;
    fn write_address(&mut self, address: usize, data: &Self::Item);
    fn write_range(&mut self, start: usize, data: &[Self::Item]);
}
