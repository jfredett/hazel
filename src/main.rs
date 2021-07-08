#[macro_use] extern crate clap;

use std::path::PathBuf;

use anyhow::Error;
use clap::App;
use tracing::{*};
use tracing::Level;

use hazel::wizard;

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    let log_level = if matches.is_present("verbose") { Level::DEBUG }
    else if matches.is_present("quiet") { Level::ERROR } 
    else { Level::INFO };
    
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    info!("Welcome to Hazel.");
    match matches.subcommand() {
        ("magics", Some(sub_args)) => { 
            info!("Starting Wizard Arena");
            let mut arena = if sub_args.is_present("resume") {
                let location = sub_args.value_of("resume").unwrap();
                info!("Resuming previous Arena at {}", location);
                // create a "Wizard::Arena" object and load the relevant files from the given directory
                let arena = wizard::arena::Arena::load(PathBuf::from(location))?;
                debug!("Arena loaded with size: {:?}", arena.size());
                arena
            } else {
                // if it lacks that option, take the path and initialize a new arena
                let location = sub_args.value_of("start").unwrap();
                let size = sub_args.value_of("size").unwrap().parse().unwrap();
                // if find-magics has the --resume option, take the path and load it
                info!("Starting new Arena at {}", location);
                // create a "Wizard::Arena" object and initialize it with some number of wizards.
                let arena = wizard::arena::Arena::new(size,PathBuf::from(location))?;
                debug!("Arena loaded with size: {:?}", arena.size());
                arena
            };
            
            loop {
                arena.step()?;
            }
        }
        (_, _) => {
            info!("Startup complete, awaiting instructions");
            #[allow(clippy::empty_loop)]
            loop {
                // intentionally blank.
            }
        }

    }
}
