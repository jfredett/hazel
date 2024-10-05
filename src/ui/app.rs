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
use crate::ui::model::pieceboard::PieceBoard;

use super::widgets::tile::Tile;

enum Mode {
    Insert,
    Command
}

pub struct Hazel {
    flags: HashMap<String, bool>,
    entry: Entry,
    // UI
    mode: Mode,
    tile: Tile,
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
            mode: Mode::Command,
            entry: stockfish(),
            tile: Tile::new(),
        };

        s.entry.exec(UCIMessage::UCI);
        s.entry.exec(UCIMessage::IsReady);
        debug!("setting startpos");
        s.entry.exec(UCIMessage::Position("startpos moves d2d4".to_string(), vec![]));
        debug!("setting startpos done");

        // s.entry.boardstate.set_startpos();
        s.entry.boardstate = PieceBoard::from_fen("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR");

        return s;
    }

    #[instrument]
    pub fn handle_events(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match self.mode {
                // Probably insert mode is just handled by the tile wholesale?
                Mode::Insert => {
                    match key.code {
                        KeyCode::Esc => {
                            self.mode = Mode::Command;
                        },
                        KeyCode::Char(c) => {
                            self.tile.handle_input(c);
                        },
                        KeyCode::Backspace => {
                            self.tile.handle_backspace();
                        },
                        KeyCode::Enter => {
                            self.tile.handle_enter();
                        },
                        _ => {
                        }
                    }
                },
                // Command mode will eventually select the tile you want/start new tiles, etc.
                Mode::Command => {
                    match key.code {
                        KeyCode::Char('i') => {
                            self.mode = Mode::Insert;
                        },
                        KeyCode::Char('q') => {
                            self.set_flag("exit", true);
                        },
                        _ => {
                        }
                    }
                }
            }
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
        self.tile.set_state(self.entry.boardstate.clone());
        frame.render_widget(&self.tile, Rect::new(0,0,64,32));
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
            "│                          Placeholder                         │",
            "┌──────────────────────────────────────────────────────────────┐",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "└──────────────────────────────────────────────────────────────┘",
            "$>                                                              ",
        ]);

        // Ignore style differences for now
        let mut actual = t.backend().buffer().clone();
        actual.set_style(Rect::new(0, 0, 64, 32), Style::default().fg(Color::White).bg(Color::Black));
        expected.set_style(Rect::new(0, 0, 64, 32), Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(actual, expected);
    }
}
