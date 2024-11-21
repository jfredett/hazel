use std::path::Path;
use std::io::Error;

use nom::{branch::alt, bytes::complete::tag, character::complete::{multispace0, multispace1, newline, one_of}, combinator::opt, multi::{many0, many1}, sequence::delimited, IResult};
use tracing::debug;

use crate::{board::Alter, constants::START_POSITION_FEN, coup::rep::Move, game::variation::Variation, notation::fen::FEN};

use super::{san::SANConversionError, Square};

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

use crate::{notation::{ben::BEN, san::SAN}, types::Color};

#[derive(Debug, Clone)]
struct HalfPly {
    color: Color,
    san: SAN,
    annotations: Vec<char>,
}

#[derive(Debug, Clone)]
struct Ply {
    number: usize,
    white: HalfPly,
    black: Option<HalfPly>,
}

impl HalfPly {

    pub fn parse<'a>(input: &'a str, color: Color, context: BEN) -> IResult<&'a str, Self> {
        // We don't actually care about the ply number.
        let (input, _) = multispace0(input)?;
        let (input, san) = SAN::parse(input, context)?;
        let (input, annotations) = many0(one_of(PGN_ANNOTATIONS))(input)?;
        let (input, _) = multispace0(input)?;

        Ok((input, HalfPly {
            color,
            san,
            annotations
        }))
    }

}

const PGN_ANNOTATIONS: &str = "!?+-.#";

impl Ply {
    /// Given a PGN ply notation like:
    ///
    /// XX. YY ZZ
    ///
    /// This returns a result of ("YY ZZ", XX as usize)
    ///
    /// Note that it consumes _all available whitespace_ after the ply number, and optionally will
    /// read the period after the ply number if it is present.
    pub fn ply_number(input: &str) -> IResult<&str, usize> {
        let (input, half_ply_num) = delimited(multispace0, many1(one_of("1234567890")), alt((tag("."), multispace1)))(input)?;

        // remove any excess whitespace because I'm not lexing.
        let (input, _) = multispace0(input)?;
        let half_ply = half_ply_num.iter().collect::<String>().parse::<usize>().unwrap();
        Ok((input, half_ply))
    }

    pub fn parse(input: &str, context: impl Into<BEN>) -> IResult<&str, Ply> {
        let mut ctx : BEN = context.into();
        let (input, number) = Self::ply_number(input)?;
        let (input, white) = HalfPly::parse(input, Color::WHITE, ctx)?;

        // Update the context before parsing the next move
        // TODO make this a better error
        let m : Result<Move, SANConversionError> = white.clone().san.try_into();
        match m {
            Ok(mov) => {
                // HACK: This all kind of sucks

                // Order matters, calculate the metadata from the current state
                let mut meta = ctx.metadata();
                meta.update(&mov, &ctx);
                ctx.set_metadata(meta);
                // then update the board to the new state to match the metadata
                for alter in mov.compile(&ctx) {
                    ctx.alter_mut(alter);
                }
            },
            Err(e) => {
                debug!("Error parsing SAN: {:?}", e);
                return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
            }
        };

        let (input, black) = opt(|input| HalfPly::parse(input, Color::BLACK, ctx))(input)?;
        let (input, _) = multispace0(input)?;

        Ok((input, Ply {
            number,
            white,
            black,
        }))
    }
}


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
        pgn.variation.setup(FEN::new(START_POSITION_FEN));
        pgn.variation.commit();


        let (mut input, tag_pairs) = TagPairs::parse(pgn_data)?;
        debug!("{}", input);
        pgn.tag_pairs = tag_pairs;

        loop {
            debug!("input: {}", input);
            let current_context : BEN = pgn.variation.current_position().into();
            #[allow(unused_variables)] // NOTE: This is not actually unused, but clippy thinks it is.
            let (new_input, maybe_ply) = opt(|input| Ply::parse(input, current_context))(input)?;
            input = new_input;
            if let Some(ply) = maybe_ply {
                let wmov : Move = ply.white.san.clone().try_into().unwrap();
                debug!("Parsed white move: {:?}", wmov);
                pgn.variation.make(ply.white.san.try_into().unwrap());
                pgn.variation.commit();
                if let Some(black) = ply.black {
                    let bmov : Move = black.san.clone().try_into().unwrap();
                    debug!("Parsed black move: {:?}", bmov);
                    pgn.variation.make(black.san.try_into().unwrap());
                    pgn.variation.commit();
                }
            } else {
                break;
            }
        }

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
            let pgn = PGN::load("tests/fixtures/no-variations-and-halts.pgn").unwrap();

            assert_eq!(pgn.variation.current_position(), FEN::new("3r2k1/5rp1/p3Q2p/1p2Bp2/8/PP1q4/4RPbP/4K3 w - - 2 30"));
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

    mod ply {
        use super::*;

        mod halfply {
            use super::*;

        }

        mod number {
            use crate::types::Color;

            use super::*;

            #[test]
            fn parses_ply_marker() {
                let hp = "1. d4 d5";
                let (input, ply) = Ply::ply_number(hp).unwrap();

                assert_eq!(input, "d4 d5");
                assert_eq!(ply, 1);
            }

            #[test]
            fn parses_ply_marker_with_multiple_digits() {
                let hp = "30. d4 d5";
                let (input, ply) = Ply::ply_number(hp).unwrap();

                assert_eq!(input, "d4 d5");
                assert_eq!(ply, 30);
            }

            #[test]
            fn parses_ply_missing_period() {
                let hp = "30 d4 d5";
                let (input, ply) = Ply::ply_number(hp).unwrap();

                assert_eq!(input, "d4 d5");
                assert_eq!(ply, 30);
            }

            #[quickcheck]
            fn parses_ply_numbers_correctly(ply_number: usize, full_move: bool, include_period: bool) -> bool {
                if ply_number == 0 { return true; }
                let period = if include_period { "." } else { "" };

                let hp = if full_move {
                    format!("{}{} x4 y8", ply_number, period)
                } else {
                    format!("{}{} z1", ply_number, period)
                };

                let (input, parsed_ply) = Ply::ply_number(&hp).unwrap();

                let input_check = if full_move { input == "x4 y8" } else { input == "z1" };
                input_check && (ply_number == parsed_ply)
            }
        }
    }
}
