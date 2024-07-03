
// TODO: Parse directly into (hazel) structs?
// TODO: Internal Enums for, e.g., `go`, `info`, etc.

use tracing::{instrument, info, debug};

#[derive(Debug, PartialEq)]
pub enum UCIMessage {
    // GUI -> Engine
    UCI,
    Debug(bool),
    IsReady,
    SetOption(String, Option<String>),
    Register,
    UCINewGame,
    Position(Vec<String>, Vec<String>),
    Go(Vec<String>),
    Stop,
    PonderHit,
    Quit,
    // Engine -> GUI
    ID(String, String),
    UCIOptions(Vec<String>),
    ReadyOk,
    BestMove(String, Option<String>),
    CopyProtection,
    Registration,
    Info(Vec<String>),
    Option(String),
}

// TODO: Error type

impl UCIMessage {
    #[instrument]
    pub fn parse(message: &str) -> UCIMessage {
        let mut parts = message.split_whitespace();
        match parts.next() {
            Some("uci") => UCIMessage::UCI,
            Some("debug") => match parts.next() {
                Some("on") => UCIMessage::Debug(true),
                Some("off") => UCIMessage::Debug(false),
                _ => panic!("Invalid debug command"),
            },
            Some("isready") => UCIMessage::IsReady,
            Some("register") => UCIMessage::Register,
            Some("ucinewgame") => UCIMessage::UCINewGame,
            Some("setoption") => {
                info!("setoption");
                let name = parts.nth(1).unwrap().to_string();
                match parts.next() {
                    Some("value") => {
                        let value = parts.next().map(|s| s.to_string());
                        UCIMessage::SetOption(name, value)
                    }
                    _ => UCIMessage::SetOption(name, None)
                }
            }
            Some("position") => {
                let mut position = vec![];
                let mut moves = vec![];
                for part in parts {
                    match part {
                        "startpos" => position.push("startpos".to_string()),
                        "moves" => (),
                        _ => moves.push(part.to_string())
                    }
                }
                UCIMessage::Position(position, moves)
            }
            Some("go") => UCIMessage::Go(parts.map(|s| s.to_string()).collect()),
            Some("stop") => UCIMessage::Stop,
            Some("ponderhit") => UCIMessage::PonderHit,
            Some("quit") => UCIMessage::Quit,
            Some("id") => {
                let name = parts.next().unwrap().to_string();
                let value = parts.collect::<Vec<&str>>().join(" ");
                UCIMessage::ID(name, value)
            }
            Some(&_) => todo!(),
            None => todo!()
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    macro_rules! assert_parses {
        ($input:expr, $expected:expr) => {
            assert_eq!(UCIMessage::parse($input), $expected);
        };
    }

    #[test]
    fn parses_uci() {
        assert_parses!("uci", UCIMessage::UCI);
    }

    #[test]
    fn parses_debug() {
        assert_eq!(UCIMessage::parse("debug on"), UCIMessage::Debug(true));
        assert_eq!(UCIMessage::parse("debug off"), UCIMessage::Debug(false));
    }

    #[test]
    fn parses_is_ready() {
        assert_parses!("isready", UCIMessage::IsReady);
    }

    #[traced_test]
    #[test]
    fn parses_set_option() {
        assert_parses!(
            "setoption name NullMove value true",
            UCIMessage::SetOption("NullMove".to_string(), Some("true".to_string()))
        );
        assert_parses!(
            "setoption name NullMove",
            UCIMessage::SetOption("NullMove".to_string(), None)
        );
    }

    #[test]
    fn parses_register() {
        assert_parses!("register", UCIMessage::Register);
    }

    #[test]
    fn parses_uci_new_game() {
        assert_parses!("ucinewgame", UCIMessage::UCINewGame);
    }

    #[test]
    fn parses_position() {
        assert_parses!(
            "position startpos moves e2e4 e7e5",
            UCIMessage::Position(
                vec!["startpos".to_string()],
                vec!["e2e4".to_string(), "e7e5".to_string()]
            )
        );
    }

    #[test]
    fn parses_go() {
        assert_parses!(
            "go wtime 1000 btime 1000",
            UCIMessage::Go(vec!["wtime".to_string(), "1000".to_string(), "btime".to_string(), "1000".to_string()])
        );
    }

    #[test]
    fn parses_stop() {
        assert_parses!("stop", UCIMessage::Stop);
    }

    #[test]
    fn parses_ponder_hit() {
        assert_parses!("ponderhit", UCIMessage::PonderHit);
    }

    #[test]
    fn parses_quit() {
        assert_parses!("quit", UCIMessage::Quit);
    }

    #[test]
    fn parses_id() {
        assert_parses!(
            "id name Hazel 0.1",
            UCIMessage::ID("name".to_string(), "Hazel 0.1".to_string())
        );
    }
}