use ratatui::layout::Direction;
use ratatui::prelude::*;
use ratatui::widgets::Borders;

use hazel::notation::pgn::PGN;
use crate::ui::widgets::placeholder::Placeholder;

mod pgn_section;
use pgn_section::PGNSection;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Max(3),
                Constraint::Fill(1)
            ].as_ref(),
        );
}

#[derive(Default)]
pub struct InfoSection {
    pgn: PGNSection,
}

impl InfoSection {
    pub fn new(pgn: PGN) -> Self {
        Self {
            pgn: PGNSection::new(pgn)
        }
    }
}

impl Widget for &InfoSection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = LAYOUT.split(area);

        let ticker_widget = Placeholder::of_size(chunks[0].width, chunks[0].height).borders(Borders::NONE); // &mut info_section::InfoSection::new();
        Widget::render(&ticker_widget, chunks[0], buf);//, state);

        self.pgn.render(chunks[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_game() -> PGN {
        PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap()
    }

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 16);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let board_section = &mut InfoSection::new(example_game());
        board_section.render(rect, &mut buffer);

        insta::assert_debug_snapshot!(buffer);
    }
}

