use ratatui::layout::Direction;
use ratatui::prelude::*;

use crate::game::variation::Variation;
use crate::notation::pgn::PGN;
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

#[derive(Default)]
pub struct PGNSection {
    pgn: PGN,
}

impl PGNSection {
    pub fn new(pgn: PGN) -> Self {
        Self { pgn }
    }
}

impl Widget for &PGNSection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = LAYOUT.split(area);

        let pgn = Placeholder::of_size(chunks[0].width, chunks[0].height);
        pgn.render(chunks[0], buf);

        let query = Placeholder::of_size(chunks[1].width, chunks[1].height);
        query.render(chunks[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::notation::pgn::PGN;

    use super::*;

    fn example_game() -> PGN {
        PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap()
    }

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 16);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let board_section = &mut PGNSection::new(example_game());
        board_section.render(rect, &mut buffer);

        assert_debug_snapshot!(buffer)
    }
}

