use nom::character::complete::one_of;

use crate::types::Piece;

use super::*;

impl Piece {
    pub fn parse_rank(input: &str) -> IResult<&str, usize> {
        let (input, rank_data) = one_of("12345678")(input)?;
        Ok((input, (rank_data.to_digit(10).unwrap() - 1u32) as usize))
    }

    pub fn parse(input: &str) -> IResult<&str, Piece> {
        let (input, piece) = one_of("KQRBNP")(input)?;
        match piece {
            'K' => Ok((input, Piece::King)),
            'Q' => Ok((input, Piece::Queen)),
            'R' => Ok((input, Piece::Rook)),
            'B' => Ok((input, Piece::Bishop)),
            'N' => Ok((input, Piece::Knight)),
            'P' => Ok((input, Piece::Pawn)),
            _ => panic!("Invalid piece"),
        }
    }
}


