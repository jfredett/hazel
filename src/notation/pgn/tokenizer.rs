use nom::{branch::alt, bytes::complete::tag, character::complete::{char, alpha1, multispace0, newline, one_of}, combinator::opt, multi::many1, sequence::delimited, IResult};
use tracing::debug;

use crate::{game::reason::Reason, types::Color};

use super::TagPair;

#[derive(Debug, Clone, PartialEq)]
pub enum PGNToken {
    GameStart,
    GameEnd,
    TagPair(TagPair),
    Turn(usize), // covers both white-starts and black-starts turns.
    Coup(String),
    Annotation(String),
    Comment(String),
    VariationStart,
    VariationEnd,
    Halt(Reason),
}

const SAN_MOVE_CHARS: &str = "abcdefghRNBQKx12345678";
const PGN_ANNOTATIONS: &str = "!?+-.#";

impl PGNToken {

    pub fn turn(input: &str) -> IResult<&str, PGNToken> {
        let (input, number_chars) = delimited(
            multispace0,
            many1(one_of("1234567890")),
            alt((
                // Order matters
                tag("..."),
                tag("."),
            ))
        )(input)?;
        let number_str = number_chars.iter().collect::<String>();
        let number = number_str.parse::<usize>().unwrap();

        let (input, _) = multispace0(input)?;

        Ok((input, PGNToken::Turn(number)))
    }

    pub fn long_castle(input: &str) -> IResult<&str, Vec<char>> {
        let (input, _) = alt((
            tag("O-O-O"), tag("o-o-o"), tag("0-0-0")
        ))(input)?;
        Ok((input, "O-O-O".chars().collect()))
    }

    pub fn short_castle(input: &str) -> IResult<&str, Vec<char>> {
        let (input, _) = alt((
            tag("O-O"), tag("o-o"), tag("0-0")
        ))(input)?;
        Ok((input, "O-O".chars().collect()))
    }

    pub fn coup(input: &str) -> IResult<&str, PGNToken> {

        let (input, san_chars) = delimited(
            multispace0,
            alt((
                Self::long_castle, // long castle
                Self::short_castle, // short castle
                many1(one_of(SAN_MOVE_CHARS))
            )),
            multispace0
        )(input)?;
        let san : String = san_chars.iter().collect();
        Ok((input, PGNToken::Coup(san)))
    }

    pub fn annotation(input: &str) -> IResult<&str, PGNToken> {
        let (input, annotation_chars) = delimited(multispace0, many1(one_of(PGN_ANNOTATIONS)), multispace0)(input)?;
        let annotation : String = annotation_chars.iter().collect();
        Ok((input, PGNToken::Annotation(annotation)))
    }

    pub fn comment(input: &str) -> IResult<&str, PGNToken> {
        let (input, comment_chars) = delimited(char('{'), alpha1, char('}'))(input)?;
        let comment : String = comment_chars.chars().collect();
        Ok((input, PGNToken::Comment(comment)))
    }

    pub fn variation_start(input: &str) -> IResult<&str, PGNToken> {
        match char('(')(input) {
            Ok((input, _)) => Ok((input, PGNToken::VariationStart)),
            Err(e) => Err(e)
        }
    }

    pub fn variation_end(input: &str) -> IResult<&str, PGNToken> {
        match char(')')(input) {
            Ok((input, _)) => Ok((input, PGNToken::VariationEnd)),
            Err(e) => Err(e)
        }
    }

