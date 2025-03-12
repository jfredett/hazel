use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, StatefulWidget, Widget};
use ratatui::Frame;
use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};


use crate::engine::driver::{GetPosition, HazelResponse, WitchHazel};
use crate::notation::ben::BEN;
use crate::types::tape::cursorlike::Cursorlike;
use crate::types::tape::familiar::state::tape_reader_state::TapeReaderState;
use crate::types::tape::familiar::{self, Familiar};
use crate::types::tape::Tape;
use crate::ui::widgets::tapereader::*;
use crate::game::chess::position::Position;
use crate::types::tape::tapelike::Tapelike;

enum Mode {
    Insert,
    Command
}

pub struct UI<'a> {
    flags: HashMap<String, bool>,
    engine: &'a WitchHazel<1024>,
    // UI
    mode: Mode,
    // tapereader: TapeReaderWidget,
    tuiloggerstate: TuiWidgetState,
}

impl Debug for UI<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hazel")
            .field("flags", &self.flags)
            .finish()
    }
}

impl<'a> UI<'a> {
    pub async fn with_handle(engine: &'a WitchHazel<1024>) -> Self {
        // TODO: Extract this to a function that we call to update the Option<Fam> every render.
        // engine.send(Box::new(GetPosition)).await;
        // let response = engine.read().await;

        // // I need a function to ask for a tapefamiliar over a position.
        // let tape = if let Some(HazelResponse::Position(Some(pos))) = response {
        //     let source_tape = pos.tape.read().unwrap();
        //     Some(source_tape.clone())
        // } else {
        //     None
        // };

// mut tapereaderfamiliar : Familiar<'a, Tape, TapeReaderState> =

        Self {
            flags: HashMap::new(),
            engine,
            mode: Mode::Command,
            // tapereader: TapeReaderWidget::default(),
            tuiloggerstate: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace)
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

                        KeyCode::Down => {
                            // self.tapereader.advance();
                        }
                        KeyCode::Up => {
                            // self.tapereader.rewind();
                        }
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

    pub fn render(&'a self, frame: &mut Frame<'_>) {

        let chunks = LAYOUT.split(frame.area());

        // this will get handled by the engine at some point, so we won't create it here. We'll
        // just ask for the current TapeReaderState for whatever thing we want directly from the
        // WitchHazel engine instance

        // FIXME: this should seek to a position chosen in the UI via up/down arror
        // self.tapereaderfamiliar.seek(self.tapereader.desired_position);
        // HACK: Initialization of this state is so weird.

        // StatefulWidget::render(&self.tapereader, chunks[0], frame.buffer_mut(), self.tapereaderfamiliar.get_mut());

        // let tlw = TuiLoggerWidget::default();
        let tlw = TuiLoggerSmartWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .state(&self.tuiloggerstate);

        Widget::render(tlw, chunks[1], frame.buffer_mut());
    }
}

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(
            [
                Constraint::Length(32),
                Constraint::Min(1),
            ].as_ref(),
        );
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
        let mut hazel = UI::with_handle(&handle).await;

        let mut t = Terminal::new(TestBackend::new(64, 32)).unwrap();
        let _ = t.draw(|f| {
            hazel.render(f);
        });

        assert_debug_snapshot!(t.backend().buffer().clone());
    }
}
