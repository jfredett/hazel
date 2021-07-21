#![allow(non_snake_case)]

use crate::{movement::{Move, MoveType}, moveset::MoveSet};

use super::*;

use bitboard::Bitboard;
use constants::*;
use serde::{Deserialize, Serialize};

mod debug;
mod creation;
mod make;
mod metadata;
mod query;

#[cfg(test)] mod tests;
#[cfg(test)] use tests::*;

use metadata::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct Ply {
    // indexed by COLOR
    pub pawns: [Bitboard; 2],
    pub kings: [Bitboard; 2],
    pub queens: [Bitboard; 2],
    // indexed by COLOR, then it's a/h rook
    pub rooks: [Bitboard; 2],
    pub bishops: [Bitboard; 2],
    pub knights: [Bitboard; 2],
    pub en_passant: Option<Bitboard>,
    pub full_move_clock: u32, // we're aligned to 64b, so this is the biggest that'll fit conveniently
    // NOTE: Maybe mask this off to 6 bits (halfmove count should never go > 50), then use the top two bits for 3-fold repetition? Stick the whole thing
    // in the metadata struct?
    pub half_move_clock: u8, // this is for the 50m rule
    pub meta: Metadata,
}

// parse a fen string and construct the ply
impl Ply {
    /// Provides a bitboard which shows the location of all squares occupied by pieces of the given
    /// color
    /// ```
    /// # #[macro_use] extern crate hazel;
    /// # use hazel::bitboard::Bitboard;
    /// # use hazel::ply::Ply;
    /// # use hazel::constants::*;
    /// let fen = "8/5k1p/2n5/3N4/6P1/3K4/8/8 w - - 0 1".to_string();
    /// let ply = Ply::from_fen(&fen);
    /// let expected_occupancy_white = bitboard!("d3", "d5", "g4");
    /// let expected_occupancy_black = bitboard!("c6", "f7", "h7");
    /// assert_eq!(ply.occupancy_for(Color::WHITE), expected_occupancy_white);
    /// assert_eq!(ply.occupancy_for(Color::BLACK), expected_occupancy_black);
    /// ```
    pub fn occupancy_for(&self, color: Color) -> Bitboard {
        self.kings[color as usize]   |
        self.queens[color as usize]  |
        self.rooks[color as usize]   |
        self.bishops[color as usize] |
        self.knights[color as usize] |
        self.pawns[color as usize]
    }
    
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
    
    /// Provides a bitboard which shows the location of all squares occupied by pieces of any
    /// color
    /// ```
    /// # #[macro_use] extern crate hazel;
    /// # use hazel::bitboard::Bitboard;
    /// # use hazel::ply::Ply;
    /// # use hazel::constants::*;
    /// let fen = "8/5k1p/2n5/3N4/6P1/3K4/8/8 w - - 0 1".to_string();
    /// let ply = Ply::from_fen(&fen);
    /// let expected_occupancy_white = bitboard!("d3", "d5", "g4");
    /// let expected_occupancy_black = bitboard!("c6", "f7", "h7");
    /// assert_eq!(ply.occupancy(), expected_occupancy_white | expected_occupancy_black);
    /// ```
    pub fn occupancy(&self) -> Bitboard {
        self.occupancy_for(Color::WHITE) | self.occupancy_for(Color::BLACK)
    }
    
    /// Returns the color of the player who will make the next move.
    pub fn current_player(&self) -> Color {
        if self.meta.contains(Metadata::BLACK_TO_MOVE) {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }
    
    /// Returns the color of the player who is not currently making the next move.
    pub fn other_player(&self) -> Color {
        if self.meta.contains(Metadata::BLACK_TO_MOVE) {
            Color::WHITE
        } else {
            Color::BLACK
        }
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
                            buf[rank][file as usize] = ASCII_PIECE_CHARS[color as usize][piece as usize];
                        }
                    }
                }
            }
        }

        buf
    }
}