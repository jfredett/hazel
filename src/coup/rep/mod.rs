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
mod debug;
mod move_type;

#[cfg(test)] mod tests;

pub use move_type::*;
pub use bitmangling::*;

impl Move {

    /// Disambiguates the move in the context of the provided query. If the move is not marked ambiguous,
    /// the move is returned as is. If the move is ambiguous, the context is used to determine the
    /// correct metadata for the move. No effort is made to ensure legality of the move.
    ///
    /// TODO: Does not look for check states.
    #[instrument(skip(context))]
    pub fn disambiguate<C>(&self, context: &C) -> Option<MoveType> where C: Query {
        // If we are not ambiguous, just return the move as is.
        if !self.is_ambiguous() { return Some(self.move_metadata()); }

        let source = context.get(self.source());
        let target = context.get(self.target());

        // If the source square is empty, we can't disambiguate
        if source.is_empty() { return None; }

        let capturing = !target.is_empty(); // If there is a piece on the target square, then we're capturing

        match source.piece().unwrap() {
            Piece::Pawn => {
                // we might still be capturing en passant, we can check to see if we're moving
                // diagonally. This can be done by checking the difference between the source
                // and target. We can also determine color here. -- if source > target, then
                // we're moving black pieces.
                let delta = self.target_idx() as isize - self.source_idx() as isize;

                if delta.abs() == 0o20 {
                    // now we can also check for a double-pawn move. the delta is just 2 column
                    // moves, which is 0o20, or a <2,0> vector, if you like.
                    return Some(MoveType::DOUBLE_PAWN);
                } else if delta.abs() == 0o11 {
                    // This implies `capturing`. Since a diagonal move is always a capture for a
                    // pawn. However, `capturing` may be unset if the move is `en passant`.
                    //
                    if !capturing {
                        // So if the capture flag is unset, but we are doing a capture move, it
                        // must be EP_CAPTURE.
                        return Some(MoveType::EP_CAPTURE);
                    } else if self.target().backrank() {
                        // If we are moving to the backrank, and we are capturing, we must be capture-promoting
                        // BUG: This doesn't let you promote to anything other than a queen, and
                        // in fact it is known that some positions promotion to a non-queen is the
                        // only way to avoid a stalemate, so this is a bug.
                        return Some(MoveType::PROMOTION_CAPTURE_QUEEN);
                    } else {
                        // Otherwise, we are just capturing as normal
                        return Some(MoveType::CAPTURE);
                    }
                } else if self.target().backrank() {
                    return Some(MoveType::PROMOTION_QUEEN);
                } else {
                    // No capture, no double-pawn, no promotion, no en passant, just a quiet move.
                    return Some(MoveType::QUIET);
                }
            },
            Piece::King => {
                // Castling is a king move in UCI, so it's a king move as far as I'm concerned.
                match self.source() {
                    E1 => {
                        if self.target() == G1 {
                            return Some(MoveType::SHORT_CASTLE);
                        } else if self.target() == C1 {
                            return Some(MoveType::LONG_CASTLE);
                        }
                    },
                    E8 => {
                        if self.target() == G8 {
                            return Some(MoveType::SHORT_CASTLE);
                        } else if self.target() == C8 {
                            return Some(MoveType::LONG_CASTLE);
                        }
                    },
                    _ => { },
                }
            },
            _ => { },
        };
        // Otherwise, moves are just captures or quiet, simple as.
        if capturing {
            return Some(MoveType::CAPTURE);
        } else {
            return Some(MoveType::QUIET);
        }
    }

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



    #[instrument(skip(context))]
    pub fn compile<C>(&self, context: &C) -> Vec<Alteration> where C : Query {
        let source = self.source();
        let target = self.target();

        let source_occupant = context.get(source);
        let target_occupant = context.get(target);

        let contextprime = self.disambiguate(context).unwrap();

        if let Some(metadata) = context.try_metadata() {
            
        }


        let alterations = match contextprime {
            MoveType::QUIET => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, source_occupant),
            ],
            MoveType::DOUBLE_PAWN => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, source_occupant),
            ],
            MoveType::SHORT_CASTLE => {
                let color = source_occupant.color().unwrap();
                let rook_source = match color {
                    Color::WHITE => H1,
                    Color::BLACK => H8,
                };
                let rook_target = match color {
                    Color::WHITE => F1,
                    Color::BLACK => F8,
                };
                vec![
                    Alteration::remove(rook_source, Occupant::rook(color)),
                    Alteration::remove(source, source_occupant),
                    Alteration::place(target, source_occupant),
                    Alteration::place(rook_target, Occupant::rook(color)),
                ]
            },
            MoveType::LONG_CASTLE => { 
                let color = source_occupant.color().unwrap();
                let rook_source = match color {
                    Color::WHITE => A1,
                    Color::BLACK => A8
                };
                let rook_target = match color {
                    Color::WHITE => D1,
                    Color::BLACK => D8
                };
                vec![
                    // remove the rook
                    Alteration::remove(rook_source, Occupant::rook(color)),
                    // remove the king
                    Alteration::remove(source, source_occupant),
                    // place the king
                    Alteration::place(target, source_occupant),
                    // place the rook
                    Alteration::place(rook_target, Occupant::rook(color))
                ]
            },
            MoveType::CAPTURE => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, source_occupant),
            ],
            MoveType::EP_CAPTURE => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, source_occupant),
            ],
            MoveType::PROMOTION_KNIGHT => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, Occupant::knight(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_BISHOP => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, Occupant::bishop(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_ROOK => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, Occupant::rook(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_QUEEN => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, Occupant::queen(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_CAPTURE_KNIGHT => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, Occupant::knight(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_CAPTURE_BISHOP => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, Occupant::bishop(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_CAPTURE_ROOK => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, Occupant::rook(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_CAPTURE_QUEEN => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, Occupant::queen(source_occupant.color().unwrap())),
            ],
            MoveType::NULLMOVE => vec![],
            _ => { unreachable!(); }
        };

        return alterations;
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

    /// This checks that the move is a valid short castle for the color, assuming the piece at the
    /// source square is a king and that there are no castle blocks.
    /// FIXME: Similar to above, this could catch a move from, e.g., e1g1 of a queen or rook.
    #[inline(always)]
    pub fn is_short_castling_move_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => self.source() == E1 && self.target() == G1,
            Color::BLACK => self.source() == E8 && self.target() == G8,
        }
    }

    /// This checks that the move is a valid long castle for the color, assuming the piece at the
    /// source square is a king and that there are no castle blocks.
    #[inline(always)]
    /// FIXME: Similar to above, this could catch a move from, e.g., e1g1 of a queen or rook.
    pub fn is_long_castling_move_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => self.source() == E1 && self.target() == C1,
            Color::BLACK => self.source() == E8 && self.target() == C8,
        }
    }
}
