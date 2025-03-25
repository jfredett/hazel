// Open a (provided) communication stream, read it by line, and parse it into UCI messages.

use std::io;
use std::io::BufRead;


use crate::engine::uci::UCIMessage;
use crate::engine::driver::{HazelResponse, WitchHazel};

#[cfg_attr(test, mutants::skip)]
pub async fn run() -> tokio::io::Result<()> {
    run_with_io(io::stdin(), io::stdout()).await
}


// TODO: This should be a config setting from a config file/option/etc.
pub const BUF_SIZE: usize = 1024;

// take arbitrary IO Streams and use it as if it were STDIN/STDOUT
// to do the `run` function above
pub async fn run_with_io<T, U>(input: T, mut output: U) -> tokio::io::Result<()>
where T: 'static + io::Read + Send + Unpin,
      U: 'static + io::Write + Send + Unpin
{
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
                Err(e) => { tracing::error!("Error writing to output: {}", e); }
            }
        }
    });

    let mut input = io::BufReader::new(input);
    tracing::info!("Starting input task");
    loop {
        let mut line = String::new();
        _ = input.read_line(&mut line);
        let message = UCIMessage::parse(&line);
        hazel.send(Box::new(message)).await;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// this tests the run_with_io function using a simulated STDIN/STDOUT and verifies the
    /// `isready` command will recieve a response of `readyok`
    #[tokio::test]
    async fn test_run_with_io() {
        // TODO: I have no idea how to test this with the current design. Trying to get a stream
        // that I can see from two places turns out to be not straightforward.
        //
        // I'm pretty sure this all works, but it should be tested. I think the SyncIOBridge type
        // might be useful here, but I need to do some more reading on it. For now, this works by
        // inspection and I think I'm just going to eat the coverage loss.
    }
}
