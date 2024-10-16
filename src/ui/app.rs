use std::collections::HashMap;
use std::fmt::Debug;

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

use tracing::{debug, instrument};

use crate::engine::uci::UCIMessage;
use crate::ui::model::entry::{Entry, stockfish};
use crate::engine::Engine;
use crate::board::simple::PieceBoard;
use crate::notation::fen::FEN;

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

        let startup_commands = vec![
            UCIMessage::UCI,
            UCIMessage::IsReady,
            UCIMessage::Position("startpos".to_string(), vec!["d2d4".to_string()]),
        ];


        for command in startup_commands {
            debug!("{}", &command);
            s.entry.exec(&command);
        }

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

    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn renders_as_expected() {
        let mut hazel = Hazel::new();

        let mut t = Terminal::new(TestBackend::new(64, 32)).unwrap();
        let _ = t.draw(|f| {
            hazel.render(f);
        });

        let expected = Buffer::with_lines(vec![
            "               Placeholder               R  N  B  Q  K  B  N  R ",
            "               Placeholder              a8 b8 c8 d8 e8 f8 g8 h8 ",
            "               Placeholder               P  P  P  P  P  P  P  P ",
            "┌──────────────────┐┌──────────────────┐a7 b7 c7 d7 e7 f7 g7 h7 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a6 b6 c6 d6 e6 f6 g6 h6 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a5 b5 c5 d5 e5 f5 g5 h5 ",
            "│    Placeholder   ││    Placeholder   │          P             ",
            "│    Placeholder   ││    Placeholder   │a4 b4 c4 d4 e4 f4 g4 h4 ",
            "│    Placeholder   ││    Placeholder   │                        ",
            "│    Placeholder   ││    Placeholder   │a3 b3 c3 d3 e3 f3 g3 h3 ",
            "│    Placeholder   ││    Placeholder   │ P  P  P     P  P  P  P ",
            "│    Placeholder   ││    Placeholder   │a2 b2 c2 d2 e2 f2 g2 h2 ",
            "│    Placeholder   ││    Placeholder   │ R  N  B  Q  K  B  N  R ",
            "│    Placeholder   ││    Placeholder   │a1 b1 c1 d1 e1 f1 g1 h1 ",
            "│    Placeholder   ││    Placeholder   │       Placeholder      ",
            "│    Placeholder   ││    Placeholder   │       Placeholder      ",
            "└──────────────────┘└──────────────────┘       Placeholder      ",
            "│  rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1  │",
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
            "└──────────────────────────────────────────────────────────────┘",
            "$>                                                              ",
        ]);

        let actual = t.backend().buffer().clone();

        // NOTE: This is going to be turned off most of the time, except when I need a snapshot of the UI
        // to cheat the test.
        // assert_eq!(actual, expected);

        // Ignore style differences for now, by... turning everything into a big list of chars
        // wrapped in &strs wrapped in my pain and suffering.
        let expected_content : Vec<String> = expected.content().iter().map(|x| x.symbol().to_string()).collect();
        let actual_content : Vec<String> = actual.content().iter().map(|x| x.symbol().to_string()).collect();

        assert_eq!(actual_content.join(""), expected_content.join(""));
    }
}
