use std::io;

mod viewmodels;
mod state;
mod widgets;
mod tui;

use widgets::hazel::HazelWidget;

pub fn run() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let result = HazelWidget::default().run(&mut terminal); 
    tui::restore()?;
    result
}
