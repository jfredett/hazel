//! This module defines a compact representation of chess moves from a given ply.
//!
//! NOTE: With respect to the name of this module. Ideally, this would be named 'move', like the
//! struct it ! defines, but alas, we are limited by rust reserving the `move` keyword for silly
//! things like memory safety or something.
//!
#![allow(non_snake_case)]


// TODO:
//
// I think this can basically be shoved into const time and fully precomputed. Every move is
// encodable in 16 bits, which means for a measily 8kb of memory I can just lookup the move by
// notation. That would eliminate the need for all the masking nonsense. I could have a set of
// macros that take a uci move and generate the correct lookup in const time? Metadata would
// probably still need to be provided, but generating it using the existing tooling should be
// straightforward enough I think.
//
// In any case, investigating const time for this code I think makes sense.


use crate::interface::{Alteration, Query};
use crate::notation::*;
use crate::types::{Color, Piece, Occupant};
use crate::constants::File;

use serde::{Deserialize, Serialize};

use tracing::instrument;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct Move(pub(crate) u16);

mod bitmangling;
mod creation;
mod compilation;
mod debug;
mod move_type;

#[cfg(test)] mod tests;

pub use move_type::*;
pub use bitmangling::*;

impl Move {


    #[instrument(skip(context))]
    pub fn to_pgn<C>(&self, context: &C) -> String where C: Query {
        if self.is_null() {
            return "".to_string();
        }

        let metadata = self.disambiguate(context).unwrap();

        if metadata.is_short_castle() {
            return "O-O".to_string();
        } else if metadata.is_long_castle() {
            return "O-O-O".to_string();
        }

        let mut result = String::default();

        let occ = context.get(self.source());

        let source_file = File::from_index(self.source().index()).to_pgn();

        result.push_str(match occ.piece().unwrap() {
            Piece::Pawn => if metadata.is_capture() { source_file } else { "" },
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Rook => "R",
            Piece::Queen => "Q",
            Piece::King => "K",
        });

        if metadata.is_capture() {
            result.push('x');
        }

        result.push_str(format!("{}", self.target()).to_owned().as_str());

        if metadata.is_promotion() {
            result.push('=');
            result.push_str(match metadata.promotion_piece().unwrap() {
                Piece::Knight => "N",
                Piece::Bishop => "B",
                Piece::Rook => "R",
                Piece::Queen => "Q",
                _ => { panic!("Invalid promotion piece, only NBRQ allowed: {:?}", self); },
            });
        }

        return result;
    }

    pub fn is_null(&self) -> bool {
        self.move_metadata().is_null()
    }

    pub fn long_castle(color: Color) -> Move {
        match color {
            Color::WHITE => Move::new(
                E1,
                C1,
                MoveType::LONG_CASTLE,
            ),
            Color::BLACK => Move::new(
                E8,
                C8,
                MoveType::LONG_CASTLE,
            ),
        }
    }

    pub fn short_castle(color: Color) -> Move {
        match color {
            Color::WHITE => Move::new(
                E1,
                G1,
                MoveType::SHORT_CASTLE,
            ),
            Color::BLACK => Move::new(
                E8,
                G8,
                MoveType::SHORT_CASTLE,
            ),
        }
    }

    // TODO: Produce a UCI object
    pub fn to_uci(&self) -> String {
        let source = self.source();
        let target = self.target();
        let metadata = self.move_metadata();
        if metadata.is_null() {
            "0000".to_string()
        } else {
            format!("{}{}{}", source, target, metadata.to_uci())
        }
    }

    // Some proxy methods
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        self.move_metadata().is_capture()
    }
    #[inline(always)]
    pub fn is_short_castle(&self) -> bool {
        self.move_metadata().is_short_castle()
    }
    #[inline(always)]
    pub fn is_long_castle(&self) -> bool {
        self.move_metadata().is_long_castle()
    }

    #[inline(always)]
    pub fn is_en_passant(&self) -> bool {
        self.move_metadata().is_en_passant()
    }

    #[inline(always)]
    pub fn is_ambiguous(&self) -> bool {
        self.move_metadata().is_ambiguous()
    }

    /// This checks that the move is a two-square move forward (relative to color), and that it
    /// started on the correct row.
    /// FIXME: Technically this could catch a two square move from, e.g., a rook or queen. So
    /// there's a bug here. I think this only gets called if we already know the piece is a pawn.
    #[inline(always)]
    pub fn is_double_pawn_push_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => { self.target_idx() - self.source_idx() == 0o20 && self.source_idx() & 0o70 == 0o10 },
            Color::BLACK => { self.source_idx() - self.target_idx() == 0o20 && self.source_idx() & 0o70 == 0o60 },
        }
    }
}
