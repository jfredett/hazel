use nom::{branch::alt, bytes::complete::tag, character::complete::{multispace0, multispace1, one_of}, combinator::opt, multi::many1, sequence::delimited, IResult};
use tracing::debug;

use crate::{board::Alter, coup::rep::Move, notation::{ben::BEN, san::SANConversionError}, types::Color};

use super::HalfPly;

#[derive(Debug, Clone)]
pub struct Ply {
    number: usize,
    white: HalfPly,
    black: Option<HalfPly>,
}


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

    pub fn white(&self) -> &HalfPly {
        &self.white
    }

    pub fn black(&self) -> Option<&HalfPly> {
        self.black.as_ref()
    }

    pub fn parse(input: &str, context: impl Into<BEN>) -> IResult<&str, Ply> {
        let mut ctx : BEN = context.into();
        // TODO: Cover the (X. WM BM) case
        // TODO: Cover the (X... BM) case
        let (input, number) = Self::ply_number(input)?;
        let (input, white) = HalfPly::parse(input, Color::WHITE, ctx)?;

        // Update the context before parsing the next move
        // TODO make this a better error
        let m : Result<Move, SANConversionError> = white.san().try_into();
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

        debug!("Remaining input after parsing ply: {}", input);
        debug!("Next Ply Contents: {:?}", (number, white.clone(), black.clone()));

        Ok((input, Ply {
            number,
            white,
            black,
        }))
    }
}
