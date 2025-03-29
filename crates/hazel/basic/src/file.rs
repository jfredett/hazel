use serde::{Deserialize, Serialize};
use quickcheck::{Arbitrary, Gen};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Serialize, Deserialize)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl Arbitrary for File {
    fn arbitrary(g: &mut Gen) -> File {
        File::from_index(usize::arbitrary(g) % 8)
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<usize> for File {
    fn from(value: usize) -> Self {
        File::from_index(value)
    }
}

impl From<u8> for File {
    fn from(value: u8) -> Self {
        File::from_index(value as usize)
    }
}

impl From<File> for u8 {
    fn from(file: File) -> Self {
        file as u8
    }
}

impl From<File> for usize {
    fn from(file: File) -> Self {
        file as usize
    }
}

impl From<char> for File {
    fn from(value: char) -> Self {
        match value {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => panic!("Invalid file character"),
        }
    }
}

impl Iterator for File {
    type Item = File;

    fn next(&mut self) -> Option<File> {
        match self {
            File::A => Some(File::B),
            File::B => Some(File::C),
            File::C => Some(File::D),
            File::D => Some(File::E),
            File::E => Some(File::F),
            File::F => Some(File::G),
            File::G => Some(File::H),
            File::H => None,
        }
    }
}


impl File {
    /// This exists because there is no `ConstFrom` option.
    pub const fn from_index(index: usize) -> Self {
        match index & 0o07 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid file index"),
        }
    }

    pub const fn to_index(self) -> usize {
        self as usize
    }

    pub const fn to_byte(self) -> u8 {
        self as u8
    }

    pub fn prev(&mut self) -> Option<Self> {
        if self == &File::A {
            return None;
        }

        Some(match self {
            File::B => File::A,
            File::C => File::B,
            File::D => File::C,
            File::E => File::D,
            File::F => File::E,
            File::G => File::F,
            File::H => File::G,
            _ => unreachable!()
        })
    }

    pub fn to_pgn(self) -> &'static str {
        match self {
            File::A => "a",
            File::B => "b",
            File::C => "c",
            File::D => "d",
            File::E => "e",
            File::F => "f",
            File::G => "g",
            File::H => "h",
        }
    }
}

pub const FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

pub const NOT_A_FILE: u64 = 0xfefefefefefefefe;
pub const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

#[cfg(test)]
mod test {
    use quickcheck_macros::quickcheck;

    use super::*;


    // FIXME: Move to bitboard crate
    // #[test]
    // fn to_bitboard() {
    //     assert_eq!(File::A.to_bitboard(), Bitboard::from(0x0101010101010101u64));
    //     assert_eq!(File::B.to_bitboard(), Bitboard::from(0x0202020202020202u64));
    //     assert_eq!(File::C.to_bitboard(), Bitboard::from(0x0404040404040404u64));
    //     assert_eq!(File::D.to_bitboard(), Bitboard::from(0x0808080808080808u64));
    //     assert_eq!(File::E.to_bitboard(), Bitboard::from(0x1010101010101010u64));
    //     assert_eq!(File::F.to_bitboard(), Bitboard::from(0x2020202020202020u64));
    //     assert_eq!(File::G.to_bitboard(), Bitboard::from(0x4040404040404040u64));
    //     assert_eq!(File::H.to_bitboard(), Bitboard::from(0x8080808080808080u64));
    // }

    #[test]
    fn from_index() {
        assert_eq!(File::from_index(0usize), File::A);
        assert_eq!(File::from_index(1usize), File::B);
        assert_eq!(File::from_index(2usize), File::C);
        assert_eq!(File::from_index(3usize), File::D);
        assert_eq!(File::from_index(4usize), File::E);
        assert_eq!(File::from_index(5usize), File::F);
        assert_eq!(File::from_index(6usize), File::G);
        assert_eq!(File::from_index(7usize), File::H);
    }

    #[test]
    fn to_index() {
        assert_eq!(File::A.to_index(), 0);
        assert_eq!(File::B.to_index(), 1);
        assert_eq!(File::C.to_index(), 2);
        assert_eq!(File::D.to_index(), 3);
        assert_eq!(File::E.to_index(), 4);
        assert_eq!(File::F.to_index(), 5);
        assert_eq!(File::G.to_index(), 6);
        assert_eq!(File::H.to_index(), 7);
    }

    #[test]
    fn to_pgn() {
        assert_eq!(File::A.to_pgn(), "a");
        assert_eq!(File::B.to_pgn(), "b");
        assert_eq!(File::C.to_pgn(), "c");
        assert_eq!(File::D.to_pgn(), "d");
        assert_eq!(File::E.to_pgn(), "e");
        assert_eq!(File::F.to_pgn(), "f");
        assert_eq!(File::G.to_pgn(), "g");
        assert_eq!(File::H.to_pgn(), "h");
    }

    #[test]
    fn to_u8() {
        assert_eq!(u8::from(File::A), 0);
        assert_eq!(u8::from(File::B), 1);
        assert_eq!(u8::from(File::C), 2);
        assert_eq!(u8::from(File::D), 3);
        assert_eq!(u8::from(File::E), 4);
        assert_eq!(u8::from(File::F), 5);
        assert_eq!(u8::from(File::G), 6);
        assert_eq!(u8::from(File::H), 7);
    }

    #[test]
    fn from_u8() {
        assert_eq!(File::from(0u8), File::A);
        assert_eq!(File::from(1u8), File::B);
        assert_eq!(File::from(2u8), File::C);
        assert_eq!(File::from(3u8), File::D);
        assert_eq!(File::from(4u8), File::E);
        assert_eq!(File::from(5u8), File::F);
        assert_eq!(File::from(6u8), File::G);
        assert_eq!(File::from(7u8), File::H);
    }


    #[quickcheck]
    fn from_u8_to_u8_roundtrips(file: File) -> bool {
        let byte = file.to_byte();
        File::from(byte) == file
    }

    #[quickcheck]
    fn next_prev_roundtrips(file: File) -> bool {
        // an artifact of the test means that we'll try to go off the board first.
        if file == File::H { return true; }
        let mut file = file;
        file.next().and_then(|mut f| f.prev()) == Some(file)

    }
}
