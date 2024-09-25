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
}
