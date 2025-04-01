use hazel_core::file::File;
use nom::character::complete::one_of;

use super::*;

pub trait SquareParsing {
    fn parse(input: &str) -> IResult<&str, Square>;
}

impl SquareParsing for Square {
    fn parse(input: &str) -> IResult<&str, Square> {
        let (input, file) = File::parse(input)?;
        let (input, rank_data) = one_of("12345678")(input)?;
        let mut sq = Square::default();
        sq = sq.set_file(file as usize);
        sq = sq.set_rank((rank_data.to_digit(10).unwrap() - 1u32) as usize);

        Ok((input, sq))
    }
}
