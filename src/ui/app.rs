use std::{cell::LazyCell, collections::HashMap, fmt::Debug, sync::RwLock};

use ratatui::{crossterm::event::{Event, KeyCode}, layout::{Constraint, Layout}, style::{Color, Style}, widgets::{Block, Borders, StatefulWidget, Widget}, Frame};
use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};

use crate::{constants::START_POSITION_FEN, engine::{driver::{GetPosition, HazelResponse, WitchHazel}, uci::UCIMessage}, types::tape::{cursorlike::Cursorlike, familiar::{self, resummon_on, state::tape_reader_state::TapeReaderState, Familiar, Quintessence}, Tape}, ui::widgets::tapereader::*};

enum Mode {
    Insert,
    Command
}

pub struct UI<'a> {
    flags: HashMap<String, bool>,
    engine: &'a WitchHazel<1024>,
    // UI
    mode: Mode,
    tapereader: TapeReaderWidget,
    tuiloggerstate: TuiWidgetState,
    // State
    tapereader_state: Option<Quintessence<TapeReaderState>>,
}

impl Debug for UI<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hazel")
            .field("flags", &self.flags)
            .finish()
    }
}

// Workflow:
// 0. handle_events
//  - Just update inputs, don't calculate any new state
// 1. update
//  - Update all state simultaneously
// 2. render
//  - render the new view.
impl<'a> UI<'a> {
    pub async fn with_handle(engine: &'a WitchHazel<1024>) -> Self {
        engine.send(Box::new(UCIMessage::Position(START_POSITION_FEN.to_owned(), vec!["d2d4".to_string(), "d7d5".to_string(), "c1f4".to_string()]))).await;

        Self {
            flags: HashMap::new(),
            engine,
            mode: Mode::Command,
            tapereader: TapeReaderWidget::default(),
            tuiloggerstate: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
            tapereader_state: None
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
                        KeyCode::Char(_c) => {
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
                            tracing::trace!("Advancing TapeReader");
                            self.tapereader.advance();
                        }
                        KeyCode::Up => {
                            tracing::trace!("Rewinding TapeReader");
                            self.tapereader.rewind();
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

    pub async fn update(&mut self) {
        // If we have an active state, we want to update it, if we don't, we want to check to see
        // if the Engine has a position we can grab. Eventually we might point at other tapes via
        // some mechanism, but for now it just wants to show the Position Tape

        // TODO: This API kinda sucks, the getposition should send back something over a channel
        // instead of relying on a sort of psuedo-synchronous thing, race conditions everywhere
        // with this.
        tracing::info!(target="hazel::ui::update", "Updating Position");
        self.engine.send(Box::new(GetPosition)).await;
        match self.engine.read().await {
            Some(HazelResponse::Position(Some(pos))) => {
                tracing::debug!(target="hazel::ui::update", "Position is nonempty");
                tracing::debug!(target="hazel::ui::update", "TRS is {:?}", &self.tapereader_state);
                let mut fam = match &self.tapereader_state {
                    Some(dust) => { pos.resummon(dust) },
                    None => { pos.conjure() }
                };
                tracing::debug!(target="hazel::ui::update", "desired pos: {:#05X}", self.tapereader.desired_position);
                fam.seek(self.tapereader.desired_position);
                self.tapereader_state = Some(familiar::dismiss(fam));
                tracing::debug!(target="hazel::ui::update", "TRS is {:?}", &self.tapereader_state);

            },
            Some(HazelResponse::Position(None)) => {
                tracing::debug!(target="hazel::ui::update", "Position is None");
                // There is no game to watch.
                self.tapereader_state = None;
            },
            _ => { }
        }
    }

    pub fn render(&'a self, frame: &mut Frame<'_>) {

        let chunks = LAYOUT.split(frame.area());

        // this will get handled by the engine at some point, so we won't create it here. We'll
        // just ask for the current TapeReaderState for whatever thing we want directly from the
        // WitchHazel engine instance

        // FIXME: this should seek to a position chosen in the UI via up/down arror
        // if let Some(fam) = &self.position_familiar {
        //     fam.seek(self.tapereader.desired_position);
        //     StatefulWidget::render(&self.tapereader, chunks[0], frame.buffer_mut(), fam.get_mut());
        // }


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

        let mut s = match &self.tapereader_state {
            Some(dust) => dust.state.clone(),
            None => TapeReaderState::default()
        };
        StatefulWidget::render(&self.tapereader, chunks[0], frame.buffer_mut(), &mut s);
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
        let hazel = UI::with_handle(&handle).await;

        let mut t = Terminal::new(TestBackend::new(64, 32)).unwrap();
        let _ = t.draw(|f| {
            hazel.render(f);
        });

        assert_debug_snapshot!(t.backend().buffer().clone());
    }
}
