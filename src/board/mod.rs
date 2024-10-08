// Trait defs
pub mod interface;
// Representation of a piece on an arbitrary square
pub mod occupant;

// Board Representations
pub mod pieceboard; // represent a board as an array containing occupants
// TODO: pub mod bitboard; // represent a board as a collection of bitboards
// TODO: pub mod mailbox; // represent a board as a collection of pieces



pub use interface::*;
