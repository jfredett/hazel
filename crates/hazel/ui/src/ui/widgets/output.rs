
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

#[derive(Default)]
pub struct Output {
    buffer: Vec<String>
}

impl Output {
    pub fn push(&mut self, line: String) {
        self.buffer.push(line);
    }

    pub fn buffer(&self) -> Vec<String> {
        self.buffer.clone()
    }
}

fn wrap_string(s: &str, width: usize) -> Vec<String> {
    let mut lines = vec![];
    let mut working_copy = s.to_string();
    while working_copy.len() > width {
        let new_line = working_copy.split_off(width);
        lines.push(working_copy);
        working_copy = new_line;
    }
    lines.push(working_copy);
    lines
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
                let new_lines = wrap_string(line, area.width as usize - 2);
                adjusted.extend(new_lines.into_iter());
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







