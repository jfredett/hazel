#![cfg(test)]

use super::*;

// NOTE: these are left as functions because they are used to test the `from_fen` and `to_fen`
// functions elsewhere. Most tests should use the constants defined in constants/test.rs
pub fn start_position() -> Ply {
    Ply {
        pieces: [[
            bitboard!("b1", "g1"),
            bitboard!("c1", "f1"),
            bitboard!("a1", "h1"),
            bitboard!("d1"),
            bitboard!("e1"),
            Bitboard::from(0x00_00_00_00_00_00_FF_00)
        ], [
            bitboard!("b8", "g8"),
            bitboard!("c8", "f8"),
            bitboard!("a8", "h8"),
            bitboard!("d8"),
            bitboard!("e8"),
            Bitboard::from(0x00_FF_00_00_00_00_00_00)
        ]],
        meta: Metadata::default()
    }
}

pub fn london_position() -> Ply {
    let mut meta = Metadata::default();
    meta.full_move_clock = 7;
    meta.to_move = Color::BLACK;

    Ply {
        pieces: [[
            bitboard!("d2", "f3"),
            bitboard!("f1", "f4"),
            bitboard!("a1", "h1"),
            bitboard!("d1"),
            bitboard!("e1"),
            bitboard!("a2", "b2", "c3", "d4", "e3", "f2", "g2", "h3")
        ], [
            bitboard!("c6", "f6"),
            bitboard!("c8", "e7"),
            bitboard!("a8", "h8"),
            bitboard!("d8"),
            bitboard!("e8"),
            bitboard!("a7", "b7", "c5", "d5", "e6", "f7", "g7", "h7")
        ]],
        meta
    }
}