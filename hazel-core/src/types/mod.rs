//! Types useful for representing parts of a chess game that don't naturally fit into other
//! modules.

// A freely moving cursor-based log object for recording and replaying actions.
pub mod log;
// Similar to a log, but no transactions, finite, relies on Zobrist.
pub mod tape;

// A type for storing a stack of moves while allowing for variations.
pub mod movesheet;

