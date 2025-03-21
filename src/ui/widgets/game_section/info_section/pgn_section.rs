use ratatui::layout::Direction;
use ratatui::prelude::*;

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


// this should be a stateful widget, it's struct-state should just be style information
// information, the provided state will be the current position of the cursor and what to display
// in the box.
//
// The parent will hold t

#[derive(Default)]
pub struct PGNSection {
    current_position: usize,
    pgn: PGN,
}

impl PGNSection {
    pub fn new(pgn: PGN) -> Self {
        Self { current_position: 0, pgn }
    }

    pub fn set_position(&mut self, position: usize) {
        self.current_position = position;
    }

    pub fn advance(&mut self) {
        self.current_position += 1;
    }

    pub fn retreat(&mut self) {
        self.current_position -= 1;
    }

    pub fn render_variation(&self, area: Rect, buf: &mut Buffer) {
        let mut pgn = self.pgn.clone();
        let mut s = vec![]; 
        let mut f = pgn.familiar();

        f.advance_until(|f_inner| {

            if !f_inner.move_string().is_empty() {
                s.push(f_inner.move_string());
            }

            f_inner.cursor_position() == self.current_position
        });

        let text = s.chunks(2).enumerate().map(|(idx, chunk)| {
            format!("{:2}. {}", idx + 1, chunk.join(" "))
        }).collect::<Vec<String>>().join("\n");

        Text::styled(text, Style::default()).render(area, buf);
    }
}

impl Widget for &PGNSection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = LAYOUT.split(area);

        self.render_variation(chunks[0], buf);

        let query = Placeholder::of_size(chunks[1].width, chunks[1].height);
        Widget::render(&query, chunks[1], buf);
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
        board_section.set_position(16);
        board_section.render(rect, &mut buffer);

        assert_debug_snapshot!(buffer)
    }
}

