use super::*;

use std::fmt::{Debug, Display};

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: This is probably easier than I'm making it.
        write!(f, "{}{}", (b'a' + self.file() as u8) as char, (b'1' + self.rank() as u8) as char)
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} ({})", (b'a' + self.file() as u8) as char, (b'1' + self.rank() as u8) as char, self.index())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(format!("{}", A1), "a1");
        assert_eq!(format!("{}", H8), "h8");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", A1), "a1 (0)");
        assert_eq!(format!("{:?}", H8), "h8 (63)");
    }
}
