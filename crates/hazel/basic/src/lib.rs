#![allow(clippy::unusual_byte_groupings, clippy::needless_range_loop)]

pub mod ben;
pub mod castle_rights;
pub mod constants;
pub mod color;
pub mod direction;
pub mod file;
pub mod interface;
pub mod occupant;
pub mod piece;
pub mod position_metadata;
pub mod square;
// NOTE: I'm not sure this belongs here, it is right at the edge, but it's small and only relies on
// this crate, so probably it's fine.
pub mod zobrist;

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;
