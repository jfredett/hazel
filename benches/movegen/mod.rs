use super::*;

use hazel::pextboard::*;

bench!(
    group: MoveGen,
    pretty: "rook_attacks/2 - Slow Method",
    name: rook_attacks_slow,
    test: { slow_rook_attacks(rook_idx, occupancy) },
    where:
        rook_idx => Bitboard::from(1 << (random_u64() % 64)); 
        occupancy => random_bitboard();
);

bench!(
    group: MoveGen,
    pretty: "rook_attacks/2 - PEXTBoard Method",
    name: rook_attacks_slow,
    test: { hazel::pextboard::attacks_for(Piece::Rook, rook_idx, occupancy) },
    where:
        rook_idx => random_usize() % 64; 
        occupancy => random_bitboard();
);

bench!(
    group: MoveGen,
    pretty: "bishop_attacks/2 - Slow Method",
    name: bishop_attacks_slow,
    test: { slow_bishop_attacks(bishop_idx, occupancy) },
    where:
        bishop_idx => Bitboard::from(1 << (random_u64() % 64)); 
        occupancy => random_bitboard();
);

bench!(
    group: MoveGen,
    pretty: "bishop_attacks/2 - PEXTBoard Method",
    name: bishop_attacks_slow,
    test: { hazel::pextboard::attacks_for(Piece::Bishop, bishop_idx, occupancy) },
    where:
        bishop_idx => random_usize() % 64; 
        occupancy => random_bitboard();
);