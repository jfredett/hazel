use nom::{character::complete::{multispace0, one_of}, multi::many0, IResult};

use crate::{notation::{ben::BEN, san::SAN}, types::Color};

use super::PGN_ANNOTATIONS;


#[derive(Debug, Clone)]
pub struct HalfPly {
    // starts variation
    // ends varaition

    color: Color,
    san: SAN,
    annotations: Vec<char>,
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

    pub fn san(&self) -> SAN {
        self.san.clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
}
