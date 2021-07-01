#[macro_use] extern crate clap;

use tracing::*;

use clap::App;
use tracing::*;


fn main() {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();

    info!("Welcome to Hazel.");
    match matches.subcommand() {
        (_, _) => {
            info!("Startup complete, awaiting instructions");
            #[allow(clippy::empty_loop)]
            loop {
                // intentionally blank.
            }
        }
    }
}
