use std::path::Path;
use std::io::Error;

use nom::{character::complete::newline, IResult};
use tracing::debug;

use crate::game::variation::Variation;

use super::Square;

mod tag_pair;
mod parsers;

use tag_pair::*;

#[derive(Debug)]
pub struct PGN {
    tag_pairs: TagPairs,
    variation: Variation,
}

/*
impl<S : Into<String>> TryFrom<S> for PGN {
type Error = Error;

fn try_from(pgn: S) -> Result<Self, Self::Error> {
todo!();
}
}
*/

impl Default for PGN {
    fn default() -> Self {
        PGN {
            tag_pairs: TagPairs::default(),
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

    pub fn parse(pgn_data: &str) -> IResult<&str, Self> {
        let mut pgn = PGN::default();

        debug!("{}", pgn_data);
        let (input, tag_pairs) = TagPairs::parse(pgn_data)?;
        debug!("{}", input);
        pgn.tag_pairs = tag_pairs;
        debug!("pgn {:?}", pgn);

        let (input, _) = newline(input)?;
        debug!("{}", input);

        let (input, variation) = Variation::parse(input)?;
        pgn.variation = variation.clone();

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
    fn imports_from_pgn_with_no_variations_and_halts() {
    let pgn = PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap();
    }

    #[test]
    fn imports_from_pgn_with_no_variations_and_halt() {
    let pgn = PGN::load("tests/fixtures/no-variations-and-no-halt.pgn").unwrap();
    }


    #[test]
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
    }
    */
}
