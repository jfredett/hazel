#![cfg(test)]

use super::*;
use crate::movement::*;
use either::Either;

/// FEN for the starting position
pub const START_POSITION_FEN : &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// FEN for a position in the london opening
pub const LONDON_POSITION_FEN : &str = "r1bqk2r/pp2bppp/2n1pn2/2pp4/3P1B2/2P1PN1P/PP1N1PP1/R2QKB1R b KQkq - 0 7";
/// FEN which includes an en passant move
pub const EN_PASSANT_POSITION_FEN : &str = "r1bqk2r/pp2bppp/2n1pn2/3p4/1PpP1B2/2P1PN1P/P2N1PP1/R2QKB1R b KQkq b3 0 8";


lazy_static! {
    /// A vector containing all legal first moves for white.
    pub static ref STARTING_MOVES : Vec<Move> = vec![
        Move::from_notation("a2", "a4", Either::Left(MoveType::quiet())),
        Move::from_notation("a2", "a3", Either::Left(MoveType::quiet())),
        Move::from_notation("b2", "b4", Either::Left(MoveType::quiet())),
        Move::from_notation("b2", "b3", Either::Left(MoveType::quiet())),
        Move::from_notation("c2", "c4", Either::Left(MoveType::quiet())),
        Move::from_notation("c2", "c3", Either::Left(MoveType::quiet())),
        Move::from_notation("d2", "d4", Either::Left(MoveType::quiet())),
        Move::from_notation("d2", "d3", Either::Left(MoveType::quiet())),
        Move::from_notation("e2", "e4", Either::Left(MoveType::quiet())),
        Move::from_notation("e2", "e3", Either::Left(MoveType::quiet())),
        Move::from_notation("f2", "f4", Either::Left(MoveType::quiet())),
        Move::from_notation("f2", "f4", Either::Left(MoveType::quiet())),
        Move::from_notation("g2", "g4", Either::Left(MoveType::quiet())),
        Move::from_notation("g2", "g4", Either::Left(MoveType::quiet())),
        Move::from_notation("h2", "h3", Either::Left(MoveType::quiet())),
        Move::from_notation("h2", "h3", Either::Left(MoveType::quiet())),
        Move::from_notation("b1", "a3", Either::Left(MoveType::quiet())),
        Move::from_notation("b1", "c3", Either::Left(MoveType::quiet())),
        Move::from_notation("g2", "f3", Either::Left(MoveType::quiet())),
        Move::from_notation("g2", "h3", Either::Left(MoveType::quiet())),
    ];
}