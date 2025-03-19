use std::range::Range;

// TODO: This should probably live under /ui/
use ratatui::{prelude::Stylize, style::Style, text::Text, widgets::{Row, TableState}};

use crate::{types::tape::{cursor::Cursor, cursorlike::Cursorlike, familiar::Familiar, tapelike::Tapelike, Tape}, Alteration};

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

        let context = cursor.read_range(self.page_range());
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

    pub fn page_range(&self) -> Range<usize> {
        tracing::trace!("Page range is: {}..{}", self.offset(), self.offset() + self.length - 1);
        (self.offset()..(self.offset() + self.length - 1)).into()
    }

    pub fn page(&self) -> usize {
        self.position / self.length
    }

    pub fn total_pages(&self) -> usize {
        self.tape_length / self.length // TODO: potential off-by-one
    }

    pub fn table_state(&self) -> TableState {
        TableState::default()
            .with_selected(self.position_in_page())
    }

    pub fn header_row(&self) -> Row {
        Row::new(vec!["Address", "Instruction", "Hash"])
            .style(Style::new().bold())
    }

    pub fn header(&self) -> Text {
        Text::from(
                // format!("{}, {}, {}, {:?}, {}, {}, {}",
                //     self.position, self.position_in_page(), self.offset(), self.page_range(), self.page(), self.total_pages(), self.length
                // )
                format!("Tape: POS: {:#07X} ({}/{}), EOT: {:#06X}",
                    self.position, self.page(), self.total_pages(), self.tape_length
                )
            )
    }

    pub fn footer(&self) -> Text {
        Text::from("Footer here".to_string())
    }

    pub fn rows(&self) -> Vec<Row> {
        let mut ret : Vec<Row> = self.context.clone().into_iter().enumerate().map(|(idx, e)| {
            // we have the alteration + context from `state` proper, we need to prepare the context
            // rows here, and add the header/footer rows (not sections) later.
            Row::new(vec![
                format!("{:#06X}", idx + self.offset()),
                e.to_string(),
                "Running Hash".to_string()
            ])
        }).collect();

        for addr in ret.len()..self.length {
            ret.push(Row::new(vec![format!("{:#06X}", self.offset() + addr)]));
        }

        ret
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
        assert_eq!(trs.page_range(), (0..DEFAULT_TAPE_READER_LENGTH).into());
        assert_eq!(trs.offset(), 0);
        assert_eq!(trs.position_in_page(), 0);
        assert_eq!(trs.page(), 0);
        trs.position = 1;
        assert_eq!(trs.page_range(), (0..DEFAULT_TAPE_READER_LENGTH).into());
        assert_eq!(trs.offset(), 0);
        assert_eq!(trs.position_in_page(), 1);
        assert_eq!(trs.page(), 0);
        trs.position = 0x1F;
        assert_eq!(trs.page_range(), (0..DEFAULT_TAPE_READER_LENGTH).into());
        assert_eq!(trs.offset(), 0);
        assert_eq!(trs.position_in_page(), 0x1F);
        assert_eq!(trs.page(), 0);
        trs.position = 0x20;
        assert_eq!(trs.page_range(), (DEFAULT_TAPE_READER_LENGTH..(2 * DEFAULT_TAPE_READER_LENGTH)).into());
        assert_eq!(trs.offset(), 0x20);
        assert_eq!(trs.page(), 1);
        assert_eq!(trs.position_in_page(), 0);
        trs.position = 0x21;
        assert_eq!(trs.page_range(), (DEFAULT_TAPE_READER_LENGTH..(2 * DEFAULT_TAPE_READER_LENGTH)).into());
        assert_eq!(trs.offset(), 0x20);
        assert_eq!(trs.page(), 1);
        assert_eq!(trs.position_in_page(), 1);


    }
    
}
