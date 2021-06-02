#![feature(nll)]
#![cfg_attr(test, allow(unused_imports))]

pub use packed_simd::*;

#[cfg(test)]
#[macro_use] extern crate quickcheck_macros;

#[macro_use] extern crate lazy_static;

#[macro_use] 
extern crate bitflags;

pub mod constants;
pub mod bitboard;
pub mod ply;
