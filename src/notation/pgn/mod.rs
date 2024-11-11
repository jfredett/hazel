use std::path::Path;
use std::io::Error;

use nom::{branch::alt, bytes::complete::{tag, take_while}, character::{complete::{alpha1, char, multispace0, multispace1, newline, none_of, one_of, space1}, is_digit}, combinator::{not, opt}, multi::{many0, many1}, sequence::delimited, IResult};
use tracing::debug;

use crate::{constants::File, coup::rep::Move, game::variation::Variation, types::Piece};

use super::Square;

#[derive(Debug)]
pub struct PGN {
    tag_pairs: TagPairs,
    variation: Variation,
}

#[derive(Debug, Default, PartialEq)]
pub struct TagPairs(Vec<TagPair>);

impl TagPairs {
    pub fn parse(input: &str) -> IResult<&str, TagPairs> {
        // a set of tagpairs looks like:
        //
        // [Word1 "String"]
        // [Word2 "String"]
        // [Word3 "String"]
        // [Word4 "String"]
        // [Word5 "String"]
        // [Word6 "String"]
        // ...
        //
        // Terminated by two newlines.

        let (input, tag_pairs) = many1(TagPair::parse)(input)?;
        Ok((input, TagPairs(tag_pairs)))
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct TagPair {
    name: String,
    value: String,
}

impl TagPair {
    pub fn parse(input: &str) -> IResult<&str, TagPair> {
        // a tagpair looks like: [Word "String"]
        let (input, _) = char('[')(input)?;
        let (input, name) = alpha1(input)?;
        let (input, _) = space1(input)?;
        let (input, value) = delimited(
            char('"'),
            nom::bytes::complete::take_until("\""),
            char('"'),
        )(input)?;
        let (input, _) = char(']')(input)?;
        let (input, _) = many0(newline)(input)?;

        Ok((input, TagPair { name: name.to_string(), value: value.to_string() }))
    }
}

impl Variation {
    /// Given a PGN ply notation like:
    ///
    /// XX. YY ZZ
    ///
    /// This returns a result of ("YY ZZ", XX as usize)
    ///
    /// Note that it consumes _all available whitespace_ after the ply number, and optionally will
    /// read the period after the ply number if it is present.
    pub fn half_ply_number(input: &str) -> IResult<&str, usize> {
        let (input, half_ply_num) = delimited(multispace0, many1(one_of("1234567890")), alt((tag("."), multispace1)))(input)?;

        // remove any excess whitespace because I'm not lexing.
        let (input, _) = multispace0(input)?;
        let half_ply = half_ply_num.iter().collect::<String>().parse::<usize>().unwrap();
        Ok((input, half_ply))
    }
}
/*
impl<S : Into<String>> TryFrom<S> for PGN {
type Error = Error;

fn try_from(pgn: S) -> Result<Self, Self::Error> {
todo!();
}
}
*/

impl PGN {
    /// Reads a PGN file and returns a PGN struct
    pub fn load<P : AsRef<Path>>(path: P) -> Result<Self, Error> {
        let pgn_data = std::fs::read_to_string(path)?;
        Ok(PGN::parse(pgn_data))
    }

    pub fn parse(pgn_data: String) -> Self {
        let mut pgn = PGN::default();



        todo!();
    }

    pub fn default() -> Self {
        PGN {
            tag_pairs: TagPairs::default(),
            variation: Variation::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod halfply_marker {
        use crate::types::Color;

        use super::*;


        #[test]
        fn parses_halfply_marker() {
            let hp = "1. d4 d5";
            let (input, half_ply) = Variation::half_ply_number(hp).unwrap();

            assert_eq!(input, "d4 d5");
            assert_eq!(half_ply, 1);
        }

        #[test]
        fn parses_halfply_marker_with_multiple_digits() {
            let hp = "30. d4 d5";
            let (input, half_ply) = Variation::half_ply_number(hp).unwrap();

            assert_eq!(input, "d4 d5");
            assert_eq!(half_ply, 30);
        }

        #[test]
        fn parses_halfply_missing_period() {
            let hp = "30 d4 d5";
            let (input, half_ply) = Variation::half_ply_number(hp).unwrap();

            assert_eq!(input, "d4 d5");
            assert_eq!(half_ply, 30);
        }


        #[quickcheck]
        fn parses_halfply_numbers_correctly(half_ply: usize, full_move: bool, include_period: bool) -> bool {
            if half_ply == 0 { return true; }
            let period = if include_period { "." } else { "" };

            let hp = if full_move {
                format!("{}{} x4 y8", half_ply, period)
            } else {
                format!("{}{} z1", half_ply, period)
            };

            let (input, parsed_half_ply) = Variation::half_ply_number(&hp).unwrap();

            let input_check = if full_move { input == "x4 y8" } else { input == "z1" };
            input_check && (half_ply == parsed_half_ply)
        }

    }
    mod tagpairs {
        use tracing::debug;

        use super::*;


        #[test]
        #[tracing_test::traced_test]
        fn parses_tagpairs() {
            let input = r#"[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
[Round "29"]
[White "Fischer, Robert J."]
[Black "Spassky, Boris V."]
[Result "1/2-1/2"]
"#;
            let tagpairs = TagPairs::parse(input).unwrap();
            debug!("{:?}", tagpairs);
            assert_eq!(tagpairs, ("", TagPairs { 0: vec![
                TagPair { name: "Event".to_string(), value: "F/S Return Match".to_string() },
                TagPair { name: "Site".to_string(), value: "Belgrade, Serbia JUG".to_string() },
                TagPair { name: "Date".to_string(), value: "1992.11.04".to_string() },
                TagPair { name: "Round".to_string(), value: "29".to_string() },
                TagPair { name: "White".to_string(), value: "Fischer, Robert J.".to_string() },
                TagPair { name: "Black".to_string(), value: "Spassky, Boris V.".to_string() },
                TagPair { name: "Result".to_string(), value: "1/2-1/2".to_string() },
            ]}));
        }
    }

    mod tagpair {
        use super::*;

        #[test]
        fn should_not_parse_tagpair_with_no_name() {
            let tagpair = TagPair::parse(r#"[ "F/S Return Match"]"#);
            assert!(tagpair.is_err());
        }

        #[test]
        fn should_notparse_tagpair_with_no_value() {
            let tagpair = TagPair::parse(r#"[Event ""]"#);
            assert!(tagpair.is_err());
        }

        #[test]
        fn parses_tagpair() {
            let tagpair = TagPair::parse(r#"[Event "F/S Return Match"]"#).unwrap();
            assert_eq!(tagpair, ("", TagPair { name: "Event".to_string(), value: "F/S Return Match".to_string() }));
        }
    }

    mod pgn {
        use super::*;

        #[test]
        fn imports_from_pgn_with_no_variations_or_halt() {
            let pgn = PGN::load("tests/pgn/no-variations-and-halts.pgn").unwrap();
        }

        #[test]
        fn imports_from_pgn_with_no_variations_and_halt() {
            let pgn = PGN::load("tests/pgn/no-variations-and-no-halt.pgn").unwrap();
        }


        #[test]
        fn imports_from_pgn_with_variations_and_no_halt() {
            let pgn = PGN::load("tests/pgn/with-variations-no-halt.pgn").unwrap();
        }

        #[test]
        fn imports_from_pgn_with_variations_and_halt() {
            let pgn = PGN::load("tests/pgn/with-variations-halts.pgn").unwrap();
        }

        #[test]
        fn imports_from_pgn_with_nested_variations_and_no_halt() {
            let pgn = PGN::load("tests/pgn/with-nested-variations-no-halt.pgn").unwrap();
        }

        #[test]
        fn imports_from_pgn_with_nested_variations_and_halt() {
            let pgn = PGN::load("tests/pgn/with-nested-variations-halts.pgn").unwrap();
        }
    }
}
