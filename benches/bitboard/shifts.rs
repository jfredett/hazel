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
        _ => panic!("Not possible")
    }
}

#[criterion(config())]
pub fn shift(c: &mut Criterion) {
    let b1 = random_bitboard();
    let dir = random_direction();
    c.bench_function("Bitboard.shift/1", |b| b.iter(|| 
        black_box(b1.shift(dir))
    ));
}

#[criterion(config())]
pub fn shift_mut(c: &mut Criterion) {
    let mut b1 = random_bitboard();
    let dir = random_direction();
    c.bench_function("Bitboard.shift_mut/1", |b| b.iter(|| 
        black_box(b1.shift_mut(dir))
    ));
}