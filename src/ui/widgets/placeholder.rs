use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::widgets::Wrap;

use ratatui::buffer::Buffer;


#[derive(Debug, Default, Clone)]
pub struct Placeholder {
    width: u16,
    height: u16,
    borders: Borders,
    text: &'static str,
    style: Style
}

// Features:
// 1. Renders to a specific size
// 2. mostly doesn't not work
// Planned Features:
// 1. tickers the text if size is too small. Maybe even DVD-logos in boxes?



/// A Placeholder widget that renders to the specific size given, with a border.
/// It contains the text "Placeholder" in the center of the widget.
impl Placeholder {
    pub fn of_size( width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            borders: Borders::ALL,
            text: "Placeholder",
            style: Style::default()
        }
    }

    pub fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    pub fn text(mut self, text: &'static str) -> Self {
        self.text = text;
        self
    }

    fn calculate_text(&self) -> Paragraph {
        let text = [self.text].repeat(self.height as usize).join("\n");
        Paragraph::new(text)
            .style(self.style)
            .wrap(Wrap { trim: false })
    }

    pub fn set_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl StatefulWidget for &Placeholder {
    type State = ();

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Widget::render(self, area, buf);
    }
}

impl Widget for &Placeholder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let widget = self.calculate_text().block(
            Block::default()
                .borders(self.borders)
        ).alignment(Alignment::Center);
        let new_rect = Rect::new(area.x, area.y, self.width, self.height);

        Widget::render(&widget, new_rect, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn calculates_text_correctly() {
        let placeholder = Placeholder::of_size(13, 3);
        let text = placeholder.calculate_text();

        let expected = Paragraph::new("Placeholder\nPlaceholder\nPlaceholder").wrap(Wrap { trim: false });
        assert_eq!(text, expected);
    }

    mod rendering {
        use super::*;

        #[test]
        fn smallest_box_without_overflow() {
            let rect = Rect::new(0, 0, 13, 3);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            Widget::render(&Placeholder::of_size(13, 3), buffer.area, &mut buffer);

            let mut expected = Buffer::with_lines(vec![
                "┌───────────┐",
                "│Placeholder│",
                "└───────────┘"
            ]);
            expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            assert_eq!(buffer, expected);
        }

        #[test]
        fn mutliline_extrawide_box_odd_rows_even_columns() {
            let rect = Rect::new(0, 0, 20, 5);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            Widget::render(&Placeholder::of_size(20, 5), buffer.area, &mut buffer);

            let mut expected = Buffer::with_lines(vec![
                "┌──────────────────┐",
                "│    Placeholder   │",
                "│    Placeholder   │",
                "│    Placeholder   │",
                "└──────────────────┘"
            ]);
            expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            assert_eq!(buffer, expected);
        }

        #[test]
        fn mutliline_extrawide_box_even_rows_odd_columns() {
            let rect = Rect::new(0, 0, 19, 6);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            let placeholder = Placeholder::of_size(19, 6);
            Widget::render(&placeholder, buffer.area, &mut buffer);

            let mut expected = Buffer::with_lines(vec![
                "┌─────────────────┐",
                "│   Placeholder   │",
                "│   Placeholder   │",
                "│   Placeholder   │",
                "│   Placeholder   │",
                "└─────────────────┘"
            ]);
            expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            assert_eq!(buffer, expected);
        }

        #[test]
        fn mutliline_extrawide_box_even_rows_even_columns() {
            let rect = Rect::new(0, 0, 20, 6);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            let placeholder = Placeholder::of_size(20, 6);
            Widget::render(&placeholder, buffer.area, &mut buffer);

            let mut expected = Buffer::with_lines(vec![
                "┌──────────────────┐",
                "│    Placeholder   │",
                "│    Placeholder   │",
                "│    Placeholder   │",
                "│    Placeholder   │",
                "└──────────────────┘"
            ]);
            expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            assert_eq!(buffer, expected);
        }

        #[test]
        fn mutliline_extrawide_box_odd_rows_odd_columns() {
            let rect = Rect::new(0, 0, 19, 5);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            let placeholder = Placeholder::of_size(19, 5);
            Widget::render(&placeholder, buffer.area, &mut buffer);

            let mut expected = Buffer::with_lines(vec![
                "┌─────────────────┐",
                "│   Placeholder   │",
                "│   Placeholder   │",
                "│   Placeholder   │",
                "└─────────────────┘"
            ]);
            expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            assert_eq!(buffer, expected);
        }

        #[test]
        fn box_with_overflow() {
            let rect = Rect::new(0, 0, 10, 3);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            let placeholder = Placeholder::of_size(10, 3);
            Widget::render(&placeholder, buffer.area, &mut buffer);

            let mut expected = Buffer::with_lines(vec![
                "┌────────┐",
                "│Placehol│",
                "└────────┘"
            ]);
            expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            assert_eq!(buffer, expected);
        }

        #[test]
        fn box_with_overflow_even_sides() {
            let rect = Rect::new(0, 0, 10, 4);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            let placeholder = Placeholder::of_size(10, 4);
            Widget::render(&placeholder, buffer.area, &mut buffer);

            let mut expected = Buffer::with_lines(vec![
                "┌────────┐",
                "│Placehol│",
                "│   der  │",
                "└────────┘"
            ]);
            expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            assert_eq!(buffer, expected);
        }

        #[test]
        fn box_with_overflow_odd_sides() {
            let rect = Rect::new(0, 0, 11, 3);
            let mut buffer = Buffer::empty(rect);
            buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            let placeholder = Placeholder::of_size(11, 3);
            Widget::render(&placeholder, buffer.area, &mut buffer);

            let mut expected = Buffer::with_lines(vec![
                "┌─────────┐",
                "│Placehold│",
                "└─────────┘"
            ]);
            expected.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

            assert_eq!(buffer, expected);
        }
    }
}

