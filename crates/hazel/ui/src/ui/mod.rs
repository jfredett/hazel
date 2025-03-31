use std::{env, io};
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

mod app;
mod widgets;

use app::UI;
use tui_logger::{init_logger, set_default_level, set_log_file, LevelFilter, TuiLoggerFile, TuiLoggerLevelOutput};

use hazel_engine::driver::hazel::WitchHazel;

/// Boilerplate to get the app started.
pub async fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    // Reroute to stderr since we want to talk on stdout for UCI potentially
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Initialize the application
    let handle = WitchHazel::new().await;
    let mut app = UI::with_handle(&handle).await;
    _ = run_app(&mut terminal, &mut app).await;

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut UI<'_>) -> io::Result<bool> {
    use tracing_subscriber::prelude::*;

    // Set up the Tracing layer
    tracing_subscriber::registry()
        .with(tui_logger::TuiTracingSubscriberLayer)
        .init();

    // Initialize the tui-logger widget
    let _ = init_logger(LevelFilter::Trace);
    set_default_level(LevelFilter::Trace);

    // prepare the log directory and file.
    let mut dir = env::temp_dir();
    dir.push("hazel.log");
    let file_options = TuiLoggerFile::new(dir.to_str().unwrap())
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_file(false)
        .output_separator(':');

    // Set the log files
    set_log_file(file_options);

    tracing::debug!(target:"hazel_representation::ui", "Logging to {}", dir.to_str().unwrap());
    tracing::debug!(target:"hazel_representation::ui", "Logging initialized");

    // do an initial draw so we don't blank-screen, this maybe should be a splash page?
    tracing::info!(target:"hazel_representation::ui", "initial draw");
    terminal.draw(|f| app.render(f) )?;

    loop {
        if app.check_flag("exit") { return Ok(true); }

        tracing::info!(target:"hazel_representation::ui", "handling events");
        let event = event::read()?;
        app.handle_events(event);

        tracing::info!(target:"hazel_representation::ui", "updating");
        app.update().await;

        tracing::info!(target:"hazel_representation::ui", "drawing");
        terminal.draw(|f| app.render(f) )?;

    }
}
