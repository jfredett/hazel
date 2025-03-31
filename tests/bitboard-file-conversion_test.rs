use hazel_core::file::File;
use hazel_bitboard::bitboard::Bitboard;
use hazel_bitboard::extensions::*;


#[test]
fn to_bitboard() {
    assert_eq!(File::A.as_bitboard(), Bitboard::new(0x0101010101010101u64));
    assert_eq!(File::B.as_bitboard(), Bitboard::new(0x0202020202020202u64));
    assert_eq!(File::C.as_bitboard(), Bitboard::new(0x0404040404040404u64));
    assert_eq!(File::D.as_bitboard(), Bitboard::new(0x0808080808080808u64));
    assert_eq!(File::E.as_bitboard(), Bitboard::new(0x1010101010101010u64));
    assert_eq!(File::F.as_bitboard(), Bitboard::new(0x2020202020202020u64));
    assert_eq!(File::G.as_bitboard(), Bitboard::new(0x4040404040404040u64));
    assert_eq!(File::H.as_bitboard(), Bitboard::new(0x8080808080808080u64));
}
