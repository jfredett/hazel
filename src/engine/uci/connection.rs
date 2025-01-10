

// Open a (provided) communication stream, read it by line, and parse it into UCI messages.

use std::io::{self, BufRead};

use tracing::{error, info};

use crate::engine::uci::UCIMessage;
use crate::engine::driver::{HazelResponse, WitchHazel};

#[cfg_attr(test, mutants::skip)]
pub async fn run() -> io::Result<()> {
    run_with_io(io::stdin(), io::stdout()).await
}


// TODO: This should be a config setting from a config file/option/etc.
pub const BUF_SIZE: usize = 1024;

// take arbitrary IO Streams and use it as if it were STDIN/STDOUT
// to do the `run` function above
pub async fn run_with_io<T,U>(input: T, mut output: U) -> io::Result<()> 
where T: 'static + io::Read + Send, U: 'static + io::Write + Send {
    let hazel = WitchHazel::<1024>::new().await;

    let echo_handle = hazel.clone();
    tokio::spawn(async move {
        while let Some(resp) = echo_handle.read().await {
            let msg = match resp {
                HazelResponse::UCIResponse(uci_msg) => { format!("{}\n", uci_msg) },
                _ => { format!("{:?}", resp) }
            };
            match output.write_all(msg.as_bytes()) {
                Ok(_) => {},
                Err(e) => { error!("Error writing to output: {}", e); }
            }
            output.flush().unwrap();
        }
    });

    let input = io::BufReader::new(input);

    info!("Starting input task");
    let handle = io::BufReader::new(input);
    let mut lines = handle.lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        let message = UCIMessage::parse(&line);
        hazel.send(Box::new(message)).await;
    }
}
