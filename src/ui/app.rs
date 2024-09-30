use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use tui_input::Input;

use crate::ui::widgets::game_section::board_section::fenwidget::FENWidget;
use crate::ui::widgets::game_section::board_section::boardwidget::BoardWidget;

use tracing::{debug, instrument};

use crate::uci::UCIMessage;
use crate::ui::model::entry::{Entry, stockfish};
use crate::engine::Engine;

use super::widgets::engine_io_section::outputwidget::OutputWidget;

pub struct Hazel {
    flags: HashMap<String, bool>,
    entry: Entry,

    input: Input
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
            input: Input::default(),
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
        let chunks = layout().split(frame.size());

        let container = Block::default()
            .title(Span::styled("Hazel", Style::default().fg(Color::White).bg(Color::Black)))
            .borders(Borders::ALL);
        frame.render_widget(container, chunks[0]);

        // render an input/output widgets, the input sends to Entry's stdin, the output is Entry's
        // stdout.

        let mut output_widget = OutputWidget::empty();

        output_widget.push("Hello, world!".to_string());
        output_widget.push("Hello, world!".to_string());
        output_widget.push("Hello, world!".to_string());
        output_widget.push("Hello, world!".to_string());
        output_widget.push("Hello, world!".to_string());
        output_widget.push("Hello, world!".to_string());

        frame.render_widget(&output_widget, chunks[3]);

        let input_widget = Block::default()
            .title("Input")
            .borders(Borders::ALL);
        frame.render_widget(input_widget, chunks[2]);

        frame.render_widget(&self.board_widget(), chunks[1]);
    }
}


fn layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Min(8),
                Constraint::Length(10),
                Constraint::Max(1),
            ].as_ref()
        )
}
