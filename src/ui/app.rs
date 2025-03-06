use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;


use crate::engine::driver::WitchHazel;
use crate::notation::ben::BEN;
use crate::types::tape::Tape;
use crate::ui::widgets::tapereader::*;
use crate::game::chess::position::Position;


enum Mode {
    Insert,
    Command
}

pub struct UI<'a> {
    flags: HashMap<String, bool>,
    engine: &'a WitchHazel<1024>,
    // UI
    mode: Mode,
    tapereader: TapeReaderWidget
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
            tapereader: TapeReaderWidget::default()
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
                            // self.tile.handle_input(c);
                        },
                        KeyCode::Backspace => {
                            // self.tile.handle_backspace();
                        },
                        KeyCode::Enter => {
                            // self.tile.handle_enter();
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

    pub fn render(&'a self, frame: &mut Frame) {
        let position = Position::new(BEN::start_position());
        let tape = position.tape.read().unwrap();

        // let mut tapereaderstate = tape.conjure::<TapeReaderState<'a>>();
        let mut tapereaderfamiliar = crate::types::tape::familiar::conjure::<TapeReaderState<'a>, Tape>(&tape);

        frame.render_stateful_widget(&self.tapereader, Rect::new(0,0,100,100), tapereaderfamiliar.get_mut());
    }
}

#[cfg(test)]
#[cfg_attr(test, mutants::skip)]
mod tests {
    use std::process::Termination;

    use ratatui::backend::TestBackend;
    use insta::assert_debug_snapshot;
    use ratatui::Terminal;

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
