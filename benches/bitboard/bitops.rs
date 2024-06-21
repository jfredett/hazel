use super::*;

bench!(
    group: Bitboard,
    pretty: "&/2",
    name: and_op,
    test: { b1 & b2 },
    where:
         b1 => random_bitboard();
         b2 => random_bitboard();
);

bench!(
    group: Bitboard,
    pretty: "|/2",
    name: or_op,
    test: { b1 | b2 },
    where:
         b1 => random_bitboard();
         b2 => random_bitboard();
);

bench!(
    group: Bitboard,
    pretty: "^/2",
    name: xor_op,
    test: { b1 ^ b2 },
    where:
         b1 => random_bitboard();
         b2 => random_bitboard();
);

bench!(
    group: Bitboard,
    pretty: "!/1",
    name: and_op,
    test: { !b1 },
    where:
         b1 => random_bitboard();
);

bench!(
    group: Bitboard,
    pretty: "&=/2",
    name: and_assign_op,
    test: { b1 &= b2 },
    where:
        b1 => random_bitboard();
        b2 => random_bitboard();
);

bench!(
    group: Bitboard,
    pretty: "|=/2",
    name: or_assign_op,
    test: { b1 |= b2 },
    where:
        b1 => random_bitboard();
        b2 => random_bitboard();
);

bench!(
    group: Bitboard,
    pretty: "^=/2",
    name: xor_assign_op,
    test: { b1 ^= b2 },
    where:
        b1 => random_bitboard();
        b2 => random_bitboard();
);
