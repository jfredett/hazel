use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

#[derive(Default)]
pub struct Input {
    content: String
}

impl Input {
    pub fn push(&mut self, input: char) {
        self.content.push(input);
    }

    pub fn pop(&mut self) {
        self.content.pop();
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn flush(&mut self) -> String {
        let content = self.content.clone();
        self.content.clear();
        content
    }
}

impl StatefulWidget for &Input {
    type State = String;

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let block = Block::default()
            .borders(Borders::NONE)
            .title(format!("$> {}", self.content))
            .border_style(Style::default().fg(Color::White).bg(Color::Black));
        block.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 3);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let input = &mut Input::default();
        input.push('a');
        input.push('b');
        input.push('c');
        input.render(rect, &mut buffer, &mut input.content());

        let mut expected = Buffer::with_lines(vec![
            "$> abc                                                          ",
            "                                                                ",
            "                                                                ",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }

    #[test]
    fn flush_emptys_input_buffer() {
        let mut input = Input::default();
        input.push('a');
        input.push('b');
        input.push('c');
        assert_eq!(input.flush(), "abc");
        assert_eq!(input.content(), "");
    }
}
