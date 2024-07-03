

// Open a (provided) communication stream, read it by line, and parse it into UCI messages.

use std::io::{self, BufRead};

use tracing::*;


use crate::uci::UCIMessage;

pub fn run() -> io::Result<()> {
    run_with_io(io::stdin(), io::stdout())
}

// take arbitrary IO Streams and use it as if it were STDIN/STDOUT
// to do the `run` function above
pub fn run_with_io<T: io::Read, U: io::Write>(input: T, output: U) -> io::Result<()> {
    let handle = io::BufReader::new(input);

    for line in handle.lines() {
        let line = line?;
        let message = UCIMessage::parse(&line);
        info!("Received UCI message: {:?}", message);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_with_dummy_io() {
        let input = "uci\nisready\n".as_bytes();
        let output = Vec::new();
        let result = run_with_io(input, output);
        assert!(result.is_ok());
    }
}
