// A driver for talking to a stockfish instance over UCI
//
//
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

#[allow(unused_imports)] // I want all the tracing stuff available regardless of whether it's used
use tracing::*;

use crate::uci::UCIMessage;
use crate::engine::Engine;

#[derive(Debug)]
pub struct Stockfish {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Stockfish {
    fn is_response_complete(&self, message: &UCIMessage, last_line: &str) -> bool {
        match message {
            UCIMessage::UCI => last_line == "uciok",
            UCIMessage::IsReady => last_line == "readyok",
            UCIMessage::Go(_) => last_line.starts_with("bestmove"),
            UCIMessage::Stop => last_line.starts_with("bestmove"),
            _ => false,
        }
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        writeln!(self.stdin, "quit")?;
        self.child.wait()?;
        Ok(())
    }
}

impl Drop for Stockfish {
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

    fn exec_message(&mut self, message: &str) -> Vec<UCIMessage> {
        self.exec(UCIMessage::parse(message))
    }

    fn exec(&mut self, message: UCIMessage) -> Vec<UCIMessage> {
        let cmd_str = message.to_string();

        writeln!(self.stdin, "{}", cmd_str).expect("Failed to write to stockfish");

        let mut response = Vec::new();
        loop {
            let mut line = String::new();
            let bytes_read = self.stdout.read_line(&mut line).expect("Failed to read from stockfish");

            if bytes_read == 0 { break; } // EOF reached.

            let line = line.trim_end();
            response.push(UCIMessage::parse(line));

            if self.is_response_complete(&message, &line) { break; } // Check if the response is complete.
        }
        response
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;
    use crate::uci::{UCIMessage, UCIOption};

    #[traced_test]
    #[test]
    fn stockfish_connects() {
        let mut stockfish = Stockfish::new();
        let response = stockfish.exec_message("uci");
        assert_eq!(response, vec![
            UCIMessage::ID("name".to_string(), "Stockfish 16.1".to_string()),
            UCIMessage::ID("author".to_string(), "the Stockfish developers (see AUTHORS file)".to_string()),
            UCIMessage::EmptyLine,
            UCIMessage::Option( UCIOption::new( "Debug Log File".to_string(), "string".to_string(), "".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "Threads".to_string(), "spin".to_string(), "1".to_string(), "1".to_string(), "1024".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "Hash".to_string(), "spin".to_string(), "16".to_string(), "1".to_string(), "33554432".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "Clear Hash".to_string(), "button".to_string(), "".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "Ponder".to_string(), "check".to_string(), "false".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "MultiPV".to_string(), "spin".to_string(), "1".to_string(), "1".to_string(), "256".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "Skill Level".to_string(), "spin".to_string(), "20".to_string(), "0".to_string(), "20".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "Move Overhead".to_string(), "spin".to_string(), "10".to_string(), "0".to_string(), "5000".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "nodestime".to_string(), "spin".to_string(), "0".to_string(), "0".to_string(), "10000".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "UCI_Chess960".to_string(), "check".to_string(), "false".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "UCI_LimitStrength".to_string(), "check".to_string(), "false".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "UCI_Elo".to_string(), "spin".to_string(), "1320".to_string(), "1320".to_string(), "3190".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "UCI_ShowWDL".to_string(), "check".to_string(), "false".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "SyzygyPath".to_string(), "string".to_string(), "<empty>".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "SyzygyProbeDepth".to_string(), "spin".to_string(), "1".to_string(), "1".to_string(), "100".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "Syzygy50MoveRule".to_string(), "check".to_string(), "true".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "SyzygyProbeLimit".to_string(), "spin".to_string(), "7".to_string(), "0".to_string(), "7".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "EvalFile".to_string(), "string".to_string(), "nn-b1a57edbea57.nnue".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::Option( UCIOption::new( "EvalFileSmall".to_string(), "string".to_string(), "nn-baff1ede1f90.nnue".to_string(), "".to_string(), "".to_string(), vec![])),
            UCIMessage::UCIOk,
        ]);
    }
}


