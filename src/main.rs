#[macro_use] extern crate clap;

use tracing::*;

use clap::App;

fn main() {
    let yaml = load_yaml!("args.yml");
    let _matches = App::from_yaml(yaml).get_matches();
    
    
    

    info!("Welcome to Hazel.");

    loop {
        // intentionally blank.
    }
}
