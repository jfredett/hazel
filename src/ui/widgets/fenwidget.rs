use ratatui::text::Text;
use ratatui::prelude::*;
use ratatui::widgets::Widget;


#[allow(unused_imports)]
use tracing::{debug, instrument};

use ratatui::buffer::Buffer;


use crate::ui::model::{
    pieceboard::PieceBoard,
    entry::Entry
};


#[derive(Debug, Default)]
pub struct FENWidget {
    board: PieceBoard
}

impl FENWidget {
    // TODO: This should be an actual From implementation, so I can build these from multiple
    // sources (e.g., Ply)
    pub fn from(entry: &Entry) -> Self {
        Self {
            board: entry.boardstate.clone()
        }
    }
}

impl Widget for &FENWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let fen = self.board.to_fen();

        Text::styled(fen, Style::default().fg(Color::White).bg(Color::Black)).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn fenwidget_builds_from_entry() {
        // I need to mock this rather than use the real thing, since the real thing needs a
        // backend.
        let entry = Entry::default();
        let fenwidget = FENWidget::from(&entry);

        assert_eq!(fenwidget.board, entry.boardstate);
    }
    */
}
