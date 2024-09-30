use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

use crate::ui::widgets::game_section::board_section::fenwidget::FENWidget;
use crate::ui::widgets::game_section::board_section::boardwidget::BoardWidget;

use tracing::{debug, instrument};

use crate::uci::UCIMessage;
use crate::ui::model::entry::{Entry, stockfish};
use crate::engine::Engine;

use super::widgets::tile::Tile;

pub struct Hazel {
    flags: HashMap<String, bool>,
    entry: Entry,
}

impl Debug for Hazel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hazel")
            .field("flags", &self.flags)
            .finish()
    }
}

impl Hazel {
    #[instrument]
    pub fn new() -> Self {
        let mut s = Self {
            flags: HashMap::new(),
            entry: stockfish(),
        };

        s.entry.exec(UCIMessage::UCI);
        s.entry.exec(UCIMessage::IsReady);
        debug!("setting startpos");
        s.entry.exec(UCIMessage::Position("startpos".to_string(), vec![]));
        debug!("setting startpos done");

        s.entry.boardstate.set_startpos();

        return s;
    }

    #[instrument]
    pub fn handle_events(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => self.set_flag("exit", true),
                _ => {}
            }
            // nyi
        }
    }

    #[instrument]
    pub fn set_flag(&mut self, flag: &str, value: bool) {
        self.flags.insert(flag.to_string(), value);
    }

    #[instrument]
    pub fn check_flag(&self, flag: &str) -> bool {
        match self.flags.get(flag) {
            Some(value) => *value,
            None => false
        }
    }

    #[instrument]
    pub fn fen_widget(&self) -> FENWidget {
        FENWidget::from(&self.entry)
    }

    #[instrument]
    pub fn board_widget(&self) -> BoardWidget {
        BoardWidget::from(&self.entry.boardstate)
    }

    #[instrument]
    pub fn input_widget(&self) -> Block {
        Block::default()
            .title("Input")
            .borders(Borders::ALL)
    }



    #[instrument]
    pub fn render(&mut self, frame: &mut Frame) {
        let tile = Tile::new();
        frame.render_stateful_widget(&tile, Rect::new(0,0,64,32), self);
    }
}

#[cfg(test)]
mod tests {
    use std::process::Termination;

    use backend::TestBackend;

    use super::*;

    #[test]
    fn placeholder() {
        let mut hazel = Hazel::new();

        let mut t = Terminal::new(TestBackend::new(64, 32)).unwrap();
        let _ = t.draw(|f| {
            hazel.render(f);
        });
        let buffer = t.backend().buffer();

        let expected = Buffer::with_lines(vec![
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
            "│                          Placeholder                         │",
            "┌──────────────────────────────────────────────────────────────┐",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "│                          Placeholder                         │",
            "└──────────────────────────────────────────────────────────────┘",
            "                           Placeholder                          ",
        ]);

        assert_eq!(buffer, &expected);
    }
}

