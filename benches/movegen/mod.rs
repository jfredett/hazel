use super::*;

use hazel::{pextboard::*, random};

bench!(
    group: MoveGen,
    pretty: "rook_attacks/2 - Slow Method",
    name: rook_attacks_slow,
    test: { slow_rook_attacks(rook_idx, random_bitboard()) },
    where:
        rook_idx => Bitboard::from(1 << (random_u64() % 64)); 
);

bench!(
    group: MoveGen,
    pretty: "rook_attacks/2 - PEXTBoard Method",
    name: rook_attacks_slow,
    test: { hazel::pextboard::attacks_for(Piece::Rook, rook_idx, random_bitboard()) },
    where:
        rook_idx => random_usize() % 64; 
        preload => black_box(hazel::pextboard::attacks_for(Piece::Rook, rook_idx, random_bitboard()));

);

bench!(
    group: MoveGen,
    pretty: "bishop_attacks/2 - Slow Method",
    name: bishop_attacks_slow,
    test: { slow_bishop_attacks(bishop_idx, random_bitboard()) },
    where:
        bishop_idx => Bitboard::from(1 << (random_u64() % 64)); 
);

bench!(
    group: MoveGen,
    pretty: "bishop_attacks/2 - PEXTBoard Method",
    name: bishop_attacks_slow,
    test: { hazel::pextboard::attacks_for(Piece::Bishop, bishop_idx, random_bitboard()) },
    where:
        bishop_idx => random_usize() % 64; 
);