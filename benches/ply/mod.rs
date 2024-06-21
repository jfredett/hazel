use super::*;

lazy_static! {
    pub static ref START_POSITION_FEN: String =
        String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    pub static ref SIMPLE_POSITION_FEN: String =
        String::from("8/5k1p/2n5/3N4/6P1/3K4/8/8 w - - 0 1");
    pub static ref LONDON_POSITION_FEN: String =
        String::from("r1bqk2r/pp2bppp/2n1pn2/2pp4/3P1B2/2P1PN1P/PP1N1PP1/R2QKB1R b KQkq - 0 7");
    pub static ref START_POSITION: Ply = Ply::from_fen(&START_POSITION_FEN);
    pub static ref LONDON_POSITION: Ply = Ply::from_fen(&LONDON_POSITION_FEN);
    pub static ref SIMPLE_POSITION: Ply = Ply::from_fen(&SIMPLE_POSITION_FEN);
}

bench!(
    group: Ply,
    pretty: "occupancy_for/1 - White, Starting Position",
    name: occupancy_for_1,
    test: { START_POSITION.occupancy_for(Color::WHITE) },
    where:
);

bench!(
    group: Ply,
    pretty: "occupancy_for/1 - Black, Starting Position",
    name: occupancy_for_2,
    test: { START_POSITION.occupancy_for(Color::BLACK) },
    where:
);

bench!(
    group: Ply,
    pretty: "occupancy_for/1 - White, Simple Position",
    name: occupancy_for_3,
    test: { SIMPLE_POSITION.occupancy_for(Color::WHITE) },
    where:
);

bench!(
    group: Ply,
    pretty: "occupancy_for/1 - Black, Simple Position",
    name: occupancy_for_4,
    test: { SIMPLE_POSITION.occupancy_for(Color::BLACK) },
    where:
);

bench!(
    group: Ply,
    pretty: "occupancy_for/1 - White, London Position",
    name: occupancy_for_5,
    test: { LONDON_POSITION.occupancy_for(Color::WHITE) },
    where:
);

bench!(
    group: Ply,
    pretty: "occupancy_for/1 - Black, London Position",
    name: occupancy_for_6,
    test: { LONDON_POSITION.occupancy_for(Color::BLACK) },
    where:
);

bench!(
    group: Ply,
    pretty: "occupancy/0 - Starting Position",
    name: occupancy_1,
    test: { START_POSITION.occupancy() },
    where:
);

bench!(
    group: Ply,
    pretty: "occupancy/0 - Simple Position",
    name: occupancy_2,
    test: { SIMPLE_POSITION.occupancy() },
    where:
);

bench!(
    group: Ply,
    pretty: "occupancy/0 - London Position",
    name: occupancy_3,
    test: { LONDON_POSITION.occupancy() },
    where:
);

bench!(
    group: Ply,
    pretty: "from_fen/1 - Simple Position",
    name: from_fen_simple,
    test: { Ply::from_fen(&SIMPLE_POSITION_FEN) },
    where:
);

bench!(
    group: Ply,
    pretty: "from_fen/1 - Start Position",
    name: from_fen_start,
    test: { Ply::from_fen(&START_POSITION_FEN) },
    where:
);

bench!(
    group: Ply,
    pretty: "from_fen/1 - London Position",
    name: from_fen_london,
    test: { Ply::from_fen(&LONDON_POSITION_FEN) },
    where:
);
