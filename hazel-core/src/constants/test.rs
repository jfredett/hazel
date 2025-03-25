// TODO: sort out where these go? probably basic? maybe into the test fixture stuff?

use hazel_basic::color::Color;
use hazel_basic::square::*;
use crate::coup::rep::{Move, MoveType};

/// FEN for an empty board.
pub const EMPTY_POSITION_FEN: &str = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
/// FEN for the starting position
pub const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// FEN for a position in the london opening
pub const LONDON_POSITION_FEN: &str = "r1bqk2r/pp2bppp/2n1pn2/2pp4/3P1B2/2P1PN1P/PP1N1PP1/R2QKB1R b KQkq - 0 7";
/// FEN which includes an en passant move
pub const EN_PASSANT_POSITION_FEN: &str = "r1bqk2r/pp2bppp/2n1pn2/3p4/1PpP1B2/2P1PN1P/P2N1PP1/R2QKB1R b KQkq b3 0 8";
/// FEN for a position what starts after white plays 1. d4
pub const D4_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1";

/// From Chessprogramming Wiki, some test positions
/// https://www.chessprogramming.org/Perft_Results
pub const POS2_KIWIPETE_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
pub const POS3_KRP_ENDGAME_FEN: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
pub const POS4_MIRROR_1_FEN: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
pub const POS4_MIRROR_2_FEN: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1";
pub const POS5_BUGCATCHER_FEN: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
pub const POS6_STEVEN_FEN: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

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
        Move::new(A2, A3, MoveType::QUIET),
        Move::new(B2, B3, MoveType::QUIET),
        Move::new(G2, G3, MoveType::QUIET),
        Move::new(D5, D6, MoveType::QUIET),
        Move::new(A2, A4, MoveType::DOUBLE_PAWN),
        Move::new(G2, G4, MoveType::DOUBLE_PAWN),
        Move::new(G2, H3, MoveType::CAPTURE),
        Move::new(D5, E6, MoveType::CAPTURE),
        Move::new(C3, B1, MoveType::QUIET),
        Move::new(C3, D1, MoveType::QUIET),
        Move::new(C3, A4, MoveType::QUIET),
        Move::new(C3, B5, MoveType::QUIET),
        Move::new(E5, D3, MoveType::QUIET),
        Move::new(E5, C4, MoveType::QUIET),
        Move::new(E5, G4, MoveType::QUIET),
        Move::new(E5, C6, MoveType::QUIET),
        Move::new(E5, G6, MoveType::CAPTURE),
        Move::new(E5, D7, MoveType::CAPTURE),
        Move::new(E5, F7, MoveType::CAPTURE),
        Move::new(D2, C1, MoveType::QUIET),
        Move::new(D2, E3, MoveType::QUIET),
        Move::new(D2, F4, MoveType::QUIET),
        Move::new(D2, G5, MoveType::QUIET),
        Move::new(D2, H6, MoveType::QUIET),
        Move::new(E2, D1, MoveType::QUIET),
        Move::new(E2, F1, MoveType::QUIET),
        Move::new(E2, D3, MoveType::QUIET),
        Move::new(E2, C4, MoveType::QUIET),
        Move::new(E2, B5, MoveType::QUIET),
        Move::new(E2, A6, MoveType::CAPTURE),
        Move::new(A1, B1, MoveType::QUIET),
        Move::new(A1, C1, MoveType::QUIET),
        Move::new(A1, D1, MoveType::QUIET),
        Move::new(H1, F1, MoveType::QUIET),
        Move::new(H1, G1, MoveType::QUIET),
        Move::new(F3, D3, MoveType::QUIET),
        Move::new(F3, E3, MoveType::QUIET),
        Move::new(F3, G3, MoveType::QUIET),
        Move::new(F3, H3, MoveType::CAPTURE),
        Move::new(F3, F4, MoveType::QUIET),
        Move::new(F3, G4, MoveType::QUIET),
        Move::new(F3, F5, MoveType::QUIET),
        Move::new(F3, H5, MoveType::QUIET),
        Move::new(F3, F6, MoveType::CAPTURE),
        Move::new(E1, D1, MoveType::QUIET),
        Move::new(E1, F1, MoveType::QUIET),
        Move::long_castle(Color::WHITE),
        Move::short_castle(Color::WHITE)
    ];

    /// A vector containing all legal first moves for white.
    pub static ref STARTING_MOVES : Vec<Move> = vec![
        Move::new(A2, A3, MoveType::QUIET),
        Move::new(B2, B3, MoveType::QUIET),
        Move::new(C2, C3, MoveType::QUIET),
        Move::new(D2, D3, MoveType::QUIET),
        Move::new(E2, E3, MoveType::QUIET),
        Move::new(F2, F3, MoveType::QUIET),
        Move::new(G2, G3, MoveType::QUIET),
        Move::new(H2, H3, MoveType::QUIET),
        Move::new(A2, A4, MoveType::DOUBLE_PAWN),
        Move::new(B2, B4, MoveType::DOUBLE_PAWN),
        Move::new(C2, C4, MoveType::DOUBLE_PAWN),
        Move::new(D2, D4, MoveType::DOUBLE_PAWN),
        Move::new(E2, E4, MoveType::DOUBLE_PAWN),
        Move::new(F2, F4, MoveType::DOUBLE_PAWN),
        Move::new(G2, G4, MoveType::DOUBLE_PAWN),
        Move::new(H2, H4, MoveType::DOUBLE_PAWN),
        Move::new(B1, A3, MoveType::QUIET),
        Move::new(B1, C3, MoveType::QUIET),
        Move::new(G1, F3, MoveType::QUIET),
        Move::new(G1, H3, MoveType::QUIET),
    ];

    pub static ref D4_MOVES : Vec<Move> = vec![
        Move::new(A7, A6, MoveType::QUIET),
        Move::new(B7, B6, MoveType::QUIET),
        Move::new(C7, C6, MoveType::QUIET),
        Move::new(D7, D6, MoveType::QUIET),
        Move::new(E7, E6, MoveType::QUIET),
        Move::new(F7, F6, MoveType::QUIET),
        Move::new(G7, G6, MoveType::QUIET),

        Move::new(A7, A5, MoveType::DOUBLE_PAWN),
        Move::new(B7, B5, MoveType::DOUBLE_PAWN),
        Move::new(C7, C5, MoveType::DOUBLE_PAWN),
        Move::new(D7, D5, MoveType::DOUBLE_PAWN),
        Move::new(E7, E5, MoveType::DOUBLE_PAWN),
        Move::new(F7, F5, MoveType::DOUBLE_PAWN),
        Move::new(G7, G5, MoveType::DOUBLE_PAWN),
        Move::new(H7, H5, MoveType::DOUBLE_PAWN),

        Move::new(H7, H6, MoveType::QUIET),
        Move::new(B8, A6, MoveType::QUIET),
        Move::new(B8, C6, MoveType::QUIET),
        Move::new(G8, F6, MoveType::QUIET),
        Move::new(G8, H6, MoveType::QUIET),
    ];
}
