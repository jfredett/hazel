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


#[tokio::main]
async fn main() {
    info!("Welcome to Hazel.");

    // TODO: actually parse arguments
    let headless : bool = false;

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
