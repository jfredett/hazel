#![feature(nll)]
#![cfg_attr(test, allow(unused_imports))]

pub use packed_simd::*;

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;

pub mod bitboard;
