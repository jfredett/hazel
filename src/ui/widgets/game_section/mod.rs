pub mod board_section;
pub mod info_section;

use ratatui::layout::Direction;
use ratatui::prelude::*;

use board_section::BoardSection;
use info_section::InfoSection;

use crate::ui::model::pieceboard::PieceBoard;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(40),
                Constraint::Length(24)
            ].as_ref(),
        );
}

pub struct GameSectionLayout<'a> {
    info_section: InfoSection,
    board_section: BoardSection<'a>
}


impl GameSectionLayout<'_> {
    pub fn new(board: PieceBoard) -> Self {
        Self {
            info_section: InfoSection::new(),
            board_section: BoardSection::from(board),
        }
    }
}

impl Widget for &GameSectionLayout<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = LAYOUT.split(area);

        self.info_section.render(chunks[0], buf);
        self.board_section.render(chunks[1], buf);
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
        let board = PieceBoard::default();

        let game_section = GameSectionLayout::new(board);
        game_section.render(rect, &mut buffer);

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

