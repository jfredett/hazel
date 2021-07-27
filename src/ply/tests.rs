#![cfg(test)]

use super::*;

// NOTE: these are left as functions because they are used to test the `from_fen` and `to_fen`
// functions elsewhere. Most tests should use the constants defined in constants/test.rs
pub fn start_position() -> Ply {
    Ply {
        pawns: [
            Bitboard::from(0x00_00_00_00_00_00_FF_00),
            Bitboard::from(0x00_FF_00_00_00_00_00_00)
        ],
        kings: [
            Bitboard::from_notation("e1"),
            Bitboard::from_notation("e8")
        ],
        queens: [
            Bitboard::from_notation("d1"),
            Bitboard::from_notation("d8")
        ],
        rooks: [
            Bitboard::from_notation("a1") | Bitboard::from_notation("h1"),
            Bitboard::from_notation("a8") | Bitboard::from_notation("h8")
        ],
        bishops: [
            Bitboard::from_notation("c1") | Bitboard::from_notation("f1"),
            Bitboard::from_notation("c8") | Bitboard::from_notation("f8")
        ],
        knights: [
            Bitboard::from_notation("b1")| Bitboard::from_notation("g1"),
            Bitboard::from_notation("b8")| Bitboard::from_notation("g8")
        ],
        meta: Metadata::default()
    }
}

pub fn london_position() -> Ply {
    let mut meta = Metadata::default();
    meta.full_move_clock = 7;
    meta.to_move = Color::BLACK;

    Ply {
        pawns: [
            Bitboard::from_notation("a2") | Bitboard::from_notation("b2") | Bitboard::from_notation("c3") | 
            Bitboard::from_notation("d4") | Bitboard::from_notation("e3") | Bitboard::from_notation("f2") | 
            Bitboard::from_notation("g2") | Bitboard::from_notation("h3")
            ,
            Bitboard::from_notation("a7") | Bitboard::from_notation("b7") | Bitboard::from_notation("c5") | 
            Bitboard::from_notation("d5") | Bitboard::from_notation("e6") | Bitboard::from_notation("f7") | 
            Bitboard::from_notation("g7") | Bitboard::from_notation("h7")
        ],
        kings: [
            Bitboard::from_notation("e1"),
            Bitboard::from_notation("e8")
        ],
        queens: [
            Bitboard::from_notation("d1"),
            Bitboard::from_notation("d8")
        ],
        rooks: [
            Bitboard::from_notation("a1") | Bitboard::from_notation("h1"),
            Bitboard::from_notation("a8") | Bitboard::from_notation("h8")
        ],
        bishops: [
            Bitboard::from_notation("f1") | Bitboard::from_notation("f4"),
            Bitboard::from_notation("c8") | Bitboard::from_notation("e7")
        ],
        knights: [
            Bitboard::from_notation("d2")| Bitboard::from_notation("f3"),
            Bitboard::from_notation("c6")| Bitboard::from_notation("f6")
        ],
        meta
    }
}