use std::path::Path;
use std::io::Error;

use nom::{character::complete::{alpha1, char, newline, none_of, space1}, multi::{many0, many1}, sequence::delimited, IResult};

use crate::game::variation::Variation;

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
