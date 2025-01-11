//! Types useful for representing parts of a chess game that don't naturally fit into other
//! modules.

// Bitboard-related types
pub mod bitboard;
pub mod pextboard;

// Piece/Occupant/Color representation
pub mod piece;
pub mod occupant;
pub mod color;

// Used for shifting bitboards around mostly
pub mod direction;

// A freely moving cursor-based log object for recording and replaying actions.
pub mod log;
// A type for storing a stack of moves while allowing for variations.
pub mod movesheet;

// An Actor type, but with a fun name because no one can stop me.
pub mod witch;

pub use bitboard::Bitboard;
pub use pextboard::PEXTBoard;
pub use piece::Piece;
pub use occupant::Occupant;
pub use direction::Direction;
pub use color::Color;
