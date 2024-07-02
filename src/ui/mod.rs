use std::io;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

mod tui;

#[derive(Debug, Default)]
struct Hazel {
    // TODO: Control Panel State goes here
    exit : bool

}

impl Hazel {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(&*self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                modifiers: _, ..
            }) => Ok(self.exit = true),
            _ => Ok(())
        }
    }
}

impl Widget for &Hazel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Hazel - A Chess Engine");
        let instructions = Title::from(Line::from(vec![
                "Press q to quit".into()
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom))
            .border_set(border::THICK);

        Paragraph::new(Text::from(""))
            .block(block)
            .render(area, buf);
    }

}

pub fn run() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let result = Hazel::default().run(&mut terminal); 
    tui::restore()?;
    result
}
