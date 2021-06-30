#[macro_use] extern crate clap;

use tracing::{*};
use tracing::Level;
use tracing_subscriber;

use clap::App;

fn main() {
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
            if sub_args.is_present("resume") {
                let location = sub_args.value_of("resume").unwrap();
                info!("Resuming previous Arena at {}", location);
                // create a "Wizard::Arena" object and load the relevant files from the given directory
                
            } else {
                // if it lacks that option, take the path and initialize a new arena
                let location = sub_args.value_of("start").unwrap();
                // if find-magics has the --resume option, take the path and load it
                info!("Starting new Arena at {}", location);
                // create a "Wizard::Arena" object and initialize it with some number of wizards.
            }
        }
        (_, _) => {
            info!("Startup complete, awaiting instructions");
            loop {
                // intentionally blank.
            }
        }

    }
}
