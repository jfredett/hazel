use std::io;

use crate::ply::Ply;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    prelude::{Layout, Constraint, Direction},
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
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(70),
                    Constraint::Percentage(15),
                ]
                .as_ref(),
            )
            .split(area);

        // A static board to render in a box.
        let board = Ply::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let plywidget = PlyWidget { ply: board };

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
            .render(layout[0], buf);

        plywidget.render(layout[1], buf);
    }
}

struct PlyWidget { ply: Ply }

impl Widget for &PlyWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Ply");
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);

        Paragraph::new(Text::from(self.ply.to_string()))
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
