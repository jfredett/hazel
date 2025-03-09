use std::range::Range;
use ratatui::{layout::Rows, widgets::{Block, Row, TableState}};

use crate::{types::tape::{cursor::Cursor, cursorlike::Cursorlike, familiar::Familiar, tapelike::Tapelike, Tape}, Alter, Alteration};

#[derive(Clone)]
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
    pub fn update(&mut self, cursor: &Cursor<Tape>, range: Range<usize>) {
        let context = cursor.read_range(self.context_range());
        self.position = cursor.position();
        self.tape_length = cursor.length();
        self.context = context.to_vec();
    }


    // convert from a desired position in the buffer to the screen of code that needs to be
    // displayed
    // TODO: Would be nice to have an 'overlap' between screens by a few (configurable number of?)
    // rows.
    pub fn position_in_screen(&self) -> usize {
        self.position % self.length
    }

    pub fn current_offset(&self) -> usize {
        self.current_page() * self.length

    }

    pub fn context_range(&self) -> Range<usize> {
        (self.current_offset()..(self.current_offset() + self.length)).into()
    }

    pub fn current_page(&self) -> usize {
        (self.position / self.length)
    }

    pub fn total_pages(&self) -> usize {
        self.tape_length / self.length // TODO: potential off-by-one
    }

    pub fn table_state(&self) -> TableState {
        TableState::default()
            .with_offset(self.current_offset())
            .with_selected(self.position_in_screen())

    }
    pub fn title_block(&self) -> Block {
        Block::new()
            .title(
                format!("Tape: POS: {:#07X} ({}/{}), EOT: {:#07X}",
                    self.position, self.current_page(), self.total_pages(), self.tape_length
                )
            )
    }

    pub fn rows(&self) -> Vec<Row> {
        self.context.clone().into_iter().enumerate().map(|(idx, e)| {
            // we have the alteration + context from `state` proper, we need to prepare the context
            // rows here, and add the header/footer rows (not sections) later.
            Row::new(vec![
                format!("{:#07X}", idx + self.current_offset()),
                e.to_string(),
                "Running Hash".to_string()
            ])
        }).collect()
    }
}


impl Familiar<'_, Tape, TapeReaderState> {
    pub fn context_range(&self) -> Range<usize> {
        let mut start = self.cursor.position();
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

impl Cursorlike<Alteration> for Familiar<'_, Tape, TapeReaderState> {
    fn advance(&mut self) {
        self.cursor.advance();
        let range = self.context_range();
        self.state.update(&self.cursor, range)
    }

    fn length(&self) -> usize {
        self.cursor.length()
    }

    fn read(&self) -> &Alteration {
        self.cursor.read()
    }

    fn at_end(&self) -> bool {
        self.cursor.at_end()
    }

    fn position(&self) -> usize {
        self.cursor.position()
    }

    fn rewind(&mut self) {
        self.cursor.rewind();
        let range = self.context_range();
        self.state.update(&self.cursor, range)
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
