use super::*;

#[criterion(config())]
pub fn and_op(c: &mut Criterion) {
    let b1 = random_bitboard();
    let b2 = random_bitboard();
    c.bench_function("Bitboard::&/2", |b| b.iter(|| 
        black_box(b1 & b2)
    ));
}

#[criterion(config())]
pub fn or_op(c: &mut Criterion) {
    let b1 = random_bitboard();
    let b2 = random_bitboard();
    c.bench_function("Bitboard::|/2", |b| b.iter(|| 
        black_box(b1 | b2)
    ));
}

#[criterion(config())]
pub fn xor_op(c: &mut Criterion) {
    let b1 = random_bitboard();
    let b2 = random_bitboard();
    c.bench_function("Bitboard::^/2", |b| b.iter(|| 
        black_box(b1 ^ b2)
    ));
}

#[criterion(config())]
pub fn not_op(c: &mut Criterion) {
    let b1 = random_bitboard();
    c.bench_function("Bitboard::!/2", |b| b.iter(|| 
        black_box(!b1)
    ));
}

#[criterion(config())]
pub fn and_assign_op(c: &mut Criterion) {
    let mut b1 = random_bitboard();
    let b2 = random_bitboard();
    c.bench_function("Bitboard::&=/2", |b| b.iter(|| 
        black_box(b1 &= b2)
    ));
}

#[criterion(config())]
pub fn or_assign_op(c: &mut Criterion) {
    let mut b1 = random_bitboard();
    let b2 = random_bitboard();
    c.bench_function("Bitboard::|=/2", |b| b.iter(|| 
        black_box(b1 |= b2)
    ));
}

#[criterion(config())]
pub fn xor_assign_op(c: &mut Criterion) {
    let mut b1 = random_bitboard();
    let b2 = random_bitboard();
    c.bench_function("Bitboard::^=/2", |b| b.iter(|| 
        black_box(b1 ^= b2)
    ));
}