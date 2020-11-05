#[macro_use] extern crate clap;

use clap::App;
#[allow(unused_imports)]
use log::{error, debug, info};

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                    "[{}] [{}] {}",
                    record.target(),
                    record.level(),
                    message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("verbose") { setup_logger(log::LevelFilter::Debug) }
    else if matches.is_present("quiet") { setup_logger(log::LevelFilter::Error) } 
    else { setup_logger(log::LevelFilter::Info) }.expect("Failed to set up logger");

    info!("Welcome to Hazel.");

    loop {
        // intentionally blank.
    }
}
