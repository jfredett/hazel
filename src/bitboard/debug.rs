use super::*;
use std::fmt::{Debug, Display, Formatter, Result};

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f)?;
        for rank in (0..=7).rev() {
            write!(f, " {}", rank + 1)?;
            for file in 0..=7 {
                if self.is_set(rank, file) {
                    write!(f, " *")?;
                } else {
                    write!(f, " .")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "   a b c d e f g h")?;
        writeln!(f)
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
    "
    8 . . . . . . . .
    7 . . . . . . . .
    6 . . . . . . . .
    5 . . . . . . . .
    4 . . . . . . . .
    3 . . . . . . . .
    2 . . . . . . . .
    1 . . . . . . . .
      a b c d e f g h
    "
    */

    macro_rules! debug_display_test {
        ($method:ident, $notation:tt, $expected:tt) => {
            #[test]
            fn $method() {
                let b = Bitboard::from_notation($notation);

                assert_eq!(format!("{:?}", b), $expected);

                assert_eq!(format!("{}", b), $expected);
            }
        };
    }

    debug_display_test!(
        e4,
        "e4",
        "
 8 . . . . . . . .
 7 . . . . . . . .
 6 . . . . . . . .
 5 . . . . . . . .
 4 . . . . * . . .
 3 . . . . . . . .
 2 . . . . . . . .
 1 . . . . . . . .
   a b c d e f g h\n"
    );

    debug_display_test!(
        d4,
        "d4",
        "
 8 . . . . . . . .
 7 . . . . . . . .
 6 . . . . . . . .
 5 . . . . . . . .
 4 . . . * . . . .
 3 . . . . . . . .
 2 . . . . . . . .
 1 . . . . . . . .
   a b c d e f g h\n"
    );

    debug_display_test!(
        a8,
        "a8",
        "
 8 * . . . . . . .
 7 . . . . . . . .
 6 . . . . . . . .
 5 . . . . . . . .
 4 . . . . . . . .
 3 . . . . . . . .
 2 . . . . . . . .
 1 . . . . . . . .
   a b c d e f g h\n"
    );

    debug_display_test!(
        f6,
        "f6",
        "
 8 . . . . . . . .
 7 . . . . . . . .
 6 . . . . . * . .
 5 . . . . . . . .
 4 . . . . . . . .
 3 . . . . . . . .
 2 . . . . . . . .
 1 . . . . . . . .
   a b c d e f g h\n"
    );
    debug_display_test!(
        h8,
        "h8",
        "
 8 . . . . . . . *
 7 . . . . . . . .
 6 . . . . . . . .
 5 . . . . . . . .
 4 . . . . . . . .
 3 . . . . . . . .
 2 . . . . . . . .
 1 . . . . . . . .
   a b c d e f g h\n"
    );

    debug_display_test!(
        h1,
        "h1",
        "
 8 . . . . . . . .
 7 . . . . . . . .
 6 . . . . . . . .
 5 . . . . . . . .
 4 . . . . . . . .
 3 . . . . . . . .
 2 . . . . . . . .
 1 . . . . . . . *
   a b c d e f g h\n"
    );

    debug_display_test!(
        c2,
        "c2",
        "
 8 . . . . . . . .
 7 . . . . . . . .
 6 . . . . . . . .
 5 . . . . . . . .
 4 . . . . . . . .
 3 . . . . . . . .
 2 . . * . . . . .
 1 . . . . . . . .
   a b c d e f g h\n"
    );
}
