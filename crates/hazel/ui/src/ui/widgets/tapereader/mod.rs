use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, text::Text, widgets::{Block, Row, StatefulWidget, Table, TableState, Widget}};
use hazel_representation::game::chess::state::tape_reader_state::TapeReaderState;

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


    pub fn header_row(&self) -> Row {
        Row::new(vec!["Address", "Instruction", "Hash"])
            .style(Style::new().bold())
    }

    pub fn header(&self, state: &mut TapeReaderState) -> Text {
        let (pos, page, total_pages, len) = state.header();
        Text::from(
            format!("Tape: POS: {:#07X} ({}/{}), EOT: {:#06X}",
                pos, page, total_pages, len
            )
        )
    }

    pub fn table_state(&self, state: &mut TapeReaderState) -> TableState {
        TableState::default()
            .with_selected(state.position_in_page())
    }

    pub fn footer(&self) -> Text {
        Text::from("Footer here".to_string())
    }

    pub fn rows(&self, state: &mut TapeReaderState) -> Vec<Row> {
        let mut ret : Vec<Row> = state.context.clone().into_iter().enumerate().map(|(idx, e)| {
            // we have the alteration + context from `state` proper, we need to prepare the context
            // rows here, and add the header/footer rows (not sections) later.
            Row::new(vec![
                format!("{:#06X}", idx + state.offset()),
                e.to_string(),
                "Running Hash".to_string()
            ])
        }).collect();

        for addr in ret.len()..state.length {
            ret.push(Row::new(vec![format!("{:#06X}", state.offset() + addr)]));
        }

        ret
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

        let table = Table::new(self.rows(state), widths)
            .column_spacing(1)
            .style(Style::new().white())
            .block(Block::bordered())
            .header(self.header_row())
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(">>");

        Widget::render(self.header(state), header, buf);
        StatefulWidget::render(&table, tape_area, buf, &mut self.table_state(state));
        Widget::render(self.footer(), footer, buf);
    }
}

#[cfg(test)]
mod tests {
    // use insta::assert_debug_snapshot;
    // use ratatui::{buffer::Buffer, style::Color};

    // use hazel_representation::{game::position::Position, notation::pgn::PGN};

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
