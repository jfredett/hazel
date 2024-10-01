pub mod outputwidget;

use ratatui::layout::Direction;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use crate::ui::app::Hazel;

// TODO: minimize the state by making EngineIOSection not need all of Hazel, instead it'll
// be it's own state management ViewModel and Hazel will construct it from itself.

pub struct EngineIOSection {
    output: Vec<String>,
    input: String,
}


struct Input {
}

struct Output {
}

impl Default for Input {
    fn default() -> Self {
        Self {}
    }
}

impl Default for Output {
    fn default() -> Self {
        Self {}
    }
}

impl StatefulWidget for &Input {
    type State = String;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::NONE)
            .title(format!("$> {}", state))
            .border_style(Style::default().fg(Color::White).bg(Color::Black));
        block.render(area, buf);
    }
}

impl StatefulWidget for &Output {
    type State = Vec<String>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White).bg(Color::Black));
        block.render(area, buf);

        // long lines are wrapped and indented by 2 spaces
        let output = state.clone();
        let mut adjusted = vec![];
        for line in output.iter() {
            if line.len() > area.width as usize - 2 {
                let mut working_copy = line.clone();
                let mut new_lines = vec![working_copy.split_off(area.width as usize - 2)];

                while working_copy.len() > area.width as usize - 2 {
                    let new_line = format!("  {}", working_copy.split_off(area.width as usize - 4));
                    new_lines.push(new_line);
                }
                new_lines.push(working_copy);
                adjusted.extend(new_lines.into_iter().rev());
            } else {
                adjusted.push(line.to_string());
            }
        }

        // should fill from the bottom to the top:
        let mut y = area.bottom() - 2;
        for line in adjusted.into_iter().rev() {
            buf.set_string(area.left() + 1, y, line, Style::default().fg(Color::White).bg(Color::Black));
            y -= 1;
        }
    }
}


lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Fill(1),
                Constraint::Max(1),
            ]
                .as_ref(),
        );
}

impl EngineIOSection {
    pub fn new() -> Self {
        Self {
            output: vec![],
            input: String::new(),
        }
    }

    pub fn push(&mut self, line: String) {
        self.output.push(line);
    }
}

impl StatefulWidget for &EngineIOSection {
    type State = Hazel;
    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let chunks = LAYOUT.split(area);


        let output_widget = Output::default();
        output_widget.render(chunks[0], buf, &mut self.output.clone());

        let input_widget = Input::default();
        input_widget.render(chunks[1], buf, &mut self.input.clone());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        let rect = Rect::new(0, 0, 64, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let engine_io_section = &mut EngineIOSection::new();

        // Mock out the output from the 'engine'
        engine_io_section.push("Stockfish 16.1 by the Stockfish developers (see AUTHORS file)".to_string());
        engine_io_section.push("> isready".to_string());
        engine_io_section.push("readyok".to_string());
        engine_io_section.push("> position startpos moves".to_string());
        engine_io_section.push("> go depth 1".to_string());
        engine_io_section.push("info string NNUE evaluation using nn-baff1ede1f90.nnue".to_string());
        engine_io_section.push("info string NNUE evaluation using nn-b1a57edbea57.nnue".to_string());
        engine_io_section.push("info depth 1 seldepth 2 multipv 1 score cp 0 nodes 20 nps 1000 hashfull 0 tbhits 0 time 20 pv d2d4".to_string());
        engine_io_section.push("bestmove d2d4".to_string());

        engine_io_section.render(rect, &mut buffer, &mut Hazel::new());

        let mut expected = Buffer::with_lines(vec![
            "┌──────────────────────────────────────────────────────────────┐",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│Stockfish 16.1 by the Stockfish developers (see AUTHORS file) │",
            "│> isready                                                     │", // prefixes inputs by >
            "│readyok                                                       │",
            "│> position startpos moves                                     │",
            "│> go depth 1                                                  │",
            "│info string NNUE evaluation using nn-baff1ede1f90.nnue        │",
            "│info string NNUE evaluation using nn-b1a57edbea57.nnue        │",
            "│info depth 1 seldepth 2 multipv 1 score cp 0 nodes 20 nps 1000│", // wraps long lines?
            "│ hashfull 0 tbhits 0 time 20 pv d2d4                          │",
            "│bestmove d2d4                                                 │",
            "└──────────────────────────────────────────────────────────────┘",
            "$>                                                              ",
        ]);

        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));
        assert_eq!(buffer, expected);
    }
}


