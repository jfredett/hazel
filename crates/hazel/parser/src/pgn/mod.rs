#![allow(dead_code, unused_imports)]

mod tag_pair;
pub mod parsers;
mod tokenizer;

use std::path::Path;
use std::io::Error;

use hazel_core::ben::BEN;
use nom::{branch::alt, bytes::complete::tag, character::complete::{multispace0, multispace1, newline, one_of}, combinator::opt, multi::{many0, many1}, sequence::delimited, IResult};

use hazel_representation::game::familiar::Familiar;
use hazel_representation::{coup::rep::Move, game::variation::Variation};
use crate::san::SAN;
use crate::pgn::tokenizer::PGNToken;

use crate::san::SANConversionError;
use hazel_core::square::Square;


use tag_pair::*;

#[derive(Default, Debug)]
#[derive(Clone)]
pub struct PGN {
    tag_pairs: Vec<TagPair>,
    variation: Variation,
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

    pub fn tags(&self) -> &Vec<TagPair> {
        &self.tag_pairs
    }

    pub fn familiar(&mut self) -> Familiar {
        self.variation.familiar()
    }

    pub fn current_position(&self) -> BEN {
        let mut v = self.variation.clone();
        v.current_position()
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut pgn = PGN::default();

        let (input, tokens) = PGNToken::tokenize(input)?;

        let mut variation = Variation::default();
        for token in tokens {
            match token {
                PGNToken::GameStart => {
                    variation.new_game()
                             .commit();
                },
                PGNToken::TagPair(tp) => {
                    pgn.tag_pairs.push(tp);
                },
                PGNToken::VariationStart => {
                    variation.start_variation().commit();
                },
                PGNToken::VariationEnd => {
                    variation.end_variation().commit();
                },
                PGNToken::Turn(_) => { }
                PGNToken::Coup(san_str) => {
                    let (input, san) = SAN::parse(&san_str, variation.current_position()).unwrap();

                    assert_eq!(input, "");

                    variation.make(san.try_into().unwrap()).commit();
                },
                PGNToken::Halt(reason) => {
                    variation.halt(reason).commit();
                },
                PGNToken::GameEnd => {
                    pgn.variation = variation.clone();
                },
                _ => {
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

    mod pgn {
        use super::*;

        #[test]
        fn imports_from_pgn_with_no_variations_and_halts() {
            let pgn = PGN::load("../../../tests/fixtures/no-variations-and-halts.pgn").unwrap();

            similar_asserts::assert_eq!(pgn.current_position(), BEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
        }

        #[test]
        fn imports_from_pgn_with_no_variations_and_halt() {
            let pgn = PGN::load("../../../tests/fixtures/no-variations-and-no-halt.pgn").unwrap();

            assert_eq!(pgn.current_position(), BEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
        }

        #[test]
        fn imports_from_pgn_with_variations_and_no_halt() {
            let pgn = PGN::load("../../../tests/fixtures/with-variations-no-halt.pgn").unwrap();

            assert_eq!(pgn.current_position(), BEN::new("1rbqkb1r/pp2p2p/2p2pp1/3p3n/2PP4/4PN2/PP3PPP/RN1QKB1R w KQk - 0 8"));
        }

        #[test]
        fn imports_from_pgn_with_variations_and_halt() {
            let pgn = PGN::load("../../../tests/fixtures/with-variations-halts.pgn").unwrap();

            assert_eq!(pgn.current_position(), BEN::new("1rbqkb1r/pp2p2p/2p2pp1/3p3n/2PP4/4PN2/PP3PPP/RN1QKB1R w KQk - 0 8"));
        }

        #[test]
        fn imports_from_pgn_with_nested_variations_and_no_halt() {
            let pgn = PGN::load("../../../tests/fixtures/with-nested-variations-no-halt.pgn").unwrap();

            assert_eq!(pgn.current_position(), BEN::new("1rbqkb1r/pp2p2p/2p2pp1/3p3n/2PP4/4PN2/PP3PPP/RN1QKB1R w KQk - 0 8"));
        }

        #[test]
        fn imports_from_pgn_with_nested_variations_and_halt() {
            let pgn = PGN::load("../../../tests/fixtures/with-nested-variations-halts.pgn").unwrap();

            assert_eq!(pgn.current_position(), BEN::new("1rbqkb1r/pp2p2p/2p2pp1/3p3n/2PP4/4PN2/PP3PPP/RN1QKB1R w KQk - 0 8"));
        }
    }
}
