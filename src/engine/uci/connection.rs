

// Open a (provided) communication stream, read it by line, and parse it into UCI messages.

use std::io;

#[cfg_attr(test, mutants::skip)]
pub async fn run() -> io::Result<()> {
    run_with_io(io::stdin(), io::stdout()).await
}

// take arbitrary IO Streams and use it as if it were STDIN/STDOUT
// to do the `run` function above
pub async fn run_with_io<T: io::Read, U: io::Write>(input: T, mut output: U) -> io::Result<()> {
    /*
    let handle = io::BufReader::new(input);
    let mut driver = Driver::new();

    // TODO: Refactor, it should map all Input to the `process_uci` function, eventually to some
    // generic `process` function for different message types.
    // It should, separately, map all output to stdout via the `listen` function
    for line in handle.lines() {
        let line = line?;
        let message = UCIMessage::parse(&line);
        info!("Received UCI message: {:?}", message);
        let response = driver.process_uci(&message).await;
        for r in response {
            output.write_all(r.to_string().as_bytes())?;
        }
    }
*/
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_with_dummy_io() {
        let input = "uci\nisready\n".as_bytes();
        let output = Vec::new();
        let result = run_with_io(input, output).await;
        assert!(result.is_ok());
    }
}
