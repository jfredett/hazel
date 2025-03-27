use std::range::Range;

use hazel_basic::interface::Alteration;

use crate::types::tape::{cursor::Cursor, cursorlike::Cursorlike, familiar::Familiar, tapelike::Tapelike, Tape};

#[derive(Debug, Clone)]
pub struct TapeReaderState {
    pub context: Vec<Alteration>,
    pub position: usize, // the currently selected position of the buffer
    pub tape_length: usize, // the total length of the tape
    pub length: usize, // the size of the slice to retrieve
}

const DEFAULT_TAPE_READER_LENGTH : usize = 32;

impl Default for TapeReaderState {
    fn default() -> Self {
        TapeReaderState {
            context: vec![],
            position: 0,
            tape_length: DEFAULT_TAPE_READER_LENGTH,
            length: DEFAULT_TAPE_READER_LENGTH,
        }
    }
}

impl TapeReaderState {
    pub fn update<T>(&mut self, cursor: &Cursor<T>) where T : Tapelike<Item = Alteration> {
        // TODO: only fetch the range when we page over.
        self.position = cursor.position();
        self.tape_length = cursor.length();

        let (start, end) = self.page_range();
        let context = cursor.read_range(start, end);
        self.context = context.to_vec();
    }

    // convert from a desired position in the buffer to the screen of code that needs to be
    // displayed
    // TODO: Would be nice to have an 'overlap' between screens by a few (configurable number of?)
    // rows.
    pub fn position_in_page(&self) -> usize {
        self.position % self.length
    }

    pub fn offset(&self) -> usize {
        self.page() * self.length
    }

    pub fn set_page_size(&mut self, ps: usize) {
        self.length = ps;
    }

    pub fn page_range(&self) -> (usize, usize) {
        (self.offset(), self.offset() + self.length - 1)
    }

    pub fn page(&self) -> usize {
        self.position / self.length
    }

    pub fn total_pages(&self) -> usize {
        self.tape_length / self.length // TODO: potential off-by-one
    }

    pub fn header(&self) -> (usize, usize, usize, usize) {
        (self.position, self.page(), self.total_pages(), self.tape_length)
    }


}


impl Familiar<Tape, TapeReaderState> {
    pub fn context_range(&self) -> Range<usize> {
        let mut start = self.cursor.position();

        if self.cursor.length() < start { return (start..start).into(); }

        let distance_to_hwm = self.cursor.length() - start;
        let end = if distance_to_hwm > self.state.length {
            self.cursor.length()
        } else {
            start -= self.state.length - distance_to_hwm;
            self.cursor.length()
        };
        (start..end).into()
    }
}

impl<T> Cursorlike for Familiar<T, TapeReaderState> where T : Tapelike<Item = Alteration> {
    fn advance(&mut self) {
        self.cursor.advance();
        self.state.update(&self.cursor);
    }

    fn length(&self) -> usize {
        self.cursor.length()
    }

    fn jump(&mut self, desired_position: usize) {
        self.cursor.jump(desired_position)
    }

    fn at_end(&self) -> bool {
        self.cursor.at_end()
    }

    fn position(&self) -> usize {
        self.cursor.position()
    }

    fn rewind(&mut self) {
        self.state.update(&self.cursor);
        self.cursor.rewind();
    }
}


// I want to break this down better:
//
// 1. ContextWidnow should be the type of a ContextFamiliar, which maintains some fixed context as
//    it advances/retreats through the tape.
// 2. It only manages calculating the context and keeping track of relative position, etc.
// 3. I can run a secondary familiar or set thereof to visualize further.
//
// Another option would be to just directly expose the tape, then calculate other items OTF w/
// familiars?

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn paging_calculation() {
        let mut trs = TapeReaderState {
            context: vec![],
            position: 0,
            tape_length: 10*DEFAULT_TAPE_READER_LENGTH,
            length: DEFAULT_TAPE_READER_LENGTH,
        };
        assert_eq!(trs.page_range(), (0,DEFAULT_TAPE_READER_LENGTH - 1));
        assert_eq!(trs.offset(), 0);
        assert_eq!(trs.position_in_page(), 0);
        assert_eq!(trs.page(), 0);
        trs.position = 1;
        assert_eq!(trs.page_range(), (0, (DEFAULT_TAPE_READER_LENGTH - 1)));
        assert_eq!(trs.offset(), 0);
        assert_eq!(trs.position_in_page(), 1);
        assert_eq!(trs.page(), 0);
        trs.position = 0x1F;
        assert_eq!(trs.page_range(), (0, (DEFAULT_TAPE_READER_LENGTH - 1)));
        assert_eq!(trs.offset(), 0);
        assert_eq!(trs.position_in_page(), 0x1F);
        assert_eq!(trs.page(), 0);
        trs.position = 0x20;
        assert_eq!(trs.page_range(), (DEFAULT_TAPE_READER_LENGTH, (2 * DEFAULT_TAPE_READER_LENGTH - 1)));
        assert_eq!(trs.offset(), 0x20);
        assert_eq!(trs.page(), 1);
        assert_eq!(trs.position_in_page(), 0);
        trs.position = 0x21;
        assert_eq!(trs.page_range(), (DEFAULT_TAPE_READER_LENGTH, (2 * DEFAULT_TAPE_READER_LENGTH - 1)));
        assert_eq!(trs.offset(), 0x20);
        assert_eq!(trs.page(), 1);
        assert_eq!(trs.position_in_page(), 1);


    }
    
}
