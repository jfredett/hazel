use super::*;
use crate::bitboard::Bitboard;
use serde::{Deserialize, Serialize};

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

impl File {
    pub fn to_bitboard(self) -> Bitboard {
        FILE_MASKS[self as usize]
    }

    pub fn from_index(index: usize) -> Self {
        match index & 0o70 >> 3 {
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

    pub fn to_index(self) -> usize {
        self as usize
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
    use super::*;

    #[test]
    fn to_bitboard() {
        assert_eq!(File::A.to_bitboard(), Bitboard::from(0x0101010101010101));
        assert_eq!(File::B.to_bitboard(), Bitboard::from(0x0202020202020202));
        assert_eq!(File::C.to_bitboard(), Bitboard::from(0x0404040404040404));
        assert_eq!(File::D.to_bitboard(), Bitboard::from(0x0808080808080808));
        assert_eq!(File::E.to_bitboard(), Bitboard::from(0x1010101010101010));
        assert_eq!(File::F.to_bitboard(), Bitboard::from(0x2020202020202020));
        assert_eq!(File::G.to_bitboard(), Bitboard::from(0x4040404040404040));
        assert_eq!(File::H.to_bitboard(), Bitboard::from(0x8080808080808080));
    }

    #[test]
    fn from_index() {
        assert_eq!(File::from_index(0), File::A);
        assert_eq!(File::from_index(1), File::B);
        assert_eq!(File::from_index(2), File::C);
        assert_eq!(File::from_index(3), File::D);
        assert_eq!(File::from_index(4), File::E);
        assert_eq!(File::from_index(5), File::F);
        assert_eq!(File::from_index(6), File::G);
        assert_eq!(File::from_index(7), File::H);
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
}
