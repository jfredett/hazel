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
/// FEN for a position what starts after white plays 1. d4
pub const D4_POSITION_FEN : &str = "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1";

/// From Chessprogramming Wiki, some test positions
/// https://www.chessprogramming.org/Perft_Results
pub const POS2_KIWIPETE_FEN : &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
pub const POS3_KRP_ENDGAME_FEN : &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
pub const POS4_MIRROR_1_FEN : &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
pub const POS4_MIRROR_2_FEN : &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1";
pub const POS5_BUGCATCHER_FEN : &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
pub const POS6_STEVEN_FEN : &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";


lazy_static! {
    
    /// Perft move counts in order of depth (vec[0] == depth 1)
    /// The coorespond to the similarly named _FEN constants.
    ///
    /// https://www.chessprogramming.org/Perft_Results
    /// 
    /// TODO: This is kinda lousy, build.rs and yaml is probably the right way to provide this info to the testing stuff.
    ///
    /// REVIEW: I typed in these numbers by hand, so if you find bugs with them double check the numbers then remove this note
    pub static ref POS2_KIWIPETE_PERFT_COUNTS : Vec<usize> = vec![48, 2039, 97862, 4085603, 193690690, 8031647685];
    pub static ref POS3_KRP_ENDGAME_COUNTS : Vec<usize> = vec![14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393];
    // NOTE: Applies to both mirrors of the position.
    pub static ref POS4_MIRROR_COUNTS : Vec<usize> = vec![6, 264, 9467, 422333, 15833292, 706045033];
    pub static ref POS5_BUGCATCHER_COUNTS : Vec<usize> = vec![44, 1486, 62379, 2103487, 89941194];
    pub static ref POS6_STEVEN_COUNTS : Vec<usize> = vec![1, 46, 2079, 89890, 3894594, 164075551, 6923051137, 287188994746, 11923589843526, 490154852788714];
    
    pub static ref POS2_KIWIPETE_MOVES : Vec<Move> = vec![
        Move::from_notation("a2", "a3", Either::Left(MoveType::quiet())),
        Move::from_notation("b2", "b3", Either::Left(MoveType::quiet())),
        Move::from_notation("g2", "g3", Either::Left(MoveType::quiet())),
        Move::from_notation("d5", "d6", Either::Left(MoveType::quiet())),
        Move::from_notation("a2", "a4", Either::Left(MoveType::quiet())),
        Move::from_notation("g2", "g4", Either::Left(MoveType::quiet())),
        Move::from_notation("g2", "h3", Either::Left(MoveType::quiet())),
        Move::from_notation("d5", "e6", Either::Left(MoveType::quiet())),
        Move::from_notation("c3", "b1", Either::Left(MoveType::quiet())),
        Move::from_notation("c3", "d1", Either::Left(MoveType::quiet())),
        Move::from_notation("c3", "a4", Either::Left(MoveType::quiet())),
        Move::from_notation("c3", "b5", Either::Left(MoveType::quiet())),
        Move::from_notation("e5", "d3", Either::Left(MoveType::quiet())),
        Move::from_notation("e5", "c4", Either::Left(MoveType::quiet())),
        Move::from_notation("e5", "g4", Either::Left(MoveType::quiet())),
        Move::from_notation("e5", "c6", Either::Left(MoveType::quiet())),
        Move::from_notation("e5", "g6", Either::Left(MoveType::quiet())),
        Move::from_notation("e5", "d7", Either::Left(MoveType::quiet())),
        Move::from_notation("e5", "f7", Either::Left(MoveType::quiet())),
        Move::from_notation("d2", "c1", Either::Left(MoveType::quiet())),
        Move::from_notation("d2", "e3", Either::Left(MoveType::quiet())),
        Move::from_notation("d2", "f4", Either::Left(MoveType::quiet())),
        Move::from_notation("d2", "g5", Either::Left(MoveType::quiet())),
        Move::from_notation("d2", "h6", Either::Left(MoveType::quiet())),
        Move::from_notation("e2", "d1", Either::Left(MoveType::quiet())),
        Move::from_notation("e2", "f1", Either::Left(MoveType::quiet())),
        Move::from_notation("e2", "d3", Either::Left(MoveType::quiet())),
        Move::from_notation("e2", "c4", Either::Left(MoveType::quiet())),
        Move::from_notation("e2", "b5", Either::Left(MoveType::quiet())),
        Move::from_notation("e2", "a6", Either::Left(MoveType::quiet())),
        Move::from_notation("a1", "b1", Either::Left(MoveType::quiet())),
        Move::from_notation("a1", "c1", Either::Left(MoveType::quiet())),
        Move::from_notation("a1", "d1", Either::Left(MoveType::quiet())),
        Move::from_notation("h1", "f1", Either::Left(MoveType::quiet())),
        Move::from_notation("h1", "g1", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "d3", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "e3", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "g3", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "h3", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "f4", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "g4", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "f5", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "h5", Either::Left(MoveType::quiet())),
        Move::from_notation("f3", "f6", Either::Left(MoveType::quiet())),
        Move::from_notation("e1", "d1", Either::Left(MoveType::quiet())),
        Move::from_notation("e1", "f1", Either::Left(MoveType::quiet())),
        Move::from_notation("e1", "g1", Either::Left(MoveType::quiet())),
        Move::from_notation("e1", "c1", Either::Left(MoveType::quiet())),
    ];

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
        Move::from_notation("g1", "f3", Either::Left(MoveType::quiet())),
        Move::from_notation("g1", "h3", Either::Left(MoveType::quiet())),
    ];

    pub static ref D4_MOVES : Vec<Move> = vec![
        Move::from_notation("a7", "a5", Either::Left(MoveType::quiet())),
        Move::from_notation("a7", "a6", Either::Left(MoveType::quiet())),
        Move::from_notation("b7", "b5", Either::Left(MoveType::quiet())),
        Move::from_notation("b7", "b6", Either::Left(MoveType::quiet())),
        Move::from_notation("c7", "c5", Either::Left(MoveType::quiet())),
        Move::from_notation("c7", "c6", Either::Left(MoveType::quiet())),
        Move::from_notation("d7", "d5", Either::Left(MoveType::quiet())),
        Move::from_notation("d7", "d6", Either::Left(MoveType::quiet())),
        Move::from_notation("e7", "e5", Either::Left(MoveType::quiet())),
        Move::from_notation("e7", "e6", Either::Left(MoveType::quiet())),
        Move::from_notation("f7", "f5", Either::Left(MoveType::quiet())),
        Move::from_notation("f7", "f5", Either::Left(MoveType::quiet())),
        Move::from_notation("g7", "g5", Either::Left(MoveType::quiet())),
        Move::from_notation("g7", "g5", Either::Left(MoveType::quiet())),
        Move::from_notation("h7", "h6", Either::Left(MoveType::quiet())),
        Move::from_notation("h7", "h6", Either::Left(MoveType::quiet())),
        Move::from_notation("b8", "a6", Either::Left(MoveType::quiet())),
        Move::from_notation("b8", "c6", Either::Left(MoveType::quiet())),
        Move::from_notation("g8", "f6", Either::Left(MoveType::quiet())),
        Move::from_notation("g8", "h6", Either::Left(MoveType::quiet())),
    ];
}