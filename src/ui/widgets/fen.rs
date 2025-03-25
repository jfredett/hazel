use ratatui::{buffer::Buffer, prelude::*, text::Text, widgets::Widget};

use crate::notation::ben::BEN;


#[derive(Debug, Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct FEN {
    fen: BEN,
    style: Style,
    alignment: Alignment
}

impl FEN {
    pub fn new(fen: BEN) -> Self {
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
    use crate::board::simple::PieceBoard;

    use super::*;

    #[test]
    fn renders_empty_fen_correctly() {
        let rect = Rect::new(0, 0, 64, 1);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let fen_widget = &FEN::new(BEN::empty());
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

        let fen_widget = &FEN::new(BEN::new(START_POSITION_FEN));
        fen_widget.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1        "
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);

    }
}
