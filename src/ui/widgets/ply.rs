use crate::ply::Ply;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    symbols::border,
    text::Text,
    widgets::{
        block::Title,
        Block, Paragraph, Widget,
    },
};

pub struct PlyWidget { pub ply: Ply }

impl Widget for &PlyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Ply");
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);

        Paragraph::new(Text::from(self.ply.to_string()))
            .block(block)
            .render(area, buf);
    }
}

