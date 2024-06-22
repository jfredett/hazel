#[macro_use]
extern crate clap;

use clap::App;
use tracing::*;

fn main() {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();

    info!("Welcome to Hazel.");
    // NOTE: Temporarily allowed this lint because eventually this will have other things in it.
    #[allow(clippy::match_single_binding)]
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
