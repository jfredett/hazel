use super::*;

use hazel::constants::move_tables::ROOK_ATTACKS;

bench!(
    group: MoveGen,
    pretty: "rook_attacks/2 - Slow Method",
    name: rook_attacks_slow,
    test: { slow_rook_attacks(rook_idx, occupancy) },
    where:
        rook_idx => Bitboard::from(random_u64() % 64); 
        occupancy => random_bitboard();
);

bench!(
    group: MoveGen,
    pretty: "rook_attacks/2 - Plain Magic Method",
    name: rook_attacks_slow,
    test: { ROOK_ATTACKS[rook_idx as usize].attacks_for(occupancy) },
    where:
        rook_idx => random_u64() % 64; 
        occupancy => random_bitboard();
);