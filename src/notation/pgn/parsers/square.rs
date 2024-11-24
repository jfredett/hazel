use nom::character::complete::one_of;

use crate::constants::File;

use super::*;

impl Square {
    pub fn parse(input: &str) -> IResult<&str, Square> {
        let (input, file) = File::parse(input)?;
        let (input, rank_data) = one_of("12345678")(input)?;
        let mut sq = Square::default();
        sq.set_file(file as usize);
        sq.set_rank((rank_data.to_digit(10).unwrap() - 1u32) as usize);

        Ok((input, sq))
    }
}
