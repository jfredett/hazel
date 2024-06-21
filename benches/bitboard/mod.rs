use super::*;

mod bitops;
mod creation;
mod shifts;

bench!(
    group: Bitboard,
    pretty: "is_empty/0",
    name: is_empty,
    test: { bb.is_empty() },
    where:
        bb => Bitboard::empty();
);

bench!(
    group: Bitboard,
    pretty: "is_full/0",
    name: is_full,
    test: { bb.is_full() },
    where:
        bb => Bitboard::full();
);

bench!(
    group: Bitboard,
    pretty: "set/2",
    name: set,
    test: { bb.set(rank,file) },
    where:
        bb => Bitboard::empty();
        rank => random_usize() % 8;
        file => random_usize() % 8;
);

bench!(
    group: Bitboard,
    pretty: "set_by_index/2",
    name: set_by_index,
    test: { bb.set_by_index(index) },
    where:
        bb => Bitboard::empty();
        index => random_usize() % 64;
);

bench!(
    group: Bitboard,
    pretty: "set_by_notation/2",
    name: set_by_notation,
    test: { bb.set_by_notation(notation) },
    where:
        bb => Bitboard::empty();
        notation => "d4";
);

bench!(
    group: Bitboard,
    pretty: "unset/2",
    name: unset,
    test: { bb.unset(3,3) },
    where:
        bb => Bitboard::from_notation("d4");
);

bench!(
    group: Bitboard,
    pretty: "flip/2",
    name: unset,
    test: { bb.flip(3,3) },
    where:
        bb => Bitboard::from_notation("d4");
);

bench!(
    group: Bitboard,
    pretty: "is_set/2",
    name: is_set,
    test: { bb.is_set(3,3) },
    where:
        bb => Bitboard::from_notation("d4");
);

bench!(
    group: Bitboard,
    pretty: "is_index_set/1",
    name: is_index_set,
    test: { bb.is_index_set(0o33) },
    where:
        bb => Bitboard::from_notation("d4");
);

bench!(
    group: Bitboard,
    pretty: "is_notation_set/1",
    name: is_notation_set,
    test: { bb.is_notation_set("d4") },
    where:
        bb => Bitboard::from_notation("d4");
);

bench!(
    group: Bitboard,
    pretty: "count/0",
    name: count,
    test: { bb.count() },
    where:
        bb => Bitboard::from(random_u64());
);
