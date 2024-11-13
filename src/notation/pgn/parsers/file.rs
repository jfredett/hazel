use nom::character::complete::one_of;

use crate::constants::File;

use super::*;

impl File {
    pub fn parse(input: &str) -> IResult<&str, File> {
        let (input, file) = one_of("abcdefgh")(input)?;
        Ok((input, File::from(file)))
    }
}
