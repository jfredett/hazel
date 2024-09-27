use std::io;
use std::error::Error;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod model;
mod app;
mod widgets;

use app::Hazel;

/// Boilerplate to get the app started.
pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    // Reroute to stderr since we want to talk on stdout for UCI potentially
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Initialize the application
    let mut app = Hazel::new();
    let _res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut Hazel) -> io::Result<bool> {
    loop {
        if app.check_flag("exit") { return Ok(true); }

        terminal.draw(|f| ui(f, app))?;

        let event = event::read()?;
        app.handle_events(event);
    }
}

fn ui(frame: &mut ratatui::Frame, app: &mut Hazel) {
    app.render(frame);
}
