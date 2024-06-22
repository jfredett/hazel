#![allow(non_snake_case)]

use crate::{
    movement::{Move, MoveType},
    moveset::MoveSet,
};

use super::*;

use bitboard::Bitboard;
use constants::*;
use serde::{Deserialize, Serialize};

mod creation;
mod debug;
mod make;
pub mod metadata;
mod movegen;
mod query;

#[cfg(test)]
mod tests;
#[cfg(test)]
use tests::*;

use metadata::*;

// FIXME: A lot of things right now are fighting uphill against the way I've laid out the ply struct.
// I think probably we should refactor it -- if only to make looking up pieces quicker, a single array
// with conventional locations for bitboards would be ideal. Being able to do some clever indexing shit
// might also improve the general QOL when using this structure.

const PIECE_COUNT: usize = 6;
const COLOR_COUNT: usize = 2;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Ply {
    /*
    pieces: [[Bitboard; PIECE_COUNT]; COLOR_COUNT]
        This now becomes a somewhat more simple thing to operate on. Instead of having to call
        self.each_friggin_piece, just loop over the array.  Will also make the eventual SIMD
        version of this a bit easier -- Bitboard -> X-Bitboard (where X is the SIMD width), this
        becomes a X-Ply. All the operations on it switch to X-wide instructions, etc.

        It would be nice to have an interface like:

        ply[Color::BLACK][Piece::Rook] => Bitboard for the rooks.

        Still want 1 bitboard per piece type, even if there are multiple pieces of that type. but
        want to support popcount (so I know how many pieces of a type there are), and also a better
        iterator for bitboards that quickly produces all the indices (using the tzcnt instruction
        repeatedly or something like.)
    */
    // indexed by COLOR
    pub pieces: [[Bitboard; PIECE_COUNT]; COLOR_COUNT],
    pub meta: Metadata,
}

// parse a fen string and construct the ply
impl Ply {
    /// Generates all legal moves from the current position for the current player
    ///
    /// TODO: Technically this is a lie, it generates pseudolegal moves.
    pub fn moves(&self) -> MoveSet {
        Move::generate(self, self.current_player())
    }

    /// Generates all legal moves from the current position for the enemy player
    /// Useful in determining threats for prophylaxis.
    ///
    /// TODO: Technically this is a lie, it generates pseudolegal moves.
    pub fn enemy_moves(&self) -> MoveSet {
        Move::generate(self, self.other_player())
    }

    pub fn en_passant(&self) -> Option<Bitboard> {
        self.meta.en_passant()
    }

    /// Returns the color of the player who will make the next move.
    pub fn current_player(&self) -> Color {
        self.meta.to_move
    }

    /// Returns the color of the player who is not currently making the next move.
    pub fn other_player(&self) -> Color {
        !self.meta.to_move
    }

    /// returns an 8x8 array with characters representing each piece in the proper locations
    fn board_buffer(&self) -> [[char; 8]; 8] {
        let mut buf = [['.'; 8]; 8];

        // Encode the board into a 8x8 array of chars.
        for rank in 0..8 {
            for file in FILES {
                for piece in PIECES {
                    for color in COLORS {
                        if self.piece_at(file, rank + 1, piece, color) {
                            buf[rank][file as usize] =
                                ASCII_PIECE_CHARS[color as usize][piece as usize];
                        }
                    }
                }
            }
        }

        buf
    }
}
