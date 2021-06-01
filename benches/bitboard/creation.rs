use super::*;

#[criterion(config())]
pub fn empty(c: &mut Criterion) {
    c.bench_function("Bitboard::empty/0", |b| b.iter(|| 
        black_box(Bitboard::empty())
    ));
}

#[criterion(config())]
pub fn full(c: &mut Criterion) {
    c.bench_function("Bitboard::full/0", |b| b.iter(|| 
        black_box(Bitboard::full())
    ));
}

#[criterion(config())]
pub fn from(c: &mut Criterion) {
    c.bench_function("Bitboard::from/1", |b| b.iter(|| 
        black_box(Bitboard::from(random_u64()))
    ));
}

#[criterion(config())]
pub fn from_notation(c: &mut Criterion) {
    c.bench_function("Bitboard::from_notation/1", |b| b.iter(|| 
        black_box(Bitboard::from_notation("a4"))
    ));
}