
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

pub struct Output {
    buffer: Vec<String>
}

impl Default for Output {
    fn default() -> Self {
        Self {
            buffer: vec![]
        }
    }
}

impl Output {
    pub fn push(&mut self, line: String) {
        self.buffer.push(line);
    }

    pub fn buffer(&self) -> Vec<String> {
        self.buffer.clone()
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_as_expected() {
        let rect = Rect::new(0, 0, 64, 3);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let output = &mut Output::default();
        output.push("This is a test".to_string());
        output.render(rect, &mut buffer, &mut output.buffer().clone());

        let mut expected = Buffer::with_lines(vec![
            "┌──────────────────────────────────────────────────────────────┐",
            "│This is a test                                                │",
            "└──────────────────────────────────────────────────────────────┘",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }

    #[test]
    fn renders_a_very_long_line_by_wrapping() {
        let rect = Rect::new(0, 0, 15, 6);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let output = &mut Output::default();
        output.push("This is a very long line that should be wrapped".to_string());
        output.render(rect, &mut buffer, &mut output.buffer().clone());

        let mut expected = Buffer::with_lines(vec![
        "┌─────────────┐",
        "│This is a ver│",
        "│y long line t│",
        "│hat should be│",
        "│ wrapped     │",
        "└─────────────┘",
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }
}







