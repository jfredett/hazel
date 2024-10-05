use ratatui::layout::Direction;
use ratatui::prelude::*;

use crate::ui::widgets::input::Input;
use crate::ui::widgets::output::Output;

lazy_static! {
    static ref LAYOUT : Layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Fill(1),
                Constraint::Max(1),
            ].as_ref(),
        );
}

pub struct EngineIOSection {
    output: Output,
    input: Input,
}

impl EngineIOSection {
    pub fn push(&mut self, line: String) {
        self.output.push(line);
    }

    pub fn handle_input(&mut self, input: char) {
        self.input.push(input);
    }

    pub fn handle_backspace(&mut self) {
        self.input.pop();
    }

    pub fn handle_enter(&mut self) {
        let content = self.input.flush();

        self.output.push(format!("> {}",  content));
        // TODO: Send to engine as well
    }
}

impl Default for EngineIOSection {
    fn default() -> Self {
        Self {
            output: Output::default(),
            input: Input::default(),
        }
    }
}

impl Widget for &EngineIOSection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = LAYOUT.split(area);

        // NOTE: I don't fully understand why the buffer is passed like this, since it's already
        // part of the widget? Seems redundant, but maybe something to do with mutability/borrow
        // checker stuff? Really passes my understanding right now.
        self.output.render(chunks[0], buf, &mut self.output.buffer());
        self.input.render(chunks[1], buf, &mut self.input.content());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let mut engine_io_section = EngineIOSection::default();

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

        engine_io_section.render(rect, &mut buffer);

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

    #[test]
    fn renders_very_long_lines_correctly() {
        let rect = Rect::new(0, 0, 27, 6);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let mut engine_io_section = EngineIOSection::default();


        engine_io_section.push("A line exceeding, the buffers shortened width, should be wrapped around.".to_string());

        engine_io_section.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "┌─────────────────────────┐",
            "│A line exceeding, the buf│",
            "│fers shortened width, sho│",
            "│uld be wrapped around.   │",
            "└─────────────────────────┘",
            "$>                         ",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }

    #[test]
    fn accepts_input() {
        let rect = Rect::new(0, 0, 64, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let mut engine_io_section = EngineIOSection::default();

        engine_io_section.handle_input('a');
        engine_io_section.handle_input('b');
        engine_io_section.handle_input('c');
        engine_io_section.handle_input('d');
        engine_io_section.handle_input('e');
        engine_io_section.handle_input('f');
        engine_io_section.handle_input('g');

        engine_io_section.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
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
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "└──────────────────────────────────────────────────────────────┘",
            "$> abcdefg                                                      ",
        ]);


        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);

    }

    #[test]
    fn handles_backspace() {
        let rect = Rect::new(0, 0, 64, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let engine_io_section = &mut EngineIOSection::default();

        engine_io_section.handle_input('a');
        engine_io_section.handle_input('b');
        engine_io_section.handle_input('c');
        engine_io_section.handle_input('d');
        engine_io_section.handle_input('e');
        engine_io_section.handle_input('f');
        engine_io_section.handle_input('g');

        engine_io_section.handle_backspace();

        engine_io_section.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
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
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "└──────────────────────────────────────────────────────────────┘",
            "$> abcdef                                                       ",
        ]);

        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);

    }


    #[test]
    fn handles_enter() {
        let rect = Rect::new(0, 0, 64, 17);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let mut engine_io_section = EngineIOSection::default();

        engine_io_section.handle_input('a');
        engine_io_section.handle_input('b');
        engine_io_section.handle_input('c');
        engine_io_section.handle_input('d');
        engine_io_section.handle_input('e');
        engine_io_section.handle_input('f');
        engine_io_section.handle_input('g');

        engine_io_section.handle_enter();

        engine_io_section.render(rect, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
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
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│                                                              │",
            "│> abcdefg                                                     │",
            "└──────────────────────────────────────────────────────────────┘",
            "$>                                                              ",
        ]);

        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);

    }
}


