use super::*;

pub fn random_direction() -> Direction {
    let d = random_usize() % 8;
    match d {
        0 => Direction::N,
        1 => Direction::NE,
        2 => Direction::E,
        3 => Direction::SE,
        4 => Direction::S,
        5 => Direction::SW,
        6 => Direction::W,
        7 => Direction::NW,
        _ => panic!("Not possible"),
    }
}

bench!(
        group: Bitboard,
        pretty: "shift/1",
        name: shift,
        test: { b1.shift(dir) },
        where:
            b1 => random_bitboard();
            dir => random_direction();
);

bench!(
        group: Bitboard,
        pretty: "shift_mut/1",
        name: shift_mut,
        test: { b1.shift_mut(dir) },
        where:
            b1 => random_bitboard();
            dir => random_direction();
);
