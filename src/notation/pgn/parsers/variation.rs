use nom::{branch::alt, bytes::complete::tag, character::complete::{multispace0, multispace1, one_of}, multi::many1, sequence::delimited};

use crate::notation::san::SAN;

use super::*;

impl Variation {
    /// Given a PGN ply notation like:
    ///
    /// XX. YY ZZ
    ///
    /// This returns a result of ("YY ZZ", XX as usize)
    ///
    /// Note that it consumes _all available whitespace_ after the ply number, and optionally will
    /// read the period after the ply number if it is present.
    pub fn half_ply_number(input: &str) -> IResult<&str, usize> {
        let (input, half_ply_num) = delimited(multispace0, many1(one_of("1234567890")), alt((tag("."), multispace1)))(input)?;

        // remove any excess whitespace because I'm not lexing.
        let (input, _) = multispace0(input)?;
        let half_ply = half_ply_num.iter().collect::<String>().parse::<usize>().unwrap();
        Ok((input, half_ply))
    }

    pub fn white_half_ply<'a>(&mut self, input: &'a str) -> IResult<&'a str, SAN> {
        // We don't actually care about the ply number.                         // _1. d4 d5 2. ...
        let (input, _ply_number) = Self::half_ply_number(input)?;               // _d4 d5 2. ...
        let pos = self.current_position();
        SAN::parse(input, pos.clone())                                                       // _d5 2. ...
    }

    pub fn parse_into<'a>(&'a mut self, input: &'a str) -> IResult<&'a str, ()> {

        // We don't actually care about the ply number.                         // _1. d4 d5 2. ...
        let (input, white_halfply) = self.white_half_ply(input)?;        // _d5 2. ...

        self.make(white_halfply.try_into().unwrap());
        debug!("{:?}", self);

        /*
        // if there is one... then.
        let (input, _) = take_while(|c| c == ' ')(input)?;                      // _d5 2.
        let (input, black_halfply) = take_while(not(|c| c.is_digit()))(input)?; // _2.
        let black_halfply = SAN::parse(black_halfply, &v);
        */

        Ok((input, ()))
    }


    pub fn parse(input: &str) -> IResult<&str, Variation> {
        let mut v = Variation::new();
        let _ = v.parse_into(input);
        Ok((input, v.clone()))
    }
}



#[cfg(test)]
mod tests {
    use super::*;


    mod halfply_marker {
        use crate::types::Color;

        use super::*;


        #[test]
        fn parses_halfply_marker() {
            let hp = "1. d4 d5";
            let (input, half_ply) = Variation::half_ply_number(hp).unwrap();

            assert_eq!(input, "d4 d5");
            assert_eq!(half_ply, 1);
        }

        #[test]
        fn parses_halfply_marker_with_multiple_digits() {
            let hp = "30. d4 d5";
            let (input, half_ply) = Variation::half_ply_number(hp).unwrap();

            assert_eq!(input, "d4 d5");
            assert_eq!(half_ply, 30);
        }

        #[test]
        fn parses_halfply_missing_period() {
            let hp = "30 d4 d5";
            let (input, half_ply) = Variation::half_ply_number(hp).unwrap();

            assert_eq!(input, "d4 d5");
            assert_eq!(half_ply, 30);
        }


        #[quickcheck]
        fn parses_halfply_numbers_correctly(half_ply: usize, full_move: bool, include_period: bool) -> bool {
            if half_ply == 0 { return true; }
            let period = if include_period { "." } else { "" };

            let hp = if full_move {
                format!("{}{} x4 y8", half_ply, period)
            } else {
                format!("{}{} z1", half_ply, period)
            };

            let (input, parsed_half_ply) = Variation::half_ply_number(&hp).unwrap();

            let input_check = if full_move { input == "x4 y8" } else { input == "z1" };
            input_check && (half_ply == parsed_half_ply)
        }
    }
}
