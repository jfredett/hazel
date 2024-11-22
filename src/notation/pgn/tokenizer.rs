use nom::{branch::alt, bytes::complete::tag, character::complete::{char, multispace0, newline, one_of}, multi::many1, sequence::delimited, IResult};
use tracing::debug;

use super::TagPair;

#[derive(Debug, Clone, PartialEq)]
pub enum PGNToken {
    GameStart,
    GameEnd,
    TagPair(TagPair),
    Turn(usize), // covers both white-starts and black-starts turns.
    Coup(String),
    VariationStart,
    VariationEnd,
    Halt
}

impl PGNToken {

    pub fn turn(input: &str) -> IResult<&str, PGNToken> {
        let (input, number_chars) = delimited(
            multispace0,
            many1(one_of("1234567890")),
            alt((
                tag("."),
                tag("..."),
            ))
        )(input)?;
        let number_str = number_chars.iter().collect::<String>();
        let number = number_str.parse::<usize>().unwrap();

        Ok((input, PGNToken::Turn(number)))
    }

    pub fn coup(input: &str) -> IResult<&str, PGNToken> {
        let (input, san_chars) = delimited(multispace0, many1(one_of("abcdefghRNBQKoO123456780-")), multispace0)(input)?;
        let san : String = san_chars.iter().collect();
        Ok((input, PGNToken::Coup(san)))
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
        let this = alt((
            tag("1-0"),
            tag("0-1"),
            tag("1/2-1/2"),
            tag("*")
        ))(input);

        match this {
            Ok(_) => Ok((input, PGNToken::Halt)),
            Err(e) => Err(e),
        }
    }

    pub fn tag_pair(input: &str) -> IResult<&str, PGNToken> {
        let (input, tag_pair) = TagPair::parse(input)?;
        Ok((input, PGNToken::TagPair(tag_pair)))
    }

    pub fn tokenize(input: &str) -> IResult<&str, Vec<PGNToken>> {
        let mut tokens = Vec::new();
        let mut last_input = "";

            debug!("Processing 1: {}", input);
        let (input, tag_pairs) = many1(Self::tag_pair)(input)?;
        tokens.extend(tag_pairs);
            debug!("Processing 2: {}", input);


        let (input, _) = newline(input)?;
        let (input, _) = newline(input)?;
        debug!("Processing 3: {}", input);

        loop {
            debug!("Processing 4: {}", input);

            let (input, token) = alt((
                PGNToken::turn,
                PGNToken::coup,
                PGNToken::variation_start,
                PGNToken::variation_end,
                PGNToken::halt
            ))(input)?;

            tokens.push(token);

            if input == last_input { break; }
            last_input = input.clone()
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
            #[tracing_test::traced_test]
            fn imports_from_pgn_with_no_variations_and_halts() {
                let (input, tokens) = PGNToken::tokenize_file("tests/fixtures/no-variations-and-halts.pgn").unwrap();
                assert_eq!(input, "");

                tracing::debug!("tokens: {:?}", tokens);

                // FIXME This is deliberately wrong.
                assert_eq!(tokens.len(), 1);

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

        #[test]
        fn imports_from_pgn_with_no_variations_and_halt() {
 //           let pgn = PGN::load("tests/fixtures/no-variations-and-no-halt.pgn").unwrap();

        }

        /*
        #[test]
        #[tracing_test::traced_test]
        fn imports_from_pgn_with_variations_and_no_halt() {
            let pgn = PGN::load("tests/fixtures/with-variations-no-halt.pgn").unwrap();
        }

        #[test]
        fn imports_from_pgn_with_variations_and_halt() {
            let pgn = PGN::load("tests/fixtures/with-variations-halts.pgn").unwrap();
        }

        #[test]
        fn imports_from_pgn_with_nested_variations_and_no_halt() {
            let pgn = PGN::load("tests/fixtures/with-nested-variations-no-halt.pgn").unwrap();
        }

        #[test]
        fn imports_from_pgn_with_nested_variations_and_halt() {
            let pgn = PGN::load("tests/fixtures/with-nested-variations-halts.pgn").unwrap();
        }
        */
    }
    
}
