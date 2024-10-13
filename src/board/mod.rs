///! Board Representation

// Trait defs
pub mod interface;

// Board Representations
pub mod simple; // represent a board as an array containing occupants
pub mod bit; // represent a board as a collection of bitboards
pub mod mailbox; // represent a board as a collection of pieces

pub use interface::*;

pub use simple::PieceBoard;

