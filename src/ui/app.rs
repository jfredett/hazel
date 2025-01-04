use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

use tracing::instrument;

use crate::engine::driver::Driver;

use super::widgets::tile::Tile;

enum Mode {
    Insert,
    Command
}

pub struct Hazel {
    flags: HashMap<String, bool>,
    engine: Driver,
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
        let s = Self {
            flags: HashMap::new(),
            engine: Driver::new(),
            mode: Mode::Command,
            tile: Tile::new(),
        };

        /*
        let startup_commands = vec![
            UCIMessage::UCI,
            UCIMessage::IsReady,
            UCIMessage::Position("startpos".to_string(), vec!["d2d4".to_string()]),
        ];
        */

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
    pub fn input_widget(&self) -> Block {
        Block::default()
            .title("Input")
            .borders(Borders::ALL)
    }


    #[instrument(skip(self, frame))]
    pub fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(&self.tile, Rect::new(0,0,64,32));
    }
}

#[cfg(test)]
#[cfg_attr(test, mutants::skip)]
mod tests {
    use std::process::Termination;

    use backend::TestBackend;
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn renders_as_expected() {
        let mut hazel = Hazel::new();

        let mut t = Terminal::new(TestBackend::new(64, 32)).unwrap();
        let _ = t.draw(|f| {
            hazel.render(f);
        });

        assert_debug_snapshot!(t.backend().buffer().clone());
    }
}
