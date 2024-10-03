use ratatui::layout::Direction;
use ratatui::prelude::*;
use ratatui::widgets::Borders;

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

pub struct InfoSection {
}

impl InfoSection {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl StatefulWidget for &InfoSection {
    type State = ();
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = LAYOUT.split(area);

        let ticker_widget = Placeholder::of_size(chunks[0].width, chunks[0].height).borders(Borders::NONE); // &mut info_section::InfoSection::new();
        ticker_widget.render(chunks[0], buf);//, state);

        let pgn_section = &mut PGNSection::new();
        pgn_section.render(chunks[1], buf, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 16);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let board_section = &mut InfoSection::new();
        board_section.render(rect, &mut buffer, &mut ());

        let mut expected = Buffer::with_lines(vec![
            "                           Placeholder                          ",
            "                           Placeholder                          ",
            "                           Placeholder                          ",
            "┌───────────────────────────────┐┌─────────────────────────────┐",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "└───────────────────────────────┘└─────────────────────────────┘",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }
}

