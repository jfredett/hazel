use super::*;
use std::fmt::{Formatter, Result, Debug};

use constants::{PIECES, FILES, COLORS, ASCII_PIECE_CHARS};


impl Debug for Ply {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut buf = [['.'; 8]; 8];

        // Encode the board into a 8x8 array of chars.
        for rank in 0..8 {
            for file in FILES {
                for piece in PIECES {
                    for color in COLORS {
                        if self.piece_at(file, rank + 1, piece, color) {
                            buf[file as usize][rank] = ASCII_PIECE_CHARS[color as usize][piece as usize];
                        }
                    }
                }
            }
        }

        // We need to start on a8 and work _down_ to h1, left to right.
        writeln!(f)?;
        for rank in 0..8 {                
            write!(f, "{} |", 8 - rank)?; 
            for file in FILES {                                        
                write!(f, " {}", buf[file as usize][7 - rank])?;
            }
            writeln!(f)?;
        }
        writeln!(f, "    a b c d e f g h")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn starting_position_displays_as_expected() {
        let ply = ply::test::start_position();
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
";

        assert_eq!(res, expected);
    }

}