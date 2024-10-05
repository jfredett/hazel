pub mod boardwidget;
pub mod fenwidget;
pub mod board;


use ratatui::layout::Direction;
use ratatui::prelude::*;
use ratatui::widgets::Borders;

use crate::ui::model::pieceboard::PieceBoard;
use crate::ui::widgets::placeholder::Placeholder;
use crate::ui::widgets::game_section::board_section::board::Board;

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

pub struct BoardSection<'a> {
    board_widget: Board<'a>
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

        let query_widget = Placeholder::of_size(chunks[1].width, chunks[1].height).borders(Borders::NONE)  ; // &mut fenwidget::FenWidget::new();
        query_widget.render(chunks[1], buf);//, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 33, 18);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        let board = PieceBoard::default();

        let board_section = &mut BoardSection::from(board);
        board_section.render(rect, &mut buffer);

        // NOTE: These are designed to be used in conjunction with a checkerboard, not this
        // monochrome one. I am not sure how to render this just yet in ratatui, but when I get
        // there, which symbol to use to represent white/black will change based on the color of
        // the square the piece is on, because of how it outlines. It's also why the pieces look
        // backwards right now (as though you are playing as black)
        let mut expected = Buffer::with_lines(vec![
            "┌───┬───┬───┬───┬───┬───┬───┬───┐",
            "│ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │",
            "├───┼───┼───┼───┼───┼───┼───┼───┤",
            "│ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │ ♟ │",
            "├───┼───┼───┼───┼───┼───┼───┼───┤",
            "│   │   │   │   │   │   │   │   │",
            "├───┼───┼───┼───┼───┼───┼───┼───┤",
            "│   │   │   │   │   │   │   │   │",
            "├───┼───┼───┼───┼───┼───┼───┼───┤",
            "│   │   │   │   │   │   │   │   │",
            "├───┼───┼───┼───┼───┼───┼───┼───┤",
            "│   │   │   │   │   │   │   │   │",
            "├───┼───┼───┼───┼───┼───┼───┼───┤",
            "│ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │",
            "├───┼───┼───┼───┼───┼───┼───┼───┤",
            "│ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │",
            "└───┴───┴───┴───┴───┴───┴───┴───┘",
            "           Placeholder           ",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }

    use ratatui::widgets::Block;

    #[test]
    fn pinky() {
        let b = Block::new().title(Span::styled("Pinky", Style::default().bg(Color::Magenta))).borders(Borders::ALL);
        let rect = Rect::new(0, 0, 33, 18);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));


        b.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "┌───────────────────────────────┐",
            "│Pinky                          │",
            "└───────────────────────────────┘",
        ]);
        expected.set_style(rect, Style::default().fg(Color::Magenta).bg(Color::Black));

        assert_eq!(buffer, expected);
    }
}

/*
─
Row::new(vec!["┌", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┬", "─", "─", "─", "┐"]),
Row::new(vec!["│", " ", "♜", " ", "│", " ", "♞", " ", "│", " ", "♝", " ", "│", " ", "♛", " ", "│", " ", "♚", " ", "│", " ", "♝", " ", "│", " ", "♞", " ", "│", " ", "♜", " ", "│"]),
Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
Row::new(vec!["│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│", " ", "♟", " ", "│"]),
Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
Row::new(vec!["│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│"]),
Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
Row::new(vec!["│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│"]),
Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
Row::new(vec!["│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│"]),
Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
Row::new(vec!["│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│", " ", " ", " ", "│"]),
Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
Row::new(vec!["│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│", " ", "♙", " ", "│"]),
Row::new(vec!["├", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┼", "─", "─", "─", "┤"]),
Row::new(vec!["│", " ", "♖", " ", "│", " ", "♘", " ", "│", " ", "♗", " ", "│", " ", "♕", " ", "│", " ", "♔", " ", "│", " ", "♗", " ", "│", " ", "♘", " ", "│", " ", "♖", " ", "│"]),
Row::new(vec!["└", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┴", "─", "─", "─", "┘"]),


*/
