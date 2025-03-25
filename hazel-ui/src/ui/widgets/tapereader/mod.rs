use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, widgets::{Block, StatefulWidget, Table, Widget}};
use hazel::types::tape::familiar::state::tape_reader_state::TapeReaderState;

#[derive(Default)]
pub struct TapeReaderWidget {
    pub desired_position: usize
}

impl TapeReaderWidget {
    pub fn layout(&self) -> Layout {
        Layout::vertical([
            Constraint::Length(1), // header
            Constraint::Length(40), // code seciton
            Constraint::Min(1), // footer
        ])
    }

    pub fn advance(&mut self) {
        tracing::trace!(target="hazel::ui::events", "advancing to desired position: {:#04X} from {:#04X}", self.desired_position + 1, self.desired_position);
        self.desired_position += 1;
    }

    pub fn rewind(&mut self) {
        tracing::trace!(target="hazel::ui::events", "rewinding to desired position: {:#04X} from {:#04X}", self.desired_position.saturating_sub(1), self.desired_position);
        self.desired_position = self.desired_position.saturating_sub(1);
    }
}


impl StatefulWidget for &TapeReaderWidget {
    type State = TapeReaderState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Columns widths are constrained in the same way as Layout...

        let chunks = self.layout().split(area);
        let header = chunks[0];
        let tape_area = chunks[1];
        let footer = chunks[2];

        let widths = [
            Constraint::Length(8),
            Constraint::Length(32),
            Constraint::Length(32)
        ];

        // TODO: This is probably not identity, but some function of height
        // state.set_page_size((code.height - 8) as usize);

        let table = Table::new(state.rows(), widths)
            .column_spacing(1)
            .style(Style::new().white())
            .block(Block::bordered())
            .header(state.header_row())
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");

        Widget::render(state.header(), header, buf);
        StatefulWidget::render(&table, tape_area, buf, &mut state.table_state());
        Widget::render(state.footer(), footer, buf);
    }
}

#[cfg(test)]
mod tests {
    // use insta::assert_debug_snapshot;
    // use ratatui::{buffer::Buffer, style::Color};

    // use hazel::{game::position::Position, notation::pgn::PGN};

    // use super::*;

    // // FIXME: familiar refactor
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
