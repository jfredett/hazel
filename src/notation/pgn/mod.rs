#![allow(dead_code, unused_imports)]

mod tag_pair;
mod parsers;
mod tokenizer;

use std::path::Path;
use std::io::Error;

use nom::{branch::alt, bytes::complete::tag, character::complete::{multispace0, multispace1, newline, one_of}, combinator::opt, multi::{many0, many1}, sequence::delimited, IResult};
use tracing::debug;

use crate::notation::pgn::tokenizer::PGNToken;
use crate::{board::Alter, constants::START_POSITION_FEN, coup::rep::Move, game::variation::Variation, notation::fen::FEN};
use crate::{notation::{ben::BEN, san::SAN}, types::Color};

use super::{san::SANConversionError, Square};

use tag_pair::*;

#[derive(Debug)]
pub struct PGN {
    tag_pairs: Vec<TagPair>,
    variation: Variation,
}


impl Default for PGN {
    fn default() -> Self {
        PGN {
            tag_pairs: vec![],
            variation: Variation::new(),
        }
    }
}

impl PGN {
    /// Reads a PGN file and returns a PGN struct
    pub fn load<P : AsRef<Path>>(path: P) -> Result<Self, Error> {
        let pgn_data = std::fs::read_to_string(path)?;
        let pgn = match PGN::parse(&pgn_data) {
            Ok((_, pgn)) => pgn,
            Err(e) => {
                eprintln!("Error parsing PGN: {:?}", e);
                return Err(Error::from_raw_os_error(0xFF));
            }
        };

        Ok(pgn)
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut pgn = PGN::default();

        let (input, tokens) = PGNToken::tokenize(input)?;

        let mut variation = Variation::new();
        for token in tokens {
            match token {
                PGNToken::GameStart => {
                    debug!("Game start");
                    variation.new_game()
                             .setup(FEN::new(START_POSITION_FEN))
                             .commit();
                },
                PGNToken::TagPair(tp) => {
                    debug!("Tag pair");
                    pgn.tag_pairs.push(tp);
                },
                PGNToken::VariationStart => {
                    debug!("Variation start");
                    variation.start_variation().commit();
                },
                PGNToken::VariationEnd => {
                    debug!("Variation end");
                    variation.end_variation().commit();
                },
                PGNToken::Turn(_) => { }
                PGNToken::Coup(san_str) => {
                    debug!("Coup: {:?}", san_str);
                    let current_position = variation.current_position();
                    let (input, san) = SAN::parse(&san_str, current_position).unwrap();

                    assert_eq!(input, "");

                    variation.make(san.try_into().unwrap()).commit();
                },
                _ => {
                    debug!("Unhandled token: {:?}", token);
                }
            }
        }

        // PGNs are terminated by two newlines. This is distinct from if they _halt_ or not, which
        // is a gamestate thing, not a PGN thing.

        Ok((input, pgn))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    mod pgn {
        use super::*;

        #[test]
        #[tracing_test::traced_test]
        fn imports_from_pgn_with_no_variations_and_halts() {
            let pgn = PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap();

            similar_asserts::assert_eq!(pgn.variation.current_position(), FEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
        }

        #[test]
        fn imports_from_pgn_with_no_variations_and_halt() {
            let pgn = PGN::load("tests/fixtures/no-variations-and-no-halt.pgn").unwrap();

            //FIXME: This is the wrong FEN.
            assert_eq!(pgn.variation.current_position(), FEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
        }

        #[test]
        #[tracing_test::traced_test]
        fn imports_from_pgn_with_variations_and_no_halt() {
            let pgn = PGN::load("tests/fixtures/with-variations-no-halt.pgn").unwrap();

            //FIXME: This is the wrong FEN.
            assert_eq!(pgn.variation.current_position(), FEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
        }

        #[test]
        fn imports_from_pgn_with_variations_and_halt() {
            let pgn = PGN::load("tests/fixtures/with-variations-halts.pgn").unwrap();

            //FIXME: This is the wrong FEN.
            assert_eq!(pgn.variation.current_position(), FEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
        }

        #[test]
        fn imports_from_pgn_with_nested_variations_and_no_halt() {
            let pgn = PGN::load("tests/fixtures/with-nested-variations-no-halt.pgn").unwrap();

            //FIXME: This is the wrong FEN.
            assert_eq!(pgn.variation.current_position(), FEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
        }

        #[test]
        fn imports_from_pgn_with_nested_variations_and_halt() {
            let pgn = PGN::load("tests/fixtures/with-nested-variations-halts.pgn").unwrap();

            //FIXME: This is the wrong FEN.
            assert_eq!(pgn.variation.current_position(), FEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
        }
    }
    */
}
