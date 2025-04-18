/// This module contains the UCI protocol implementation for the engine. It includes as UCI
/// 'primitive' commands the extended/nonstandard commands that Stockfish implements.
use std::fmt::{self, Display, Formatter};


pub const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const LONDON_POSITION_FEN: &str = "r1bqk2r/pp2bppp/2n1pn2/2pp4/3P1B2/2P1PN1P/PP1N1PP1/R2QKB1R b KQkq - 0 7";

pub mod connection;
pub use connection::run;

#[derive(Debug, PartialEq, Clone)]
pub enum UCIMessage {
    // GUI -> Engine
    UCI,
    Debug(bool),
    IsReady,
    SetOption(String, Option<String>),
    Register,
    UCINewGame,
    // NOTE: Position will _always_ have a FEN string as it's first part, never the literal `startpos`.
    // If `startpos` is given, it'll replace with the Starting position FEN. `startpos` is bad and
    // I refuse it.
    Position(String, Vec<String>),
    Go(Vec<String>),
    Stop,
    PonderHit,
    Quit,
    // Engine -> GUI
    ID(String, String),
    ReadyOk,
    UCIOk,
    BestMove(String, Option<String>),
    CopyProtection,
    Registration,
    Info(Vec<String>),
    Option(UCIOption),
    EmptyLine,
    // Stockfish Extensions
    D,
    /*
    Eval,
    Bench,
    Compiler,
    ExportNet,
    Flip
    */
}

#[derive(Debug, PartialEq, Clone)]
pub struct UCIOption {
    name: String,
    option_type: String,
    default: String,
    min: String,
    max: String,
    var: Vec<String>,
}

impl Display for UCIOption {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "option name {} type {} default {} min {} max {} var {}",
            self.name, self.option_type, self.default, self.min, self.max, self.var.join(" "))
    }
}

impl UCIOption {
    const KEYWORDS: [&'static str; 6] = ["name", "type", "default", "min", "max", "var"];

    pub fn new(name: String, option_type: String, default: String, min: String, max: String, var: Vec<String>) -> UCIOption {
        UCIOption {
            name,
            option_type,
            default,
            min,
            max,
            var,
        }
    }

    fn empty() -> UCIOption {
        UCIOption {
            name: "".to_string(),
            option_type: "".to_string(),
            default: "".to_string(),
            min: "".to_string(),
            max: "".to_string(),
            var: vec![],
        }
    }

    fn is_keyword(s: &str) -> bool {
        UCIOption::KEYWORDS.contains(&s)
    }

    fn set(&mut self, keyword: &str, value: String) {
        match keyword {
            "name"    => self.name = value,
            "type"    => self.option_type = value,
            "default" => self.default = value,
            "min"     => self.min = value,
            "max"     => self.max = value,
            "var"     => self.var = value.split_whitespace().map(|s| s.to_string()).collect(),
            "option"  => { }, // ignore
            _         => { panic!("Unknown keyword: {}", keyword) }
        }
    }

    pub fn parse(option: &str) -> UCIOption {
        // split the option by keyword in KEYWORDS
        let mut buf = vec![];
        let mut current_keyword = "option";
        let mut ret = UCIOption::empty();
        let mut parts = option.split_whitespace();
        loop {
            match parts.next() {
                Some(k) if UCIOption::is_keyword(k) => {
                    let value = buf.clone().join(" ");
                    ret.set(current_keyword, value);
                    buf = vec![];
                    current_keyword = k;
                },
                Some(s) => {
                    buf.push(s.to_string());
                },
                None => {
                    // we've run out of parts
                    //      so set the final option
                    ret.set(current_keyword, buf.clone().join(" "));
                    //          and then return
                    return ret;
                }
            }
        }

    }
}

// TODO: Error type

impl Display for UCIMessage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            UCIMessage::UCI => write!(f, "uci"),
            UCIMessage::Debug(b) => write!(f, "debug {}", if *b { "on" } else { "off" }),
            UCIMessage::IsReady => write!(f, "isready"),
            UCIMessage::SetOption(name, value) => {
                match value {
                    Some(v) => write!(f, "setoption name {} value {}", name, v),
                    None => write!(f, "setoption name {}", name)
                }
            },
            UCIMessage::Register => write!(f, "register"),
            UCIMessage::UCINewGame => write!(f, "ucinewgame"),
            UCIMessage::Position(fen, moves) => {
                if fen == START_POSITION_FEN || fen == "startpos" {
                    write!(f, "position startpos moves {}", moves.join(" "))
                } else {
                    write!(f, "position fen {} moves {}", fen, moves.join(" "))
                }
            },
            UCIMessage::Go(args) => write!(f, "go {}", args.join(" ")),
            UCIMessage::Stop => write!(f, "stop"),
            UCIMessage::PonderHit => write!(f, "ponderhit"),
            UCIMessage::Quit => write!(f, "quit"),
            UCIMessage::ID(name, value) => write!(f, "id {} {}", name, value),
            UCIMessage::ReadyOk => write!(f, "readyok"),
            UCIMessage::UCIOk => write!(f, "uciok"),
            UCIMessage::BestMove(best_move, ponder) => {
                match ponder {
                    Some(p) => write!(f, "bestmove {} ponder {}", best_move, p),
                    None => write!(f, "bestmove {}", best_move)
                }
            },
            UCIMessage::CopyProtection => write!(f, "copyprotection"),
            UCIMessage::Registration => write!(f, "registration"),
            UCIMessage::Info(info) => write!(f, "info {}", info.join(" ")),
            UCIMessage::Option(option) => write!(f, "{}", option),
            UCIMessage::EmptyLine => write!(f, ""),
            UCIMessage::D => write!(f, "d"),
        }
    }
}

