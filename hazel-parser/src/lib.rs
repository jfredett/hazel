
// Move Notations
pub mod uci; // Canonical Move Notation, Universal Chess Interface uses 'Long Algebraic' notation,
             // hazel calls it 'UCI' for short.
pub mod san; // Standard Algebraic Notation

// Board Notations
pub mod ben;

// Game Notations
pub mod pgn;

// Re-exports

// Square Notations
pub use hazel_basic::square;
