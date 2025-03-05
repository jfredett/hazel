use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};


use crate::engine::driver::WitchHazel;

use super::widgets::tile::Tile;

enum Mode {
    Insert,
    Command
}

pub struct UI<'a> {
    flags: HashMap<String, bool>,
    engine: &'a WitchHazel<1024>,
    // UI
    mode: Mode,
    // I think I'm going to replace this with a different widget entirely, and build from there.
    tile: Tile,
}

impl<'a> Debug for UI<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hazel")
            .field("flags", &self.flags)
            .finish()
    }
}

impl<'a> UI<'a> {
    pub fn with_handle(engine: &'a WitchHazel<1024>) -> Self {
        Self {
            flags: HashMap::new(),
            engine,
            mode: Mode::Command,
            tile: Tile::new(),
        }
    }

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

    pub fn set_flag(&mut self, flag: &str, value: bool) {
        self.flags.insert(flag.to_string(), value);
    }

    pub fn check_flag(&self, flag: &str) -> bool {
        match self.flags.get(flag) {
            Some(value) => *value,
            None => false
        }
    }

    pub fn input_widget(&self) -> Block {
        Block::default()
            .title("Input")
            .borders(Borders::ALL)
    }


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

    #[tokio::test]
    async fn renders_as_expected() {
        let handle  = WitchHazel::<1024>::new().await;
        let mut hazel = UI::with_handle(&handle);

        let mut t = Terminal::new(TestBackend::new(64, 32)).unwrap();
        let _ = t.draw(|f| {
            hazel.render(f);
        });

        assert_debug_snapshot!(t.backend().buffer().clone());
    }
}
