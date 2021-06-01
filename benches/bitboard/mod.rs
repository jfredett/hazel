use super::*;

mod creation;
mod bitops;
mod shifts; 

pub fn random_bitboard() -> Bitboard {
    Bitboard::from(random_u64())
}

#[criterion(config())]
pub fn is_empty(c: &mut Criterion) {
    let bb = Bitboard::empty();
    c.bench_function("Bitboard.is_empty/0", |b| b.iter(|| 
            black_box(bb.is_empty())
    ));
}

#[criterion(config())]
pub fn is_full(c: &mut Criterion) {
    let bb = Bitboard::empty();
    c.bench_function("Bitboard.is_full/0", |b| b.iter(|| 
            black_box(bb.is_full())
    ));
}

#[criterion(config())]
pub fn set(c: &mut Criterion) {
    let mut bb = Bitboard::empty();
    let rank = random_usize() % 8;
    let file = random_usize() % 8;
    c.bench_function("Bitboard.set/2", |b| b.iter(|| 
            black_box(bb.set(rank, file))
    ));
}

#[criterion(config())]
pub fn set_by_index(c: &mut Criterion) {
    let mut bb = Bitboard::empty();
    let index = random_usize() % 64;
    c.bench_function("Bitboard.set_by_index/1", |b| b.iter(|| 
            black_box(bb.set_by_index(index))
    ));
}

#[criterion(config())]
pub fn set_by_notation(c: &mut Criterion) {
    let mut bb = Bitboard::empty();
    let notation = "d4";
    c.bench_function("Bitboard.set_by_notation/1", |b| b.iter(|| 
            black_box(bb.set_by_notation(notation))
    ));
}

#[criterion(config())]
pub fn unset(c: &mut Criterion) {
    let mut bb = Bitboard::empty();
    bb.set_by_notation("d4");
    c.bench_function("Bitboard.unset/2", |b| b.iter(|| 
            black_box(bb.unset(3,3))
    ));
}

#[criterion(config())]
pub fn flip(c: &mut Criterion) {
    let mut bb = Bitboard::empty();
    bb.set_by_notation("d4");
    c.bench_function("Bitboard.flip/2", |b| b.iter(|| 
            black_box(bb.flip(3,3))
    ));
}

#[criterion(config())]
pub fn is_set(c: &mut Criterion) {
    let mut bb = Bitboard::empty();
    bb.set_by_notation("d4");
    c.bench_function("Bitboard.is_set/2", |b| b.iter(|| 
            black_box(bb.is_set(3,3))
    ));
}

#[criterion(config())]
pub fn is_index_set(c: &mut Criterion) {
    let mut bb = Bitboard::empty();
    bb.set_by_notation("d4");
    c.bench_function("Bitboard.is_index_set/1", |b| b.iter(|| 
            black_box(bb.is_index_set(0o33))
    ));
}

#[criterion(config())]
pub fn is_notation_set(c: &mut Criterion) {
    let mut bb = Bitboard::empty();
    bb.set_by_notation("d4");
    c.bench_function("Bitboard.is_notation_set/1", |b| b.iter(|| 
            black_box(bb.is_notation_set("d4"))
    ));
}

#[criterion(config())]
pub fn count(c: &mut Criterion) {
    let bb = Bitboard::from(random_u64());
    c.bench_function("Bitboard.count/0", |b| b.iter(|| 
        black_box(bb.count())
    ));
}