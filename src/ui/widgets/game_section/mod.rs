pub mod board_section;
pub mod info_section;

use ratatui::layout::Direction;
use ratatui::prelude::*;
use crate::ui::app::Hazel;

use board_section::BoardSection;
use info_section::InfoSection;

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

pub struct GameSectionLayout {
}


impl GameSectionLayout {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl StatefulWidget for &GameSectionLayout {
    type State = Hazel;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chunks = LAYOUT.split(area);

        let info_section = &mut InfoSection::new();
        info_section.render(chunks[0], buf, state);

        let board_section = &mut BoardSection::new();
        board_section.render(chunks[1], buf, state);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        let rect = Rect::new(0, 0, 64, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let game_section = &mut GameSectionLayout::new();
        game_section.render(rect, &mut buffer, &mut Hazel::new());

        let mut expected = Buffer::with_lines(vec![
            "           Placeholder           ┌─────────────────────────────┐",
            "           Placeholder           │         Placeholder         │",
            "           Placeholder           │         Placeholder         │",
            "┌───────────────┐┌──────────────┐│         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder ││         Placeholder         │",
            "│  Placeholder  ││  Placeholder │└─────────────────────────────┘",
            "└───────────────┘└──────────────┘          Placeholder          ",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }
}

