use ratatui::buffer::Buffer;
use ratatui::layout::{Direction, Layout, Rect};

use ratatui::widgets::{StatefulWidget, Widget};
use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Row, Table},
};

use crate::{Alter, Alteration};

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

#[derive(Clone)]
pub struct TapeReaderState<'a> {
    context: Option<&'a [Alteration]>, // I could alternately do this with a deque instead of re-grabbing the slice each update? This doesn't re-alloc on length change.
    length: usize, // the size of the slice to retrieve
    offset: usize // an offset at which the context window starts.
}

const DEFAULT_TAPE_READER_LENGTH : usize = 32;

impl Default for TapeReaderState<'_> {
    fn default() -> Self {
        TapeReaderState {
            context: None,
            length: DEFAULT_TAPE_READER_LENGTH,
            offset: 0
        }
    }
}


impl Alter for TapeReaderState<'_> {
    fn alter(&self, alter: Alteration) -> Self {
        let mut ret = self.clone();
        ret.alter_mut(alter);
        ret
    }

    fn alter_mut(&mut self, alter: Alteration) -> &mut Self {
        todo!()
    }
}

/// This is just a method-container and lifetime marker, the state is provided externally via a
/// familiar on the tape.
#[derive(Default)]
pub struct TapeReaderWidget {

}

impl TapeReaderWidget {
    pub fn layout(&self) -> Layout {
        let layout = Layout::horizontal([
            Constraint::Length(1), // header
            Constraint::Min(1), // code seciton
            Constraint::Length(1), // footer
        ]);
        layout
    }

}


impl<'a> StatefulWidget for &'a TapeReaderWidget {
    type State = TapeReaderState<'a>;

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

        let rows = if let Some(tape_slice) = state.context {
            tape_slice
        } else {
            &[]
        }.into_iter().enumerate().map(|(idx, e)| {
            // we have the alteration + context from `state` proper, we need to prepare the context
                // rows here, and add the header/footer rows (not sections) later.
            Row::new(vec![
                format!("ADDR: {}", idx + state.offset),
                e.to_string(),
                "Running Hash".to_string()
            ])
        });

        let table = Table::new(rows, widths)
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1)
            // You can set the style of the entire Table.
            .style(Style::new().blue())
            // It has an optional header, which is simply a Row always visible at the top.
            .header(
                Row::new(vec!["Address", "Instruction", "Hash"])
                    .style(Style::new().bold())
                    // To add space between the header and the rest of the rows, specify the margin
                    .bottom_margin(1),
            )
            // It has an optional footer, which is simply a Row always visible at the bottom.
            .footer(Row::new(vec!["PLACEHOLDER FOR FEN OF CURRENT POSITION"]))
            // As any other widget, a Table can be wrapped in a Block.
            .block(Block::new().title("Table"))
            // The selected row, column, cell and its content can also be styled.
            .row_highlight_style(Style::new().reversed())
            .column_highlight_style(Style::new().red())
            .cell_highlight_style(Style::new().blue())
            // ...and potentially show a symbol in front of the selection.
            .highlight_symbol(">>");

        Placeholder::of_size(header.width, header.height).render(header, buf);
        Widget::render(&table, code, buf);
        Placeholder::of_size(footer.width, footer.height).render(footer, buf);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use ratatui::{buffer::Buffer, style::Color};

    use crate::{game::position::Position, notation::pgn::PGN};

    use super::*;

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 32);
        let mut actual = Buffer::empty(rect);
        actual.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let pgn = PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap();
        let mut position = Position::new(pgn.current_position());
        let tape = position.tape.read().unwrap();
        let mut fam = tape.conjure::<TapeReaderState>();

        let tapereader = TapeReaderWidget::default();

        tapereader.render(rect, &mut actual, fam.get_mut());

        assert_debug_snapshot!(actual);
    }
}
