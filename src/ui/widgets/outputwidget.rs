use ratatui::prelude::*;
use ratatui::widgets::Widget;

use ratatui::buffer::Buffer;


#[derive(Debug, Default, Clone)]
pub struct OutputWidget(Vec<String>);

impl OutputWidget {
    pub fn empty() -> Self {
        Self {
            0: vec![]
        }
    }

    pub fn push(&mut self, message: String) {
        self.0.push(message);
    }
}

impl Widget for &OutputWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let frame = Text::styled(
            self.0.join("\n"),
            Style::default().fg(Color::White).bg(Color::Black)
        );
        frame.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders() {
        let rect = Rect::new(0, 0, 13, 1);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let output = &mut OutputWidget::empty();
        output.push("Hello, world!".to_string());
        output.render(buffer.area, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "Hello, world!"
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);
    }

    #[test]
    fn renders_multiline() {
        let rect = Rect::new(0, 0, 13, 3);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let output = &mut OutputWidget::empty();
        output.push("Hello, world!".to_string());
        output.push("Hello, world!".to_string());
        output.render(buffer.area, &mut buffer);

        let mut expected = Buffer::with_lines(vec![
            "Hello, world!",
            "Hello, world!",
            ""
        ]);
        expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        assert_eq!(buffer, expected);

    }
}
