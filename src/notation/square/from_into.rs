use super::*;

impl From<(usize, usize)> for Square {
    fn from(coords: (usize, usize)) -> Self {
        Self(coords.0 * 8 + coords.1)
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
        Self::try_from(value.as_bytes())
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
