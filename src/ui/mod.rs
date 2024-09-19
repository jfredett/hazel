use std::io;

mod widgets;
mod tui;

use widgets::hazel::Hazel;

pub fn run() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let result = Hazel::default().run(&mut terminal); 
    tui::restore()?;
    result
}
