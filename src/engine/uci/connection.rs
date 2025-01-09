

// Open a (provided) communication stream, read it by line, and parse it into UCI messages.

use std::io::{self, BufRead};

use tracing::{error, info};

use crate::engine::uci::UCIMessage;
use crate::engine::driver::WitchHazel;

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

    print!("> ");
    let echo_handle = hazel.clone();
    tokio::spawn(async move {
        info!("Waiting for message");
        while let Some(msg) = echo_handle.read().await {
            match write!(output, "{:?}\n> ", msg) {
                Ok(_) => {},
                Err(e) => {
                    error!("Error writing to output: {:?}", e);
                }
            }
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


#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[tokio::test]
    async fn test_with_dummy_io() {
    // BUG: This is a bad test, doesn't check output
    //
    // Probably I need to write these as 'real' integration tests that spawn a process and do
    // stuff to it.
    let input = "uci\nisready\n".as_bytes();
    let output = Vec::new();
    let result = run_with_io(input, output).await;
    assert!(result.is_ok());
    }
    */
}
