use ratatui::text::Text;
use ratatui::prelude::*;
use ratatui::widgets::Widget;

#[allow(unused_imports)]
use tracing::{debug, instrument};

use ratatui::buffer::Buffer;

use crate::ui::model::entry::Entry;
use crate::board::simple::PieceBoard;
use crate::board::interface::query;


#[derive(Debug, Default)]
pub struct FEN {
    // TODO: This can probably be aliased so I don't have to use it's full legal name here, but I
    // am lazy and don't want to read the docs.
    fen: crate::notation::fen::FEN,
    style: Style,
    alignment: Alignment
}

impl From<&Entry> for FEN {
    fn from(entry: &Entry) -> Self {
        Self::new(query::to_fen(&entry.boardstate))
    }
}

impl From<Entry> for FEN {
    fn from(entry: Entry) -> Self {
        Self::new(query::to_fen(&entry.boardstate))
    }
}

impl From<&PieceBoard> for FEN {
    fn from(board: &PieceBoard) -> Self {
        Self::new(query::to_fen(board))
    }
}

impl From<PieceBoard> for FEN {
    fn from(board: PieceBoard) -> Self {
        Self::new(query::to_fen(&board))
    }
}

impl FEN {
    pub fn new(fen: crate::notation::fen::FEN) -> Self {
        Self {
            fen,
            style: Style::default(),
            alignment: Alignment::Left
        }
    }

    pub fn set_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    #[inline(always)]
    pub fn center(self) -> Self {
        self.alignment(Alignment::Center)
    }
}

impl Widget for &FEN {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let fenstring = format!("{}", self.fen);
        Text::styled(fenstring, self.style).alignment(self.alignment).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::START_POSITION_FEN;

    use super::*;

    #[test]
    fn renders_empty_fen_correctly() {
        let rect = Rect::new(0, 0, 64, 1);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        let board = PieceBoard::default();

        let fen_widget = &FEN::from(board);
        fen_widget.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "8/8/8/8/8/8/8/8 w KQkq - 0 1                                    "
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }

    #[test]
    fn render_startpos_fen_correctly() {
        let rect = Rect::new(0, 0, 64, 1);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let mut board = PieceBoard::default();
        board.set_fen(&crate::notation::fen::FEN::new(START_POSITION_FEN));

        let fen_widget = &FEN::from(board);
        fen_widget.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1        "
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);

    }
}