    pub fn halt(input: &str) -> IResult<&str, PGNToken> {
        // if we see any of 0-1, 1-0, 1/2-1/2, or * we know it's a halt.
        let (input, is_halt) = opt(alt((
            tag("0-1"),
            tag("1-0"),
            tag("1/2-1/2"),
            tag("*"),
        )))(input)?;
        // chew up any whitespace
        let (input, _) = multispace0(input)?;

        match is_halt {
            Some(r) => {
                let reason = match r {
                    "0-1" => Reason::Winner(Color::BLACK),
                    "1-0" => Reason::Winner(Color::WHITE),
                    "1/2-1/2" => Reason::Stalemate,
                    _ => Reason::Aborted
                };
                Ok((input, PGNToken::Halt(reason)))
            },
            None => Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag))),
        }
    }

    pub fn tag_pair(input: &str) -> IResult<&str, PGNToken> {
        let (input, tag_pair) = TagPair::parse(input)?;
        Ok((input, PGNToken::TagPair(tag_pair)))
    }

    pub fn tag_pair_section(input: &str) -> IResult<&str, Vec<PGNToken>> {
        let (input, tag_pairs) = many1(Self::tag_pair)(input)?;
        Ok((input, tag_pairs))
    }

    pub fn variation_section(input: &str) -> IResult<&str, Vec<PGNToken>> {
        let (input, tokens) = many1(alt((
            // Order matters
            PGNToken::variation_start,
            PGNToken::variation_end,
            PGNToken::turn,
            PGNToken::halt,
            PGNToken::coup,
            PGNToken::annotation,
            PGNToken::comment,
        )))(input)?;
        Ok((input, tokens))
    }

    pub fn tokenize(input: &str) -> IResult<&str, Vec<PGNToken>> {
        let mut tokens = Vec::new();
        tokens.push(PGNToken::GameStart);

        debug!("Processing 1: {}", input);
        let (input, tag_pairs) = Self::tag_pair_section(input)?;
        tokens.extend(tag_pairs);
        debug!("Processing 2: {}", input);

        let (input, _) = newline(input)?;
        debug!("Processing 3: {}", input);

        let (input, toks) = Self::variation_section(input)?;
        tokens.extend(toks);

        if input.is_empty() {
            tokens.push(PGNToken::GameEnd);
        }

        Ok((input, tokens))
    }

    pub fn tokenize_file(path: &str) -> IResult<String, Vec<PGNToken>> {
        let input = std::fs::read_to_string(path).unwrap();
        let (input, tokens) = Self::tokenize(&input).unwrap();
        Ok((input.to_string(), tokens))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    mod tokenizer {
        use super::*;

        mod tokenize {

            use super::*;
            #[test]
            fn imports_from_pgn_with_no_variations_and_halt() {
                let (input, tokens) = PGNToken::tokenize_file("tests/fixtures/no-variations-and-no-halt.pgn").unwrap();
                let expected = [
                    PGNToken::GameStart,
                    PGNToken::TagPair(TagPair { name: "Event".to_string(), value: "No Variations, No Halt".to_string() }),
                    PGNToken::TagPair(TagPair { name: "White".to_string(), value: "white".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Black".to_string(), value: "black".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Result".to_string(), value: "0-1".to_string() }),
                    PGNToken::TagPair(TagPair { name: "CurrentPosition".to_string(), value: "3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - -".to_string() }),
                    PGNToken::TagPair(TagPair { name: "TimeControl".to_string(), value: "900+10".to_string() }),
                    PGNToken::Turn(1), PGNToken::Coup("e4".to_string()), PGNToken::Coup("c6".to_string()),
                    PGNToken::Turn(2), PGNToken::Coup("d4".to_string()), PGNToken::Coup("d5".to_string()),
                    PGNToken::Turn(3), PGNToken::Coup("exd5".to_string()), PGNToken::Coup("cxd5".to_string()),
                    PGNToken::Turn(4), PGNToken::Coup("c4".to_string()), PGNToken::Coup("Nf6".to_string()),
                    PGNToken::Turn(5), PGNToken::Coup("Nc3".to_string()), PGNToken::Coup("Nc6".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Nf3".to_string()), PGNToken::Coup("e6".to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("Be2".to_string()), PGNToken::Coup("Bd6".to_string()),
                    PGNToken::Turn(8), PGNToken::Coup("O-O".to_string()), PGNToken::Coup("dxc4".to_string()),
                    PGNToken::Turn(9), PGNToken::Coup("Bxc4".to_string()), PGNToken::Coup("O-O".to_string()),
                    PGNToken::Turn(10), PGNToken::Coup("b3".to_string()), PGNToken::Coup("a6".to_string()),
                    PGNToken::Turn(11), PGNToken::Coup("Bd3".to_string()), PGNToken::Coup("h6".to_string()),
                    PGNToken::Turn(12), PGNToken::Coup("Bc2".to_string()), PGNToken::Coup("b5".to_string()),
                    PGNToken::Turn(13), PGNToken::Coup("Ne4".to_string()), PGNToken::Coup("Nxe4".to_string()),
                    PGNToken::Turn(14), PGNToken::Coup("Bxe4".to_string()), PGNToken::Coup("Bb7".to_string()),
                    PGNToken::Turn(15), PGNToken::Coup("Bc2".to_string()), PGNToken::Coup("Nb4".to_string()),
                    PGNToken::Turn(16), PGNToken::Coup("Bb1".to_string()), PGNToken::Coup("Qd7".to_string()),
                    PGNToken::Turn(17), PGNToken::Coup("a3".to_string()), PGNToken::Coup("Nd5".to_string()),
                    PGNToken::Turn(18), PGNToken::Coup("Ne5".to_string()), PGNToken::Coup("Qc7".to_string()),
                    PGNToken::Turn(19), PGNToken::Coup("Qd3".to_string()), PGNToken::Coup("f5".to_string()),
                    PGNToken::Turn(20), PGNToken::Coup("Qg3".to_string()), PGNToken::Coup("Qc3".to_string()),
                    PGNToken::Turn(21), PGNToken::Coup("Bd3".to_string()), PGNToken::Coup("Qxd4".to_string()),
                    PGNToken::Turn(22), PGNToken::Coup("Re1".to_string()), PGNToken::Coup("Qxa1".to_string()),
                    PGNToken::Turn(23), PGNToken::Coup("Kf1".to_string()), PGNToken::Coup("Bxe5".to_string()),
                    PGNToken::Turn(24), PGNToken::Coup("Qg6".to_string()), PGNToken::Coup("Nf4".to_string()),
                    PGNToken::Turn(25), PGNToken::Coup("Bxf4".to_string()), PGNToken::Coup("Qd4".to_string()),
                    PGNToken::Turn(26), PGNToken::Coup("Qxe6".to_string()), PGNToken::Annotation("+".to_string()), PGNToken::Coup("Rf7".to_string()),
                    PGNToken::Turn(27), PGNToken::Coup("Bxe5".to_string()), PGNToken::Coup("Qxd3".to_string()), PGNToken::Annotation("+".to_string()),
                    PGNToken::Turn(28), PGNToken::Coup("Re2".to_string()), PGNToken::Coup("Bxg2".to_string()), PGNToken::Annotation("+".to_string()),
                    PGNToken::Turn(29), PGNToken::Coup("Ke1".to_string()), PGNToken::Coup("Rd8".to_string()),
                    PGNToken::GameEnd
                ];

                similar_asserts::assert_eq!(input, "");

                for i in 0..tokens.len() {
                    assert_eq!(tokens[i], expected[i]);
                }


                similar_asserts::assert_eq!(tokens, expected);

            }

            #[test]
            #[tracing_test::traced_test]
            fn imports_from_pgn_with_variations_and_no_halt() {
                let (input, tokens) = PGNToken::tokenize_file("tests/fixtures/with-variations-no-halt.pgn").unwrap();
                let expected = [
                    PGNToken::GameStart,
                    PGNToken::TagPair(TagPair { name: "Event".to_string(), value: "With Variations".to_string() }),
                    PGNToken::TagPair(TagPair { name: "White".to_string(), value: "white".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Black".to_string(), value: "black".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Result".to_string(), value: "0-1".to_string() }),
                    PGNToken::TagPair(TagPair { name: "TimeControl".to_string(), value: "900+10".to_string() }),
                    PGNToken::Turn(1), PGNToken::Coup("d4".to_string()), PGNToken::Coup("d5".to_string()),
                    PGNToken::Turn(2), PGNToken::Coup("Bf4".to_string()), PGNToken::Coup("c6".to_string()),
                    PGNToken::Turn(3), PGNToken::Coup("Nf3".to_string()), PGNToken::Coup("Nf6".to_string()),
                    PGNToken::Turn(4), PGNToken::Coup("e3".to_string()), PGNToken::Coup("Nh5".to_string()),
                    PGNToken::VariationStart, // BVO
                    PGNToken::Turn(4), PGNToken::Coup("Bf5".to_string()),
                    PGNToken::Turn(5), PGNToken::Coup("c4".to_string()), PGNToken::Coup("a5".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Nc3".to_string()), PGNToken::Coup("e6".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(5), PGNToken::Coup("Be5".to_string()), PGNToken::Coup("f6".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Bxb8".to_string()),
                    PGNToken::VariationStart, // WVO
                    PGNToken::Turn(6), PGNToken::Coup("Bg3".to_string()), PGNToken::Coup("Nxg3".to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("hxg3".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(6), PGNToken::Coup("Rxb8".to_string().to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("c4".to_string()), PGNToken::Coup("g6".to_string()),
                    PGNToken::GameEnd
                ];
                assert_eq!(input, "");
                assert_eq!(tokens, expected);
            }

            #[test]
            fn imports_from_pgn_with_variations_and_halt() {
                let (input, tokens) = PGNToken::tokenize_file("tests/fixtures/with-variations-halts.pgn").unwrap();
                let expected = [
                    PGNToken::GameStart,
                    PGNToken::TagPair(TagPair { name: "Event".to_string(), value: "With Variations".to_string() }),
                    PGNToken::TagPair(TagPair { name: "White".to_string(), value: "white".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Black".to_string(), value: "black".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Result".to_string(), value: "0-1".to_string() }),
                    PGNToken::TagPair(TagPair { name: "TimeControl".to_string(), value: "900+10".to_string() }),
                    PGNToken::Turn(1), PGNToken::Coup("d4".to_string()), PGNToken::Coup("d5".to_string()),
                    PGNToken::Turn(2), PGNToken::Coup("Bf4".to_string()), PGNToken::Coup("c6".to_string()),
                    PGNToken::Turn(3), PGNToken::Coup("Nf3".to_string()), PGNToken::Coup("Nf6".to_string()),
                    PGNToken::Turn(4), PGNToken::Coup("e3".to_string()), PGNToken::Coup("Nh5".to_string()),
                    PGNToken::VariationStart, // BVO
                    PGNToken::Turn(4), PGNToken::Coup("Bf5".to_string()),
                    PGNToken::Turn(5), PGNToken::Coup("c4".to_string()), PGNToken::Coup("a5".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Nc3".to_string()), PGNToken::Coup("e6".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(5), PGNToken::Coup("Be5".to_string()), PGNToken::Coup("f6".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Bxb8".to_string()),
                    PGNToken::VariationStart, // WVO
                    PGNToken::Turn(6), PGNToken::Coup("Bg3".to_string()), PGNToken::Coup("Nxg3".to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("hxg3".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(6), PGNToken::Coup("Rxb8".to_string().to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("c4".to_string()), PGNToken::Coup("g6".to_string()),
                    PGNToken::Halt(Reason::Winner(Color::BLACK)),
                    PGNToken::GameEnd
                ];
                assert_eq!(input, "");
                assert_eq!(tokens, expected);
            }

            #[test]
            fn imports_from_pgn_with_nested_variations_and_no_halt() {
                let (input, tokens) = PGNToken::tokenize_file("tests/fixtures/with-nested-variations-no-halt.pgn").unwrap();
                let expected = [
                    PGNToken::GameStart,
                    PGNToken::TagPair(TagPair { name: "Event".to_string(), value: "Nested Variations".to_string() }),
                    PGNToken::TagPair(TagPair { name: "White".to_string(), value: "white".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Black".to_string(), value: "black".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Result".to_string(), value: "0-1".to_string() }),
                    PGNToken::TagPair(TagPair { name: "TimeControl".to_string(), value: "900+10".to_string() }),
                    PGNToken::Turn(1), PGNToken::Coup("d4".to_string()), PGNToken::Coup("d5".to_string()),
                    PGNToken::Turn(2), PGNToken::Coup("Bf4".to_string()), PGNToken::Coup("c6".to_string()),
                    PGNToken::Turn(3), PGNToken::Coup("Nf3".to_string()), PGNToken::Coup("Nf6".to_string()),
                    PGNToken::Turn(4), PGNToken::Coup("e3".to_string()), PGNToken::Coup("Nh5".to_string()),
                    PGNToken::VariationStart, // BVO
                    PGNToken::Turn(4), PGNToken::Coup("Bf5".to_string()),
                    PGNToken::Turn(5), PGNToken::Coup("c4".to_string()), PGNToken::Coup("a5".to_string()),
                    PGNToken::VariationStart, // BVO
                    PGNToken::Turn(5), PGNToken::Coup("e6".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Qb3".to_string()), PGNToken::Coup("b6".to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("Nc3".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(6), PGNToken::Coup("Nc3".to_string()), PGNToken::Coup("e6".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(5), PGNToken::Coup("Be5".to_string()), PGNToken::Coup("f6".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Bxb8".to_string()),
                    PGNToken::VariationStart, // WVO
                    PGNToken::Turn(6), PGNToken::Coup("Bg3".to_string()), PGNToken::Coup("Nxg3".to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("hxg3".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(6), PGNToken::Coup("Rxb8".to_string().to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("c4".to_string()), PGNToken::Coup("g6".to_string()),
                    PGNToken::GameEnd
                ];
                assert_eq!(input, "");
                similar_asserts::assert_eq!(tokens, expected);
            }

            #[test]
            fn imports_from_pgn_with_nested_variations_and_halt() {
                let (input, tokens) = PGNToken::tokenize_file("tests/fixtures/with-nested-variations-halts.pgn").unwrap();
                let expected = [
                    PGNToken::GameStart,
                    PGNToken::TagPair(TagPair { name: "Event".to_string(), value: "Nested Variations".to_string() }),
                    PGNToken::TagPair(TagPair { name: "White".to_string(), value: "white".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Black".to_string(), value: "black".to_string() }),
                    PGNToken::TagPair(TagPair { name: "Result".to_string(), value: "0-1".to_string() }),
                    PGNToken::TagPair(TagPair { name: "TimeControl".to_string(), value: "900+10".to_string() }),
                    PGNToken::Turn(1), PGNToken::Coup("d4".to_string()), PGNToken::Coup("d5".to_string()),
                    PGNToken::Turn(2), PGNToken::Coup("Bf4".to_string()), PGNToken::Coup("c6".to_string()),
                    PGNToken::Turn(3), PGNToken::Coup("Nf3".to_string()), PGNToken::Coup("Nf6".to_string()),
                    PGNToken::Turn(4), PGNToken::Coup("e3".to_string()), PGNToken::Coup("Nh5".to_string()),
                    PGNToken::VariationStart, // BVO
                    PGNToken::Turn(4), PGNToken::Coup("Bf5".to_string()),
                    PGNToken::Turn(5), PGNToken::Coup("c4".to_string()), PGNToken::Coup("a5".to_string()),
                    PGNToken::VariationStart, // BVO
                    PGNToken::Turn(5), PGNToken::Coup("e6".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Qb3".to_string()), PGNToken::Coup("b6".to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("Nc3".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(6), PGNToken::Coup("Nc3".to_string()), PGNToken::Coup("e6".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(5), PGNToken::Coup("Be5".to_string()), PGNToken::Coup("f6".to_string()),
                    PGNToken::Turn(6), PGNToken::Coup("Bxb8".to_string()),
                    PGNToken::VariationStart, // WVO
                    PGNToken::Turn(6), PGNToken::Coup("Bg3".to_string()), PGNToken::Coup("Nxg3".to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("hxg3".to_string()),
                    PGNToken::VariationEnd,
                    PGNToken::Turn(6), PGNToken::Coup("Rxb8".to_string().to_string()),
                    PGNToken::Turn(7), PGNToken::Coup("c4".to_string()), PGNToken::Coup("g6".to_string()),
                    PGNToken::Halt(Reason::Winner(Color::BLACK)),
                    PGNToken::GameEnd
                ];
                assert_eq!(input, "");
                similar_asserts::assert_eq!(tokens, expected);
            }
        }

        #[test]
        fn imports_from_pgn_with_no_variations_and_halts() {
            let (input, tokens) = PGNToken::tokenize_file("tests/fixtures/no-variations-and-halts.pgn").unwrap();

            let expected = [
                PGNToken::GameStart,
                PGNToken::TagPair(TagPair { name: "Event".to_string(), value: "No Variations, Includes Halt".to_string() }),
                PGNToken::TagPair(TagPair { name: "White".to_string(), value: "white".to_string() }),
                PGNToken::TagPair(TagPair { name: "Black".to_string(), value: "black".to_string() }),
                PGNToken::TagPair(TagPair { name: "Result".to_string(), value: "0-1".to_string() }),
                PGNToken::TagPair(TagPair { name: "CurrentPosition".to_string(), value: "3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - -".to_string() }),
                PGNToken::TagPair(TagPair { name: "TimeControl".to_string(), value: "900+10".to_string() }),
                PGNToken::TagPair(TagPair { name: "Termination".to_string(), value: "black won on time".to_string() }),
                PGNToken::Turn(1), PGNToken::Coup("e4".to_string()), PGNToken::Coup("c6".to_string()),
                PGNToken::Turn(2), PGNToken::Coup("d4".to_string()), PGNToken::Coup("d5".to_string()),
                PGNToken::Turn(3), PGNToken::Coup("exd5".to_string()), PGNToken::Coup("cxd5".to_string()),
                PGNToken::Turn(4), PGNToken::Coup("c4".to_string()), PGNToken::Coup("Nf6".to_string()),
                PGNToken::Turn(5), PGNToken::Coup("Nc3".to_string()), PGNToken::Coup("Nc6".to_string()),
                PGNToken::Turn(6), PGNToken::Coup("Nf3".to_string()), PGNToken::Coup("e6".to_string()),
                PGNToken::Turn(7), PGNToken::Coup("Be2".to_string()), PGNToken::Coup("Bd6".to_string()),
                PGNToken::Turn(8), PGNToken::Coup("O-O".to_string()), PGNToken::Coup("dxc4".to_string()),
                PGNToken::Turn(9), PGNToken::Coup("Bxc4".to_string()), PGNToken::Coup("O-O".to_string()),
                PGNToken::Turn(10), PGNToken::Coup("b3".to_string()), PGNToken::Coup("a6".to_string()),
                PGNToken::Turn(11), PGNToken::Coup("Bd3".to_string()), PGNToken::Coup("h6".to_string()),
                PGNToken::Turn(12), PGNToken::Coup("Bc2".to_string()), PGNToken::Coup("b5".to_string()),
                PGNToken::Turn(13), PGNToken::Coup("Ne4".to_string()), PGNToken::Coup("Nxe4".to_string()),
                PGNToken::Turn(14), PGNToken::Coup("Bxe4".to_string()), PGNToken::Coup("Bb7".to_string()),
                PGNToken::Turn(15), PGNToken::Coup("Bc2".to_string()), PGNToken::Coup("Nb4".to_string()),
                PGNToken::Turn(16), PGNToken::Coup("Bb1".to_string()), PGNToken::Coup("Qd7".to_string()),
                PGNToken::Turn(17), PGNToken::Coup("a3".to_string()), PGNToken::Coup("Nd5".to_string()),
                PGNToken::Turn(18), PGNToken::Coup("Ne5".to_string()), PGNToken::Coup("Qc7".to_string()),
                PGNToken::Turn(19), PGNToken::Coup("Qd3".to_string()), PGNToken::Coup("f5".to_string()),
                PGNToken::Turn(20), PGNToken::Coup("Qg3".to_string()), PGNToken::Coup("Qc3".to_string()),
                PGNToken::Turn(21), PGNToken::Coup("Bd3".to_string()), PGNToken::Coup("Qxd4".to_string()),
                PGNToken::Turn(22), PGNToken::Coup("Re1".to_string()), PGNToken::Coup("Qxa1".to_string()),
                PGNToken::Turn(23), PGNToken::Coup("Kf1".to_string()), PGNToken::Coup("Bxe5".to_string()),
                PGNToken::Turn(24), PGNToken::Coup("Qg6".to_string()), PGNToken::Coup("Nf4".to_string()),
                PGNToken::Turn(25), PGNToken::Coup("Bxf4".to_string()), PGNToken::Coup("Qd4".to_string()),
                PGNToken::Turn(26), PGNToken::Coup("Qxe6".to_string()), PGNToken::Annotation("+".to_string()), PGNToken::Coup("Rf7".to_string()),
                PGNToken::Turn(27), PGNToken::Coup("Bxe5".to_string()), PGNToken::Coup("Qxd3".to_string()), PGNToken::Annotation("+".to_string()),
                PGNToken::Turn(28), PGNToken::Coup("Re2".to_string()), PGNToken::Coup("Bxg2".to_string()), PGNToken::Annotation("+".to_string()),
                PGNToken::Turn(29), PGNToken::Coup("Ke1".to_string()), PGNToken::Coup("Rd8".to_string()),
                PGNToken::Halt(Reason::Winner(Color::BLACK)),
                PGNToken::GameEnd
            ];

            similar_asserts::assert_eq!(input, "");
            similar_asserts::assert_eq!(tokens, expected);
        }
    }

    mod turn {
        use super::*;

        #[test]
        #[tracing_test::traced_test]
        fn parses_turn() {
            let (input, token) = PGNToken::turn("1. ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Turn(1));

            let (input, token) = PGNToken::turn("1... ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Turn(1));
        }
    }

    mod coup {
        use super::*;

        #[test]
        fn tokenizes_annotated_move_correctly() {
            let m = "1. Qxe6+ ";
            let (input, tokens) = PGNToken::variation_section(m).unwrap();

            let expected = [
                PGNToken::Turn(1),
                PGNToken::Coup("Qxe6".to_string()),
                PGNToken::Annotation("+".to_string())
            ];

            assert_eq!(input, "");
            similar_asserts::assert_eq!(tokens, expected);

        }
    }

    mod annotation {
        use super::*;

        #[test]
        fn parses_annotation() {
            let (input, token) = PGNToken::annotation("! ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Annotation("!".to_string()));

            let (input, token) = PGNToken::annotation("? ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Annotation("?".to_string()));

            let (input, token) = PGNToken::annotation("+ ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Annotation("+".to_string()));

            let (input, token) = PGNToken::annotation("- ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Annotation("-".to_string()));

            let (input, token) = PGNToken::annotation(". ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Annotation(".".to_string()));
        }
    }

    mod halt {
        use super::*;

        #[test]
        #[tracing_test::traced_test]
        fn parses_halt() {
            let (input, token) = PGNToken::halt("1-0 ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Halt(Reason::Winner(Color::WHITE)));;

            let (input, token) = PGNToken::halt("0-1 ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Halt(Reason::Winner(Color::BLACK)));

            let (input, token) = PGNToken::halt("1/2-1/2 ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Halt(Reason::Stalemate));

            let (input, token) = PGNToken::halt("* ").unwrap();
            assert_eq!(input, "");
            assert_eq!(token, PGNToken::Halt(Reason::Aborted));
        }
    }

}
