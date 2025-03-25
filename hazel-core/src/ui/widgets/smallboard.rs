use ratatui::prelude::*;
use ratatui::widgets::{Widget, Block, Borders};
use ratatui::buffer::Buffer;

use crate::Query;
use crate::board::simple::PieceBoard;
use crate::notation::*;

/// 8x8 text-only no color board.
#[derive(Default)]
pub struct SmallBoard {
    board: PieceBoard
}

impl SmallBoard {
    // TODO: This should be an actual From implementation, so I can build these from multiple
    // sources (e.g., Ply)
    pub fn from(board: &PieceBoard) -> Self {
        Self {
            board: *board
        }
    }
}


fn eight_cells(direction: Direction) -> Layout {
    Layout::default()
        .direction(direction)
        .constraints(Constraint::from_maxes([1].repeat(8)))
}

impl Widget for &SmallBoard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cols = eight_cells(Direction::Horizontal).split(area);
        let rows = [
            eight_cells(Direction::Vertical).split(cols[0]),
            eight_cells(Direction::Vertical).split(cols[1]),
            eight_cells(Direction::Vertical).split(cols[2]),
            eight_cells(Direction::Vertical).split(cols[3]),
            eight_cells(Direction::Vertical).split(cols[4]),
            eight_cells(Direction::Vertical).split(cols[5]),
            eight_cells(Direction::Vertical).split(cols[6]),
            eight_cells(Direction::Vertical).split(cols[7])
        ];

        for i in 0..8 {
            for j in 0..8 {
                let sq = Square::from((i, j));
                let cell = Block::default()
                    .title(self.board.get(sq).to_string())
                    .borders(Borders::NONE);
                cell.render(rows[j][i], buf);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_empty_board() {
        let rect = Rect::new(0, 0, 8, 8);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let board = PieceBoard::default();
        let board_widget = &SmallBoard::from(&board);
        board_widget.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            "........",
            "........"
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }
}




