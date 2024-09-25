use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::text::Text;
use ratatui::prelude::*;
use ratatui::widgets::{Widget, Block, Borders};


use tracing::{debug, instrument};

use ratatui::buffer::Buffer;

use crate::uci::UCIMessage;
use crate::ui::model::{
    pieceboard::PieceBoard,
    entry::{Entry, stockfish}
};
use crate::engine::Engine;

#[derive(Debug, Default)]
pub struct FENWidget {
    board: PieceBoard
}

impl FENWidget {
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


pub struct Hazel {
    flags: HashMap<String, bool>,
    entry: Entry
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
            entry: stockfish()
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
    pub fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(10),
                    Constraint::Percentage(70),
                ].as_ref()
            )
            .split(frame.size());

        let block = Block::default()
            .title("Hazel")
            .borders(Borders::ALL);

        frame.render_widget(block, frame.size());
        frame.render_widget(&self.fen_widget(), chunks[0]);

        // render an input/output widgets, the input sends to Entry's stdin, the output is Entry's
        // stdout.
        let input_widget = Block::default()
            .title("Input")
            .borders(Borders::ALL);
        let output_widget = Block::default()
            .title("Output")
            .borders(Borders::ALL);

        frame.render_widget(input_widget, chunks[1]);
        frame.render_widget(output_widget, chunks[2]);

        frame.render_widget(&self.board_widget(), chunks[3]);

    }
}

pub struct BoardWidget {
    board: PieceBoard
}

impl BoardWidget {
    pub fn from(board: &PieceBoard) -> Self {
        Self {
            board: board.clone()
        }
    }
}


fn eight_cells(direction: Direction) -> Layout {
    Layout::default()
        .direction(direction)
        .constraints(
            [
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
                /* Would be cool to scale up the board dynamically with the allotted size.
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                Constraint::Ratio(1, 8),
                */
            ].as_ref()
        )
}

impl Widget for &BoardWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cols = eight_cells(Direction::Horizontal).split(area);
        let rows = [
            eight_cells(Direction::Vertical).split(cols[0]),
            eight_cells(Direction::Vertical).split(cols[1]),
            eight_cells(Direction::Vertical).split(cols[2]),
            eight_cells(Direction::Vertical).split(cols[3]),
            eight_cells(Direction::Vertical).split(cols[4]),
            eight_cells(Direction::Vertical).split(cols[5]),
            eight_cells(Direction::Vertical).split(cols[6]),
            eight_cells(Direction::Vertical).split(cols[7])
        ];

        for i in 0..8 {
            for j in 0..8 {
                let cell = Block::default()
                    .title(self.board.get(i,j).to_string())
                    .borders(Borders::NONE);
                cell.render(rows[j][i], buf);
            }
        }
    }
}
