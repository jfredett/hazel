use super::*;
use std::fmt::{Debug, Display, Formatter, Result};

impl Debug for Ply {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let buf = self.board_buffer();
        // We need to start on a8 and work _down_ to h1, left to right.
        writeln!(f)?;
        for rank in 0..8 {
            write!(f, "{} |", 8 - rank)?;
            for file in FILES {
                write!(f, " {}", buf[7 - rank][file as usize])?;
            }
            writeln!(f)?;
        }
        writeln!(f, "    a b c d e f g h")?;
        if self.current_player() == Color::BLACK {
            writeln!(f, "    Black to play")
        } else {
            writeln!(f, "    White to play")
        }
    }
}

impl Display for Ply {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn starting_position_displays_as_expected() {
        let ply = start_position();
        let res = format!("{:?}", ply);
        let expected = "
8 | r n b q k b n r
7 | p p p p p p p p
6 | . . . . . . . .
5 | . . . . . . . .
4 | . . . . . . . .
3 | . . . . . . . .
2 | P P P P P P P P
1 | R N B Q K B N R
    a b c d e f g h
    White to play
";

        assert_eq!(res, expected);
    }
}
