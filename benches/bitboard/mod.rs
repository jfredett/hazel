use super::*;

#[criterion(config())]
pub fn is_empty(c: &mut Criterion) {
    let bb = Bitboard::empty();
    c.bench_function("Bitboard::is_empty/0", |b| b.iter(|| 
            black_box(bb.is_empty())
    ));
}
