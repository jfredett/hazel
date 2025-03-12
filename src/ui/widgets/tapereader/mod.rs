use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Layout, Rect};

use ratatui::widgets::{StatefulWidget, TableState, Widget};
use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Row, Table},
};

use crate::types::tape::familiar::state::tape_reader_state::TapeReaderState;

use super::placeholder::Placeholder;

// I had hoped to maybe have this be a familiar on a tape and not actually the tape itself, but
// I think it has to be a tape because it needs to see the context to print it all.
//
// ideally it's a Alter + Default object that also has a render function. It then creates a bunch
// of familiars it can sync as a group on the same tape.
//
// I suppose I could structure it as two structs, one which has a whole position that renders the
// context of the tape, then the tapereader is a bunch of familiars that get synced together?
// 
// Maybe could be interesting to allow a familiar that gets a small slice of 'context', could be
// useful for some kind of sliding-window analysis?
//
// Something like tape.read_range(), then have TapeReader have a 'context' value, it reads the
// range, then does whatever stuff it needs to 
//
// 

/// This is just a method-container and lifetime marker, the state is provided externally via a
/// familiar on the tape.
pub struct TapeReaderWidget {
    pub desired_position: usize
}

impl Default for TapeReaderWidget {
    fn default() -> Self {
        TapeReaderWidget {
            desired_position: 0,
        }
    }
}

impl TapeReaderWidget {
    pub fn layout(&self) -> Layout {
        Layout::horizontal([
            Constraint::Length(1), // header
            Constraint::Min(1), // code seciton
            Constraint::Length(1), // footer
        ])
    }

    pub fn advance(&mut self) {
        self.desired_position += 1;
    }

    pub fn rewind(&mut self) {
        self.desired_position = self.desired_position.saturating_sub(1);
    }
}


impl StatefulWidget for &TapeReaderWidget {
    type State = TapeReaderState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Columns widths are constrained in the same way as Layout...

        let chunks = self.layout().split(area);
        let header = chunks[0];
        let code = chunks[1];
        let footer = chunks[2];

        let widths = [
            Constraint::Length(8),
            Constraint::Length(32),
            Constraint::Length(32)
        ];

        // TODO: This is probably not identity, but some function of height
        //state.set_page_size((code.height - 6) as usize);

        let table = Table::new(state.rows(), widths)
            .column_spacing(1)
            .style(Style::new().white())
            .header(state.header_row())
            .block(state.title_block())
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");


        StatefulWidget::render(&Placeholder::of_size(header.width, header.height), header, buf, &mut ());
        StatefulWidget::render(&table, code, buf, &mut state.table_state());
        StatefulWidget::render(&Placeholder::of_size(footer.width, footer.height), footer, buf, &mut ());
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use ratatui::{buffer::Buffer, style::Color};

    use crate::{game::position::Position, notation::pgn::PGN};

    use super::*;

    // FIXME: familiar refactor
    // #[test]
    // fn renders_as_expected() {
    //     let rect = Rect::new(0, 0, 64, 32);
    //     let mut actual = Buffer::empty(rect);
    //     actual.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

    //     let pgn = PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap();
    //     let mut position = Position::new(pgn.current_position());
    //     let tape = position.tape.read().unwrap();
    //     let mut fam = tape.conjure::<TapeReaderState>();

    //     let tapereader = TapeReaderWidget::default();

    //     tapereader.render(rect, &mut actual, fam.get_mut());

    //     assert_debug_snapshot!(actual);
    // }
}
