// A driver for talking to a stockfish instance over UCI
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

#[allow(unused_imports)] // I want all the tracing stuff available regardless of whether it's used
use tracing::*;

use crate::engine::uci::UCIMessage;
use crate::engine::Engine;

#[derive(Debug)]
pub struct Stockfish {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Stockfish {
    #[cfg_attr(test, mutants::skip)]
    pub fn close(&mut self) -> std::io::Result<()> {
        writeln!(self.stdin, "quit")?;
        self.child.wait()?;
        Ok(())
    }
}

impl Drop for Stockfish {
    #[cfg_attr(test, mutants::skip)]
    fn drop(&mut self) {
        // Attempt to close the process gracefully.
        let _ = self.close();
    }
}

impl Stockfish {
    pub fn new() -> Stockfish {
        // start the stockfish process
        let mut child = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start stockfish");

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let mut stdout = BufReader::new(stdout);


        // just burn off everything it outputs until we ask it to get ready
        let mut line = String::new();
        let _ = stdout.read_line(&mut line).expect("Failed to first stanza from stockfish");

        Stockfish { child, stdin, stdout }
    }
}

impl Engine<UCIMessage> for Stockfish {

    #[instrument]
    fn exec_message(&mut self, message: &str) -> Vec<UCIMessage> {
        self.exec(&UCIMessage::parse(message))
    }

    #[instrument]
    fn exec(&mut self, message: &UCIMessage) -> Vec<UCIMessage> {
        debug!("{}", message.to_string());
        let cmd_str = message.to_string();

        writeln!(self.stdin, "{}", cmd_str).expect("Failed to write to stockfish");

        if message.has_response() {
            let mut response = Vec::new();
            loop {
                let mut line = String::new();
                let bytes_read = self.stdout.read_line(&mut line).expect("Failed to read from stockfish");

                if bytes_read == 0 { break; } // EOF reached.

                let line = line.trim_end();
                if *message != UCIMessage::D {
                    response.push(UCIMessage::parse(line));
                } else {
                    debug!("{}", line);
                }

                if message.is_complete(&line) { break; } // Check if the response is complete.
            }
            return response
        } else {
            return vec![]
        }
    }

}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;
    use tracing_test::traced_test;
    use crate::engine::uci::UCIMessage;

    #[traced_test]
    #[test]
    fn stockfish_connects() {
        let mut stockfish = Stockfish::new();
        let response = stockfish.exec_message("uci");
        // I'm only checking the first couple stanzas which I hope stay consistent across versions.
        // I just want a canary to make sure I connected, UCI is timeless.
        assert_matches!(&response[0], UCIMessage::ID(key, value) if key == "name" && value.starts_with("Stockfish"));
        assert_matches!(&response[1], UCIMessage::ID(key, value) if key == "author" && value.starts_with("the Stockfish developers"));
        assert_matches!(&response.last().unwrap(), UCIMessage::UCIOk);
    }
}


