pub mod boardwidget;
pub mod fenwidget;


use ratatui::layout::Direction;
use ratatui::prelude::*;
use ratatui::widgets::Borders;

use crate::ui::widgets::placeholder::Placeholder;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Fill(1),
                Constraint::Max(1)
            ].as_ref(),
        );
}

pub struct BoardSection {
}

impl BoardSection {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl StatefulWidget for &BoardSection {
    type State = ();
    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let chunks = LAYOUT.split(area);

        let board_widget = Placeholder::of_size(chunks[0].width, chunks[0].height); // &mut boardwidget::BoardWidget::new();
        board_widget.render(chunks[0], buf);//, state);
        let query_widget = Placeholder::of_size(chunks[1].width, chunks[1].height).borders(Borders::NONE); // &mut fenwidget::FenWidget::new();
        query_widget.render(chunks[1], buf);//, state);
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

        let board_section = &mut BoardSection::new();
        board_section.render(rect, &mut buffer, &mut ());

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
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "└──────────────────────────────────────────────────────────────┘",
            "                           Placeholder                          ",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }
}

