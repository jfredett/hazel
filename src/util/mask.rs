// This appears to be wrong in this case. The items it identifies are used in the async functions,
// but it reports them not being used at all.
#![allow(dead_code)]

use tracing::*;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, Command};
use std::process::Stdio;
use tokio_stream::StreamExt;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
use std::fmt::Debug;

/// Mask wraps a unix-y process and all it's various IO streams, and presents a Rust-y, tokio-ified
/// interface thereto.
pub struct Mask {
    command: String,
    pub stdin: ChildStdin,
    pub stdout: ReceiverStream<String>,
    pub stderr: ReceiverStream<String>,
}

impl Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mask")
            .field("command", &self.command)
            .finish()
    }
}

const BUFFER_SIZE: usize = 256;

impl Mask {
    /// Creates a new instance of `Mask` by starting the Stockfish process.
    #[instrument]
    pub async fn new(command: &str) -> tokio::io::Result<Self> {
        let mut process = Command::new(command.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = process.stdin.take().expect("Failed to open stdin");
        let stdout = process.stdout.take().expect("Failed to open stdout");
        let stderr = process.stderr.take().expect("Failed to open stderr");

        let (out_tx, out_rx) = channel(BUFFER_SIZE);
        let (err_tx, err_rx) = channel(BUFFER_SIZE);

        let out_rx = ReceiverStream::new(out_rx);
        let err_rx = ReceiverStream::new(err_rx);

        tokio::spawn(async move {
            let mut stdout = BufReader::new(stdout).lines();
            while let Some(line) = stdout.next_line().await.unwrap() {
                out_tx.send(line).await.unwrap();
            }
        });

        tokio::spawn(async move {
            let mut stderr = BufReader::new(stderr).lines();
            while let Some(line) = stderr.next_line().await.unwrap() {
                err_tx.send(line).await.unwrap();
            }
        });

        Ok(Mask {
            command: command.to_string(),
            stdin,
            stdout: out_rx,
            stderr: err_rx,
        })
    }


    /// Sends a command to the stdin of the child.
    #[instrument]
    pub async fn send(&mut self, command: &str) -> tokio::io::Result<()> {
        self.stdin.write_all(command.as_bytes()).await?;
        self.stdin.write_all(b"\n").await?;
        self.stdin.flush().await
    }

    /// Reads a single line of response from the child, blocking until it is available.
    #[instrument]
    pub async fn read(&mut self) -> Option<String> {
        self.stdout.next().await
    }

    /// Reads a single line of error from the child, blocking until it is available.
    #[instrument]
    pub async fn read_err(&mut self) -> Option<String> {
        self.stderr.next().await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod mask {
        use tracing_test::traced_test;

        use super::*;

        #[tokio::test]
        async fn works_with_stockfish() {
            let mut mask = Mask::new("stockfish").await.unwrap();
            // Burn off the initial stanza from starting stockfish
            let _ = mask.read().await;
            mask.send("isready").await.unwrap();
            let response = mask.read().await.unwrap();
            assert_eq!(response, "readyok");
        }

        #[tokio::test]
        async fn works_with_stockfish_when_there_is_no_output() {
            let mut mask = Mask::new("stockfish").await.unwrap();
            // Burn off the initial stanza from starting stockfish
            let _ = mask.read().await;
            mask.send("position startpos moves").await.unwrap();
            // in the old version, this would block w/o hacking it, but it shouldn't block now.
            // now if we send an 'isready', it should return.
            mask.send("isready").await.unwrap();
            let response = mask.read().await.unwrap();
            assert_eq!(response, "readyok");
        }
    }

}
