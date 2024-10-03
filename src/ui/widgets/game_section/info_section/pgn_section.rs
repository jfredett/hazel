use ratatui::layout::Direction;
use ratatui::prelude::*;

use crate::ui::widgets::placeholder::Placeholder;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(51),
                Constraint::Percentage(49)
            ].as_ref(),
        );
}

pub struct PGNSection {
}

impl PGNSection {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl StatefulWidget for &PGNSection {
    type State = ();
    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let chunks = LAYOUT.split(area);

        let pgn = Placeholder::of_size(chunks[0].width, chunks[0].height); // &mut pgnwidget::new();
        pgn.render(chunks[0], buf);//, state);

        // This is shown as time-per-move in the sketch, but should be swappable for whatever I
        // like.
        let query = Placeholder::of_size(chunks[1].width, chunks[1].height);
        query.render(chunks[1], buf);//, state);
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

        let board_section = &mut PGNSection::new();
        board_section.render(rect, &mut buffer, &mut ());

        let mut expected = Buffer::with_lines(vec![
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
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "│          Placeholder          ││         Placeholder         │",
            "└───────────────────────────────┘└─────────────────────────────┘",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }
}

