#![allow(unused_imports)]

use tracing::*;

use hazel::uci;
use hazel::ui;

use std::thread;
use tracing::info;
use crossbeam::channel::Sender;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[derive(Debug, PartialEq)]
#[allow(dead_code)] // this is a stub, not dead code
enum RaceControlMessage {
    Exit
}

fn main() {
    info!("Welcome to Hazel.");

    // parse arguments
    let headless : bool = false;

    /*
     *
     * | External GUI |
     *       |
     *       |
     *  STDIN/STDOUT
     *       |
     *       |
     * | UCI Socket |
     *       |
     *       |
     * | Race Control | --> | UI |
     *       |
     *       \-------> | Grid | 1-*> | Engine | --> STDIN/STDOUT --> External Engine over UCI
     *                     |      *> | Engine | --> STDIN/STDOUT --> Another UCI-speaking client
     *                     1      *> | Engine | --> Hazel::Driver
     *                     |      *> | Engine | --> TCP/UDP/etc --> Race Control on another machine
     *                     |      *> | Engine | --> protobuf API --> other speaker of bespoke protocol
     *                     |
     *                     \------*> | Track |
     *
     * Engines do not know about each other, programs run on the 'Grid', which is managed via the
     * UI through Race control (UI = View, Race Control = Controller, Grid = Model). The Grid
     * provides a scripting API for setting up games between all engines on the grid by providing
     * metacommands around routing UCI connections between engines and mediating games.
     *
     * The UI is a simple terminal interface that allows the user to start games, view the grid,
     * and configure engines. It also allows the user to provide 'Tracks', which are scripts that 
     * initiate and run games between engines. For instance, a 'Swiss' track would run a Swiss
     * Tournament between all engines on the grid, a 'Round Robin' track would run a round-robin,
     * etc. Tracks may also compare non-game metrics, such as 'perft' tracks or other
     * cross-reference style tests.
     *
     *
     *
     *
     */

    // if headless, we'll start a UCI connection to a Hazel Driver
    // if not headless, we'll just start the UI

    let _ = if headless {
        // Log to STDERR
        let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stderr());
        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .init();
        let _ = uci::run();
    } else {
        // Log to a file
        let (non_blocking, _guard) = tracing_appender::non_blocking(std::fs::File::create("hazel.log").unwrap());
        let subscriber = fmt::Subscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .with_writer(non_blocking)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        let _ = ui::run();
    };

    ()
}
