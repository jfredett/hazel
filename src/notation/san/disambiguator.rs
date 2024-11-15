use crate::constants::File;

use super::*;

#[derive(Debug, Clone, Copy)]
pub(super) enum Disambiguator {
    File(File),
    Rank(usize),
    Sq(Square)
}

impl Disambiguator {
    pub fn parse(input: &str) -> IResult<&str, Disambiguator> {
        let (input, sq) = opt(Square::parse)(input)?;
        if let Some(sq) = sq {
            return Ok((input, Disambiguator::Sq(sq)));
        }

        let (input, file) = opt(File::parse)(input)?;
        if let Some(file) = file {
            return Ok((input, Disambiguator::File(file)));
        }

        let (input, rank) = opt(Piece::parse_rank)(input)?;
        if let Some(rank) = rank {
            return Ok((input, Disambiguator::Rank(rank)));
        }

        Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))
    }

    pub fn square(&self) -> Square {
        match self {
            Disambiguator::Sq(sq) => *sq,
            _ => panic!("Disambiguator is not a square"),
        }
    }
}

