#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]
// NOTE: This is necessary because `bench!` is not super bright and just makes everything mutable for convenience sake.
#![allow(unused_mut)]


/*
*
* NOTE: FIXME:
*
* These benchmarks are old and crusty, someday, I will reimplement them, but a functioning engine
* is my priority right now. Originally I mostly wanted to mess around with Criterion, but along the
* way it turns our I preferred working on Chess as a problem then on benchmarking as a tool.
*
* So until I need to start optimizing for performance, these are bitrot.
*/

pub use criterion::{black_box, Criterion, Throughput};
pub use criterion_macro::criterion;

use rand::distributions::{Distribution, Uniform};

/*
#[macro_use]
extern crate lazy_static;
*/
#[macro_use]
extern crate paste;

/*
use hazel::bitboard::*;
use hazel::constants::*;
use hazel::ply::*;
//use hazel::constants::magic::*;
mod bitboard;
mod movegen;
mod ply;
*/

/// A helper macro for quickly defining benchmarks. Invoke as follows:
///
/// bench!(
///     group: GroupNameUsuallyAStructName,
///     pretty: "a nicer looking name for the report",
///     name: a_valid_name_for_the_defined_function,
///     test: { some test using the variables defined below },
///     where:
///         var => value;
///         can => specify;
///         many => vars(functions, allowed, too)
/// );
///
/// This will define a benchmark function, set it to measure throughput in items/s, and do wall-time.
///
/// TODO: Make the field ordering arbitrary
/// TODO: make `name` optional and autogenerate function names
/// TODO: better variable setup (would love it if it were just raw rust there)
#[macro_export]
macro_rules! bench {
    (group: $group:ident, pretty: $pretty:tt, name: $name:ident, test: $test:block, where: $($var:ident => $val:expr;)* ) => {
        paste! {
            #[allow(non_snake_case)]
            #[criterion(config())]
            pub fn [<$group _ $name>](c: &mut Criterion) {
                $(
                    // FIXME: This declares everything mutable-by-default, but the macro should ideally let you specify this.
                    let mut $var = $val;
                )*
                let mut g = c.benchmark_group(stringify!($group));
                g.throughput(Throughput::Elements(1 as u64));
                g.bench_function(format!("{}::{}", stringify!($group), $pretty), |b| b.iter(||
                    black_box($test)
                ));
                g.finish();
            }
        }
    }
}

fn config() -> Criterion {
    Criterion::default().sample_size(25000)
}

// NOTE: These measurements serve as references for 'the fastest possible thing' this machine can do.
// It's performance should basically be 1 operation + any overhead, giving some context to the
// other benchmarks.
bench!(
    group: Reference,
    pretty: "Measurement (Integer Operations)",
    name: reference,
    test: { 2i64 + 2i64 },
    where:
);

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

pub fn random_u64() -> u64 {
    let range = Uniform::new(u64::MIN, u64::MAX);
    let mut rng = rand::thread_rng();

    range.sample(&mut rng)
}

/*
pub fn random_bitboard() -> Bitboard {
    Bitboard::from(random_u64())
}
*/
