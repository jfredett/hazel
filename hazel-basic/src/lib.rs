#![allow(clippy::unusual_byte_groupings, clippy::needless_range_loop)]

pub mod ben;
pub mod color;
pub mod direction;
pub mod file;
pub mod occupant;
pub mod piece;
pub mod interface;
pub mod position_metadata;
pub mod castle_rights;
pub mod square;

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;



/// FEN for an empty board.
pub const EMPTY_POSITION_FEN: &str = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
/// FEN for the starting position
pub const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
