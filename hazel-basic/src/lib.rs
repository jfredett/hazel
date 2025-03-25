#![allow(clippy::unusual_byte_groupings, clippy::needless_range_loop)]

pub mod color;
pub mod direction;
pub mod file;
pub mod occupant;
pub mod piece;
pub mod square;

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;
