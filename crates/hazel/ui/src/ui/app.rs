use hazel_representation::game::chess::state::tape_reader_state::TapeReaderState;
use hazel_representation::board::PieceBoard;
use hazel_core::constants::START_POSITION_FEN;
use hazel_core::ben::BEN;
use hazel_engine::driver::hazel::{GetPosition, HazelResponse, WitchHazel};
use hazel_engine::uci::UCIMessage;

use ratatui::{crossterm::event::{Event, KeyCode}, layout::{Constraint, Layout}, style::{Color, Style}, widgets::{StatefulWidget, Widget}, Frame};

use spell::{cursorlike::Cursorlike, familiar::{self, Quintessence}};

use std::{collections::HashMap, fmt::Debug, sync::Mutex};

use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};

use crate::ui::widgets::tapereader::TapeReaderWidget;

use super::widgets::{board::Board, fen::FEN, input::Input, output::Output};

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
    current_ben: Option<Quintessence<BEN>>,
    current_inputline: Mutex<String>
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
        engine.send(Box::new(
            UCIMessage::Position(START_POSITION_FEN.to_owned(),
            vec![
                "d2d4".to_string(),
                "d7d5".to_string(),
                "c1f4".to_string(),
                "g8f6".to_string(),
                "e2e3".to_string(),
                "b8c6".to_string(),
                "g1f3".to_string(),
                "e7e6".to_string(),
                "f1d3".to_string(),
                "e1g1".to_string(),
            ]
        ))).await;

        Self {
            flags: HashMap::new(),
            engine,
            mode: Mode::Command,
            tapereader: TapeReaderWidget::default(),
            tuiloggerstate: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
            tapereader_state: None,
            current_ben: None,
            current_inputline: Mutex::new("".to_string())
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

    pub fn output_widget(&self) -> Output {
        Output::default()
    }

    pub fn input_widget(&self) -> Input {
        Input::default()
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
                let mut fam = match &self.tapereader_state {
                    Some(dust) => { pos.resummon(dust) },
                    None => { pos.conjure() }
                };
                tracing::debug!(target="hazel::ui::update", "desired pos: {:#05X}", self.tapereader.desired_position);
                // BUG: pending `spellstate` trait move fam.seek(self.tapereader.desired_position);
                self.tapereader_state = Some(familiar::dismiss(fam));

                let mut fam = match &self.current_ben {
                    Some(dust) => { pos.resummon(dust) },
                    None => { pos.conjure() }
                };
                fam.seek(self.tapereader.desired_position);
                self.current_ben = Some(familiar::dismiss(fam));

            },
            Some(HazelResponse::Position(None)) => {
                tracing::debug!(target="hazel::ui::update", "Position is None");
                // There is no game to watch.
                self.tapereader_state = None;
                self.current_ben = None;
            },
            _ => { }
        }
    }

    fn tui_logger_widget(&self) -> TuiLoggerSmartWidget {
        TuiLoggerSmartWidget::default()
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
            .state(&self.tuiloggerstate)
    }

    pub fn render(&'a self, frame: &mut Frame<'_>) {
        let chunks = PRIMARY_LAYOUT.split(frame.area());
        let upper_section = chunks[0];
        let log_section = chunks[1];
        let io_section = chunks[2];

        let chunks = UPPER_LAYOUT.split(upper_section);
        let board_section = chunks[0];
        let tapereader_section = chunks[1];

        let chunks = BOARD_SECTION_LAYOUT.split(board_section);
        let _board_header = chunks[0];
        let board_field = chunks[1];
        let board_footer = chunks[2];

        let chunks = IO_SECTION_LAYOUT.split(io_section);
        let output_section = chunks[0];
        let input_section = chunks[1];

        let tlw = self.tui_logger_widget();
        let mut state = match &self.tapereader_state {
            Some(dust) => dust.state.clone(),
            None => TapeReaderState::default()
        };

        let ben = match &self.current_ben {
            Some(dust) => dust.state,
            None => BEN::empty()
        };

        let mut pieceboard = PieceBoard::default();
        pieceboard.set_position(ben);

        let board = Board::from(pieceboard);
        let fen = FEN::new(ben);

        StatefulWidget::render(&self.tapereader, tapereader_section, frame.buffer_mut(), &mut state);
        Widget::render(&board, board_field, frame.buffer_mut());
        Widget::render(&fen, board_footer, frame.buffer_mut());
        Widget::render(tlw, log_section, frame.buffer_mut());

        // TODO: Maintain backlog
        StatefulWidget::render(&self.output_widget(), output_section, frame.buffer_mut(), &mut vec![]);
        {
            // this feels wrong
            let mut current_line = self.current_inputline.lock().unwrap();
            StatefulWidget::render(&self.input_widget(), input_section, frame.buffer_mut(), &mut current_line);
        }
    }
}

lazy_static! {
    static ref PRIMARY_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Min(1),
            ].as_ref());

    static ref UPPER_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ].as_ref());

    static ref BOARD_SECTION_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ].as_ref());

    static ref IO_SECTION_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1)
        ].as_ref());
}


#[cfg(test)]
mod tests {
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
