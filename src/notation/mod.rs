
// Square Notations
pub mod square; // Represent a Square as an index with 0 at a1 and 63 at h8
// pub mod bitsquare; // Represent a Square as a bitboard with 1 at a1 and 2^63 at h8
// pub mod coord; // Represent a Square as a coordinate with a file and rank

// Move Notations
pub mod uci; // Canonical Move Notation, Universal Chess Interface uses 'Long Algebraic' notation,
             // hazel calls it 'UCI' for short.
//
// pub mod san; // Standard Algebraic Notation

// Board Notations
pub mod fen;

// Traits
pub mod interface;

// Re-exports
pub use square::*;
pub use interface::*;



