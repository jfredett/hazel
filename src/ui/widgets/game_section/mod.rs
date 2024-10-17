pub mod board_section;
pub mod info_section;

use ratatui::layout::Direction;
use ratatui::prelude::*;

use board_section::BoardSection;
use info_section::InfoSection;

use crate::board::simple::PieceBoard;

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

#[derive(Default)]
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
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        let board = PieceBoard::default();

        let game_section = GameSectionLayout::new(board);
        game_section.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "               Placeholder                                      ",
            "               Placeholder              a8 b8 c8 d8 e8 f8 g8 h8 ",
            "               Placeholder                                      ",
            "┌──────────────────┐┌──────────────────┐a7 b7 c7 d7 e7 f7 g7 h7 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a6 b6 c6 d6 e6 f6 g6 h6 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a5 b5 c5 d5 e5 f5 g5 h5 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a4 b4 c4 d4 e4 f4 g4 h4 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a3 b3 c3 d3 e3 f3 g3 h3 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a2 b2 c2 d2 e2 f2 g2 h2 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a1 b1 c1 d1 e1 f1 g1 h1 ",
            "└──────────────────┘└──────────────────┘       Placeholder      ",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }
}