impl UCIMessage {
    pub fn parse(message: &str) -> UCIMessage {
        let mut parts = message.split_whitespace();
        match parts.next() {
            Some("uci") => UCIMessage::UCI,
            Some("debug") => match parts.next() {
                Some("on") => UCIMessage::Debug(true),
                Some("off") => UCIMessage::Debug(false),
                _ => {
                    tracing::error!("Invalid debug command");
                    // return an error type eventually, maybe? Spec generally suggests ignoring
                    // errors like this.
                    // for now, set debug on if we're not sure what's going on.
                    UCIMessage::Debug(true)
                },
            },
            Some("isready") => UCIMessage::IsReady,
            Some("register") => UCIMessage::Register,
            Some("ucinewgame") => UCIMessage::UCINewGame,
            Some("setoption") => {
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
                let mut pos_spec = parts.clone().take_while(|&s| s != "moves").map(|s| s.to_string());
                let moves : Vec<String> = parts.skip_while(|&s| s != "moves").skip(1).map(|s| s.to_string() ).collect();


                if pos_spec.next().unwrap() == "startpos" {
                    UCIMessage::Position(START_POSITION_FEN.to_string(), moves)
                } else {
                    UCIMessage::Position(
                        pos_spec.collect::<Vec<String>>().join(" "),
                        moves
                    )
                }
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
            Some("uciok") => UCIMessage::UCIOk,
            Some("bestmove") => {
                let best_move = parts.next().unwrap().to_string();
                match parts.next() {
                    Some("ponder") => UCIMessage::BestMove(best_move, Some(parts.next().unwrap().to_string())),
                    _ => UCIMessage::BestMove(best_move, None)
                }
            }
            Some("copyprotection") => UCIMessage::CopyProtection,
            Some("registration") => UCIMessage::Registration,
            Some("info") => {
                UCIMessage::Info(
                    parts.collect::<Vec<&str>>()
                        .chunks(2)
                        .map(|s| s.join(" ").to_string())
                        .collect()
                )
            }
            Some("option") => {
                UCIMessage::Option(UCIOption::parse(message))
            }
            Some("readyok") => UCIMessage::ReadyOk,
            Some("d") => UCIMessage::D,
            Some(_) => panic!("Unknown UCI message: {}", message),
            None => { UCIMessage::EmptyLine }
        }
    }

    pub fn has_response(&self) -> bool {
        !matches!(self, UCIMessage::UCINewGame | UCIMessage::Position(_, _) | UCIMessage::Quit)
    }

    pub fn is_complete(&self, last_line: &str) -> bool {
        match self {
            UCIMessage::UCI => last_line == "uciok",
            UCIMessage::IsReady => last_line == "readyok",
            UCIMessage::Go(_) => last_line.starts_with("bestmove"),
            UCIMessage::Stop => last_line.starts_with("bestmove"),
            UCIMessage::D => last_line.starts_with("Checkers:"),
            _ => false,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    mod display {
        use super::*;

        macro_rules! assert_displays {
            ($input:expr, $expected:expr) => {
                assert_eq!($input.to_string(), $expected);
            };
        }

        #[test]
        fn displays_uci() {
            assert_displays!(UCIMessage::UCI, "uci");
        }

        #[test]
        fn displays_debug() {
            assert_displays!(UCIMessage::Debug(true), "debug on");
            assert_displays!(UCIMessage::Debug(false), "debug off");
        }

        #[test]
        fn displays_is_ready() {
            assert_displays!(UCIMessage::IsReady, "isready");
        }

        #[test]
        fn displays_set_option() {
            assert_displays!(UCIMessage::SetOption("NullMove".to_string(), Some("true".to_string())), "setoption name NullMove value true");
            assert_displays!(UCIMessage::SetOption("NullMove".to_string(), None), "setoption name NullMove");
        }

        #[test]
        fn displays_register() {
            assert_displays!(UCIMessage::Register, "register");
        }

        #[test]
        fn displays_uci_new_game() {
            assert_displays!(UCIMessage::UCINewGame, "ucinewgame");
        }

        #[test]
        fn displays_position() {
            assert_displays!(UCIMessage::Position(START_POSITION_FEN.to_string(), vec!["e2e4".to_string(), "e7e5".to_string()]), "position startpos moves e2e4 e7e5");
            assert_displays!(UCIMessage::Position(LONDON_POSITION_FEN.to_string(), vec![]), format!("position fen {} moves ", LONDON_POSITION_FEN));
        }

        #[test]
        fn displays_go() {
            assert_displays!(UCIMessage::Go(vec!["wtime".to_string(), "1000".to_string(), "btime".to_string(), "1000".to_string()]), "go wtime 1000 btime 1000");
        }

        #[test]
        fn displays_stop() {
            assert_displays!(UCIMessage::Stop, "stop");
        }

        #[test]
        fn displays_ponder_hit() {
            assert_displays!(UCIMessage::PonderHit, "ponderhit");
        }

        #[test]
        fn displays_quit() {
            assert_displays!(UCIMessage::Quit, "quit");
        }

        #[test]
        fn displays_id() {
            assert_displays!(UCIMessage::ID("name".to_string(), "Hazel 0.1".to_string()), "id name Hazel 0.1");
        }

        #[test]
        fn displays_ready_ok() {
            assert_displays!(UCIMessage::ReadyOk, "readyok");
        }

        #[test]
        fn displays_best_move() {
            assert_displays!(UCIMessage::BestMove("e2e4".to_string(), Some("e7e5".to_string())), "bestmove e2e4 ponder e7e5");
            assert_displays!(UCIMessage::BestMove("e2e4".to_string(), None), "bestmove e2e4");
        }

        #[test]
        fn displays_copy_protection() {
            assert_displays!(UCIMessage::CopyProtection, "copyprotection");
        }

        #[test]
        fn displays_registration() {
            assert_displays!(UCIMessage::Registration, "registration");
        }

        #[test]
        fn displays_info() {
            assert_displays!(UCIMessage::Info(vec![
                "depth 1".to_string(),
                "seldepth 1".to_string(),
                "nodes 1".to_string(),
                "nps 1".to_string(),
                "time 1".to_string(),
                "pv e2e4".to_string()
            ]), "info depth 1 seldepth 1 nodes 1 nps 1 time 1 pv e2e4");
        }

        #[test]
        fn displays_option() {
            assert_displays!(UCIMessage::Option(UCIOption::new(
                "NullMove".to_string(),
                "check".to_string(),
                "true".to_string(),
                "".to_string(),
                "".to_string(),
                vec![]
            )), "option name NullMove type check default true min  max  var ");
        }

        #[test]
        fn displays_empty_line() {
            assert_displays!(UCIMessage::EmptyLine, "");
        }

        #[test]
        fn displays_d() {
            assert_displays!(UCIMessage::D, "d");
        }
    }

    mod parse {
        use super::*;

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
        fn parses_uciok() {
            assert_parses!("uciok", UCIMessage::UCIOk);
        }

        #[test]
        fn parses_empty_line() {
            assert_parses!("", UCIMessage::EmptyLine);
        }

        #[test]
        fn parses_empty_line_with_whitespace() {
            assert_parses!("  ", UCIMessage::EmptyLine);
        }

        #[test]
        fn parses_readyok() {
            assert_parses!("readyok", UCIMessage::ReadyOk);
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
        fn parses_position_with_non_startpos_fen() {
            assert_parses!(
                &format!("position fen {} moves", LONDON_POSITION_FEN),
                UCIMessage::Position(
                    LONDON_POSITION_FEN.to_string(),
                    vec![]
                )
            );
        }

        #[test]
        fn parses_position() {
            assert_parses!(
                "position startpos moves e2e4 e7e5",
                UCIMessage::Position(
                    // NOTE: See note in the struct defn.
                    START_POSITION_FEN.to_string(),
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

        #[test]
        fn parses_ready_ok() {
            assert_parses!("readyok", UCIMessage::ReadyOk);
        }

        #[test]
        fn parses_best_move() {
            assert_parses!(
                "bestmove e2e4 ponder e7e5",
                UCIMessage::BestMove("e2e4".to_string(), Some("e7e5".to_string()))
            );
            assert_parses!(
                "bestmove e2e4",
                UCIMessage::BestMove("e2e4".to_string(), None)
            );
        }

        #[test]
        fn parses_copy_protection() {
            assert_parses!("copyprotection", UCIMessage::CopyProtection);
        }

        #[test]
        fn parses_registration() {
            assert_parses!("registration", UCIMessage::Registration);
        }

        #[test]
        fn parses_info() {
            assert_parses!(
                "info depth 1 seldepth 1 nodes 1 nps 1 time 1 pv e2e4",
                UCIMessage::Info(vec![
                    "depth 1".to_string(),
                    "seldepth 1".to_string(),
                    "nodes 1".to_string(),
                    "nps 1".to_string(),
                    "time 1".to_string(),
                    "pv e2e4".to_string()
                ])
            );
        }

        #[test]
        #[allow(non_snake_case)] // I like naming puns, especially when they're this bad.
        fn parses_CamelCase_option() {
            assert_parses!(
                "option name NullMove type check default true",
                UCIMessage::Option(UCIOption::new(
                    "NullMove".to_string(),
                    "check".to_string(),
                    "true".to_string(),
                    "".to_string(),
                    "".to_string(),
                    vec![]
                ))
            );
        }

        #[test]
        fn parses_multi_word_option() {
            assert_parses!(
                "option name Debug Log File type string default",
                UCIMessage::Option(UCIOption::new(
                    "Debug Log File".to_string(),
                    "string".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    vec![]
                ))
            );
        }

        #[test]
        fn parses_all_default_option() {
            assert_parses!(
                "option name foo",
                UCIMessage::Option(UCIOption::new(
                    "foo".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    vec![]
                ))
            );
        }

        #[test]
        fn parses_option_with_var() {
            assert_parses!(
                "option name Threads type spin default 1 min 1 max 1024",
                UCIMessage::Option(UCIOption::new(
                    "Threads".to_string(),
                    "spin".to_string(),
                    "1".to_string(),
                    "1".to_string(),
                    "1024".to_string(),
                    vec![]
                ))
            );
        }

        #[test]
        fn parses_option_with_var_and_default() {
            assert_parses!(
                "option name Threads type spin default 1 min 1 max 1024 var 1 2 4 8 16 32 64 128 256 512 1024",
                UCIMessage::Option(UCIOption::new(
                    "Threads".to_string(),
                    "spin".to_string(),
                    "1".to_string(),
                    "1".to_string(),
                    "1024".to_string(),
                    vec![
                        "1".to_string(),
                        "2".to_string(),
                        "4".to_string(),
                        "8".to_string(),
                        "16".to_string(),
                        "32".to_string(),
                        "64".to_string(),
                        "128".to_string(),
                        "256".to_string(),
                        "512".to_string(),
                        "1024".to_string(),
                    ]
                ))
            );
        }

        #[test]
        #[allow(non_snake_case)] // it looks like a typo if we don't capitalize it
        fn parses_stockfish_extension_D() {
            assert_parses!("d", UCIMessage::D);
        }
    }
}
