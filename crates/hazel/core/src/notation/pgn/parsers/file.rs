use hazel_basic::file::File;
use nom::character::complete::one_of;

use super::*;

// TODO: Move this to extension section
pub trait FileParsing {
    fn parse(input: &str) -> IResult<&str, File>;
}

impl FileParsing for File {
    fn parse(input: &str) -> IResult<&str, File> {
        let (input, file) = one_of("abcdefgh")(input)?;
        Ok((input, File::from(file)))
    }
}
