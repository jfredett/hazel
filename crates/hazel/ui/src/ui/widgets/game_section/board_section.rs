

use ratatui::layout::Direction;
use ratatui::prelude::*;
use ratatui::widgets::Borders;

use hazel_representation::board::simple::PieceBoard;
use crate::ui::widgets::placeholder::Placeholder;
use crate::ui::widgets::board::Board;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(16),
                Constraint::Fill(1)
            ].as_ref(),
        );
}

#[derive(Default)]
pub struct BoardSection<'a> {
    board_widget: Board<'a>,
}

impl From<PieceBoard> for BoardSection<'_> {
    fn from(board: PieceBoard) -> Self {
        Self {
            board_widget: Board::from(board)
        }
    }
}

impl Widget for &BoardSection<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = LAYOUT.split(area);

        self.board_widget.render(chunks[0], buf);

        let query_widget = Placeholder::of_size(chunks[1].width, chunks[1].height).borders(Borders::NONE);
        Widget::render(&query_widget, chunks[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_empty_board() {
        let rect = Rect::new(0, 0, 33, 18);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        let board = PieceBoard::default();

        let board_section = &mut BoardSection::from(board);
        board_section.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "                                 ",
            "a8 b8 c8 d8 e8 f8 g8 h8          ",
            "                                 ",
            "a7 b7 c7 d7 e7 f7 g7 h7          ",
            "                                 ",
            "a6 b6 c6 d6 e6 f6 g6 h6          ",
            "                                 ",
            "a5 b5 c5 d5 e5 f5 g5 h5          ",
            "                                 ",
            "a4 b4 c4 d4 e4 f4 g4 h4          ",
            "                                 ",
            "a3 b3 c3 d3 e3 f3 g3 h3          ",
            "                                 ",
            "a2 b2 c2 d2 e2 f2 g2 h2          ",
            "                                 ",
            "a1 b1 c1 d1 e1 f1 g1 h1          ",
            "           Placeholder           ",
            "           Placeholder           ",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }

    #[test]
    fn renders_startpos() {
        let rect = Rect::new(0, 0, 33, 18);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let mut board = PieceBoard::default();
        board.set_startpos();

        let board_section = &mut BoardSection::from(board);
        board_section.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            " R  N  B  Q  K  B  N  R          ",
            "a8 b8 c8 d8 e8 f8 g8 h8          ",
            " P  P  P  P  P  P  P  P          ",
            "a7 b7 c7 d7 e7 f7 g7 h7          ",
            "                                 ",
            "a6 b6 c6 d6 e6 f6 g6 h6          ",
            "                                 ",
            "a5 b5 c5 d5 e5 f5 g5 h5          ",
            "                                 ",
            "a4 b4 c4 d4 e4 f4 g4 h4          ",
            "                                 ",
            "a3 b3 c3 d3 e3 f3 g3 h3          ",
            " P  P  P  P  P  P  P  P          ",
            "a2 b2 c2 d2 e2 f2 g2 h2          ",
            " R  N  B  Q  K  B  N  R          ",
            "a1 b1 c1 d1 e1 f1 g1 h1          ",
            "           Placeholder           ",
            "           Placeholder           ",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        // this sucks
        let expected_content : Vec<String> = expected.content().iter().map(|x| x.symbol().to_string()).collect();
        let actual_content : Vec<String> = buffer.content().iter().map(|x| x.symbol().to_string()).collect();


        assert_eq!(actual_content.join(""), expected_content.join(""));
    }
}
