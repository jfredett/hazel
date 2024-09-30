pub mod outputwidget;

use ratatui::layout::Direction;
use ratatui::prelude::*;
use ratatui::widgets::Borders;
use crate::ui::app::Hazel;

use super::placeholder::Placeholder;


pub struct EngineIOSection {
}


lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(13),
                Constraint::Max(1),
            ]
                .as_ref(),
        );
}

impl EngineIOSection {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl StatefulWidget for &EngineIOSection {
    type State = Hazel;
    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let chunks = LAYOUT.split(area);

        Placeholder::of_size(chunks[0].width, chunks[0].height).render(chunks[0], buf);
        Placeholder::of_size(chunks[1].width, chunks[1].height).borders(Borders::NONE).render(chunks[1], buf);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        let rect = Rect::new(0, 0, 64, 14);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let engine_io_section = &mut EngineIOSection::new();
        engine_io_section.render(rect, &mut buffer, &mut Hazel::new());

        let mut expected = Buffer::with_lines(vec![
            "┌──────────────────────────────────────────────────────────────┐",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "└──────────────────────────────────────────────────────────────┘",
            "                           Placeholder                          ",
        ]);

        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        assert_eq!(buffer, expected);
    }
}


