use std::fmt::Display;

use crate::{notation::SquareNotation, types::Color};


/// Represents a single square by it's index rooted at a1 = 0, h8 = 63
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(usize);

impl Square {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(&self) -> usize {
        self.0
    }

    pub const fn file(&self) -> usize {
        self.0 % 8
    }

    pub const fn rank(&self) -> usize {
        self.0 / 8
    }

    pub const fn coords(&self) -> (usize, usize) {
        (self.file(), self.rank())
    }

    pub const fn backrank_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => self.rank() == 0,
            Color::BLACK => self.rank() == 7,
        }
    }

    pub const fn backrank(&self) -> bool {
        self.rank() == 0 || self.rank() == 7
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file, rank) = self.coords();
        write!(f, "{}{}", (b'a' + file as u8) as char, (b'1' + rank as u8) as char)
    }
}

impl SquareNotation for Square { }

impl From<(usize, usize)> for Square {
    fn from(coords: (usize, usize)) -> Self {
        Self(coords.1 * 8 + coords.0)
    }
}

impl From<(u16, u16)> for Square {
    fn from(coords: (u16, u16)) -> Self {
        Self(coords.1 as usize * 8 + coords.0 as usize)
    }
}

impl From<Square> for usize {
    fn from(square: Square) -> usize {
        square.0
    }
}

impl From<&Square> for usize {
    fn from(square: &Square) -> usize {
        square.0
    }
}

impl From<&Square> for Square {
    fn from(square: &Square) -> Square {
        *square
    }
}

impl TryFrom<&[u8]> for Square {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(());
        }

        let file = value[0];
        let rank = value[1];

        let file = match file {
            b'a' => 0,
            b'b' => 1,
            b'c' => 2,
            b'd' => 3,
            b'e' => 4,
            b'f' => 5,
            b'g' => 6,
            b'h' => 7,
            _ => return Err(())
        };

        let rank = match rank {
            b'1' => 0,
            b'2' => 1,
            b'3' => 2,
            b'4' => 3,
            b'5' => 4,
            b'6' => 5,
            b'7' => 6,
            b'8' => 7,
            _ => return Err(())
        };

        Ok(Self(rank * 8 + file))
    }
}

impl TryFrom<&str> for Square {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(());
        }

        let file = value.chars().nth(0).ok_or(())?;
        let rank = value.chars().nth(1).ok_or(())?;

        let file = match file {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return Err(())
        };

        let rank = match rank {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return Err(())
        };

        Ok(Self(rank * 8 + file))
    }
}

impl TryFrom<usize> for Square {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        if index < 64 {
            Ok(Self(index))
        } else {
            Err(())
        }
    }
}


// These can be used to directly index into a board array at comptime with no overhead
// since everything here is const-time.
pub const A1: Square = Square::new(0);
pub const B1: Square = Square::new(1);
pub const C1: Square = Square::new(2);
pub const D1: Square = Square::new(3);
pub const E1: Square = Square::new(4);
pub const F1: Square = Square::new(5);
pub const G1: Square = Square::new(6);
pub const H1: Square = Square::new(7);
pub const A2: Square = Square::new(8);
pub const B2: Square = Square::new(9);
pub const C2: Square = Square::new(10);
pub const D2: Square = Square::new(11);
pub const E2: Square = Square::new(12);
pub const F2: Square = Square::new(13);
pub const G2: Square = Square::new(14);
pub const H2: Square = Square::new(15);
pub const A3: Square = Square::new(16);
pub const B3: Square = Square::new(17);
pub const C3: Square = Square::new(18);
pub const D3: Square = Square::new(19);
pub const E3: Square = Square::new(20);
pub const F3: Square = Square::new(21);
pub const G3: Square = Square::new(22);
pub const H3: Square = Square::new(23);
pub const A4: Square = Square::new(24);
pub const B4: Square = Square::new(25);
pub const C4: Square = Square::new(26);
pub const D4: Square = Square::new(27);
pub const E4: Square = Square::new(28);
pub const F4: Square = Square::new(29);
pub const G4: Square = Square::new(30);
pub const H4: Square = Square::new(31);
pub const A5: Square = Square::new(32);
pub const B5: Square = Square::new(33);
pub const C5: Square = Square::new(34);
pub const D5: Square = Square::new(35);
pub const E5: Square = Square::new(36);
pub const F5: Square = Square::new(37);
pub const G5: Square = Square::new(38);
pub const H5: Square = Square::new(39);
pub const A6: Square = Square::new(40);
pub const B6: Square = Square::new(41);
pub const C6: Square = Square::new(42);
pub const D6: Square = Square::new(43);
pub const E6: Square = Square::new(44);
pub const F6: Square = Square::new(45);
pub const G6: Square = Square::new(46);
pub const H6: Square = Square::new(47);
pub const A7: Square = Square::new(48);
pub const B7: Square = Square::new(49);
pub const C7: Square = Square::new(50);
pub const D7: Square = Square::new(51);
pub const E7: Square = Square::new(52);
pub const F7: Square = Square::new(53);
pub const G7: Square = Square::new(54);
pub const H7: Square = Square::new(55);
pub const A8: Square = Square::new(56);
pub const B8: Square = Square::new(57);
pub const C8: Square = Square::new(58);
pub const D8: Square = Square::new(59);
pub const E8: Square = Square::new(60);
pub const F8: Square = Square::new(61);
pub const G8: Square = Square::new(62);
pub const H8: Square = Square::new(63);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_from_index() {
        assert_eq!(Square::try_from(0), Ok(A1));
        assert_eq!(Square::try_from(63), Ok(H8));
        assert_eq!(Square::try_from(64), Err(()));
    }

    #[test]
    fn index_from_square() {
        assert_eq!(usize::from(A1), 0);
        assert_eq!(usize::from(H8), 63);
    }
}
