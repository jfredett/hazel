use super::*;

bench!(
    group: Bitboard,
    pretty: "full/0",
    name: full,
    test: { Bitboard::full() },
    where:
);

bench!(
    group: Bitboard,
    pretty: "from_notation/0",
    name: from_notation,
    test: { Bitboard::from_notation("d4") },
    where:
);
