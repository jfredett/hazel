use nom::{bytes::complete::take_until, character::complete::{alpha1, char, space1}, multi::{many0, many1}, sequence::delimited};
use nom::IResult;

use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TagPair {
    pub(crate) name: String, // FIXME: Temporary pub(crate)
    pub(crate) value: String, // FIXME: Temporary pub(crate)
}

impl TagPair {
    pub fn parse(input: &str) -> IResult<&str, TagPair> {
        // a tagpair looks like: [Word "String"]
        let (input, _) = char('[')(input)?;
        let (input, name) = alpha1(input)?;
        let (input, _) = space1(input)?;
        let (input, value) = delimited(
            char('"'),
            take_until("\""),
            char('"'),
        )(input)?;
        let (input, _) = char(']')(input)?;
        let (input, _) = opt(newline)(input)?;

        Ok((input, TagPair { name: name.to_string(), value: value.to_string() }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    mod tagpairs {

        use super::*;


        #[test]
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
            assert_eq!(tagpairs, ("", TagPairs(vec![
                TagPair { name: "Event".to_string(), value: "F/S Return Match".to_string() },
                TagPair { name: "Site".to_string(), value: "Belgrade, Serbia JUG".to_string() },
                TagPair { name: "Date".to_string(), value: "1992.11.04".to_string() },
                TagPair { name: "Round".to_string(), value: "29".to_string() },
                TagPair { name: "White".to_string(), value: "Fischer, Robert J.".to_string() },
                TagPair { name: "Black".to_string(), value: "Spassky, Boris V.".to_string() },
                TagPair { name: "Result".to_string(), value: "1/2-1/2".to_string() },
            ])));
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
        fn should_parse_tagpair_with_no_value() {
            let tagpair = TagPair::parse(r#"[Event ""]"#);
            assert_eq!(tagpair, Ok(("", TagPair { name: "Event".to_string(), value: "".to_string() })));
        }

        #[test]
        fn parses_tagpair() {
            let tagpair = TagPair::parse(r#"[Event "F/S Return Match"]"#).unwrap();
            assert_eq!(tagpair, ("", TagPair { name: "Event".to_string(), value: "F/S Return Match".to_string() }));
        }
    }
}
