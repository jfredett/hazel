#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]

pub use criterion::{black_box, Criterion};
pub use criterion_macro::criterion;

use rand::distributions::{Distribution, Uniform};

use packed_simd::*;

use hazel::bitboard::*;

mod bitboard;

fn config() -> Criterion {
    Criterion::default()
        .sample_size(5000)
}

/// This measurement serves as a reference for 'the fastest possible thing' this machine can do.
/// It is simply adding two constant f64 values inside a black box. It's performance should
/// basically be 1 operation + any overhead, giving some context to the other benchmarks.
#[criterion(config())]
pub fn reference(c: &mut Criterion) {
    c.bench_function("Reference Measurement (Integer Operations)", |b| b.iter(||
        black_box(2i64 + 2i64)
    ));
}

#[criterion(config())]
pub fn simd_reference(c: &mut Criterion) {
    c.bench_function("Reference Measurement (SIMD creation)", |b| b.iter(||
        black_box(i64x4::new(0,1,2,3))
    ));
}

pub fn random_double() -> f64 {
    let range = Uniform::new(f64::MIN, f64::MAX);
    let mut rng = rand::thread_rng();

    range.sample(&mut rng)
}

pub fn random_usize() -> usize {
    let range = Uniform::new(usize::MIN, usize::MAX);
    let mut rng = rand::thread_rng();

    range.sample(&mut rng)
}
