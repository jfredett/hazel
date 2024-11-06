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


use crate::board::interface::{Alteration, Query};
use crate::notation::*;
use crate::types::{Color, Piece, Occupant};
use crate::constants::File;

use serde::{Deserialize, Serialize};

use tracing::instrument;
use tracing::trace;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct Move(pub(crate) u16);

#[rustfmt::skip] const SOURCE_IDX_MASK   : u16   = 0b111111_000000_0_000;
#[rustfmt::skip] const SOURCE_IDX_SHIFT  : usize = 10;
#[rustfmt::skip] const TARGET_IDX_MASK   : u16   = 0b000000_111111_0_000;
#[rustfmt::skip] const TARGET_IDX_SHIFT  : usize = 4;
#[rustfmt::skip] const METADATA_MASK     : u16   = 0b000000_000000_1_111;

mod debug;
mod move_type;

pub use move_type::*;

impl Move {
    pub const fn empty() -> Move {
        Move (0)
    }

    /// Creates a move from a given source and target index,
    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::notation::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(D2, D4, MoveType::QUIET);
    /// assert_eq!(m.source(), D2);
    /// assert_eq!(m.target(), D4);
    /// assert!(!m.is_promotion());
    /// assert!(m.move_metadata().is_quiet());
    /// ```
    pub fn new(source: impl Into<Square>, target: impl Into<Square>, metadata: MoveType) -> Move {
        let s : Square = source.into();
        let t : Square = target.into();

        #[rustfmt::skip] Move( (s.index() as u16) << SOURCE_IDX_SHIFT
                             | (t.index() as u16) << TARGET_IDX_SHIFT
                             |   metadata as u16 )
    }

    pub fn from(source: impl Into<Square>, target: impl Into<Square>, metadata: MoveType) -> Move {
        trace!("Deprecated use of Move::from, use Move::new instead");
        Move::new(
            source,
            target,
            metadata
        )
    }

    pub fn null() -> Move {
        // We only care about the metadata bits for a null move. So the source/target are just
        // whatever is convenient.
        Move::from(A1, A1, MoveType::NULLMOVE)
    }

    /// Creates a move from the given source and target squares (given in notation), and
    /// the provided metadata. If a Right(Piece) is provided, the move is assumed to be a
    /// valid promotion. No error checking is done.
    ///
    /// NOTE: do not use this internally, this is for testing convenience!
    ///
    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::notation::*;
    /// # use hazel::constants::*;
    /// # use hazel::types::Piece;
    /// # use either::Either;
    /// // the move from d2 -> d4
    /// let m = Move::from_notation("d2", "d4", MoveType::DOUBLE_PAWN);
    ///
    /// assert_eq!(m.source(), D2);
    /// assert_eq!(m.target(), D4);
    /// assert!(!m.is_promotion());
    ///
    /// let pm = Move::from_notation("d7", "d8", MoveType::PROMOTION_QUEEN);
    /// assert_eq!(pm.source(), D7);
    /// assert_eq!(pm.target(), D8);
    /// assert!(pm.is_promotion());
    /// assert_eq!(pm.promotion_piece(), Piece::Queen);
    /// ```
    pub fn from_notation(source: &str, target: &str, metadata: MoveType) -> Move {
        Move::from(
            Square::try_from(source).unwrap(),
            Square::try_from(target).unwrap(),
            metadata,
        )
    }

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
                    A5 => {
                        if self.target() == A7 {
                            return Some(MoveType::SHORT_CASTLE);
                        } else if self.target() == A3 {
                            return Some(MoveType::LONG_CASTLE);
                        }
                    },
                    H5 => {
                        if self.target() == H7 {
                            return Some(MoveType::SHORT_CASTLE);
                        } else if self.target() == H3 {
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

        let mut result = String::new();

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
            Color::WHITE => Move::from(
                E1,
                C1,
                MoveType::LONG_CASTLE,
            ),
            Color::BLACK => Move::from(
                E8,
                C8,
                MoveType::LONG_CASTLE,
            ),
        }
    }

    pub fn short_castle(color: Color) -> Move {
        match color {
            Color::WHITE => Move::from(
                E1,
                G1,
                MoveType::SHORT_CASTLE,
            ),
            Color::BLACK => Move::from(
                E8,
                G8,
                MoveType::SHORT_CASTLE,
            ),
        }
    }

    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::notation::*;
    /// // the move from d2 -> d4
    ///
    /// let m = Move::from(D2, D4, MoveType::DOUBLE_PAWN);
    /// assert_eq!(m.source(), D2);
    /// ```
    pub fn source_idx(&self) -> usize {
        ((self.0 & SOURCE_IDX_MASK) >> SOURCE_IDX_SHIFT).into()
    }

    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::notation::*;
    /// // the move from d2 -> d4
    ///
    /// let m = Move::from(D2, D4, MoveType::DOUBLE_PAWN);
    /// assert_eq!(m.source(), D2);
    pub fn source(&self) -> Square {
        self.source_idx().try_into().unwrap()
    }

    /// Gets the target index from the compact move representation
    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::notation::*;
    ///
    /// // the move from d2 -> d4
    ///
    /// let m = Move::from(D2, D4, MoveType::DOUBLE_PAWN);
    ///
    /// assert_eq!(m.target_idx(), usize::from(D4));
    /// ```
    pub fn target_idx(&self) -> usize {
        ((self.0 & TARGET_IDX_MASK) >> TARGET_IDX_SHIFT).into()
    }

    /// Gets the target index from the compact move representation
    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::notation::*;
    ///
    /// // the move from d2 -> d4
    ///
    /// let m = Move::from(D2, D4, MoveType::DOUBLE_PAWN);
    ///
    /// assert_eq!(m.target(), D4);
    /// ```
    pub fn target(&self) -> Square {
        self.target_idx().try_into().unwrap()
    }

    /// True if the move indicates a promotion
    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::notation::*;
    ///
    /// // the move from d2 -> d4
    /// let m1 = Move::from(D2, D4, MoveType::DOUBLE_PAWN);
    /// let m2 = Move::from(D7, D8, MoveType::PROMOTION_QUEEN);
    /// assert!(!m1.is_promotion());
    /// assert!(m2.is_promotion());
    /// ```
    pub fn is_promotion(&self) -> bool {
        self.move_metadata().is_promotion()
    }

    /// Calculates the promotion piece is there is a promotion to be done.
    ///
    /// NOTE: Will return garbage for non-promotion moves. No checking is done ahead of time.
    ///
    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::constants::*;
    /// # use hazel::notation::*;
    /// # use hazel::types::Piece;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(D2, D4, MoveType::DOUBLE_PAWN);
    /// let m2 = Move::from(D7, D8, MoveType::PROMOTION_QUEEN);
    /// // assert!(m1.promotion_piece()); DON'T DO THIS! It's not a promotion so this is misinterpreting the union type.
    /// assert_eq!(m2.promotion_piece(), Piece::Queen);
    /// ```
    pub fn promotion_piece(&self) -> Piece {
        self.move_metadata().promotion_piece().unwrap()
    }

    /// Interprets the metadata bits when the piece is not a promotion. Use the provided `is_` functions
    /// on MoveType to interpret the data.
    /// ```
    /// # use hazel::coup::rep::*;
    /// # use hazel::notation::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(D2, D4, MoveType::QUIET);
    /// assert!(m1.move_metadata().is_quiet());
    /// ```
    pub fn move_metadata(&self) -> MoveType {
        MoveType::new(self.0 & METADATA_MASK)
    }

    #[instrument(skip(context))]
    pub fn compile<C>(&self, context: &C) -> Vec<Alteration> where C : Query {
        let source = self.source();
        let target = self.target();

        let source_occupant = context.get(source);
        let target_occupant = context.get(target);

        let contextprime = self.disambiguate(context).unwrap();


        let mut alterations = match contextprime {
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
                return vec![
                    Alteration::remove(rook_source, Occupant::rook(color)),
                    Alteration::remove(source, source_occupant),
                    Alteration::place(target, source_occupant),
                    Alteration::place(rook_target, Occupant::rook(color))
                    // TODO: Track Metadata here? I'm really starting to think I should.
                ];
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
                return vec![
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

        alterations.push(Alteration::tag(contextprime as u8));

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
    // TODO: Maybe move metadata to a simple enum we can just match on, would make the #make/unmake implementations nicer
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

#[cfg(test)]
mod test {
    use super::*;

    mod from_notation {
        use super::*;

        #[test]
        fn quiet_move_parses_correctly() {
            let m = Move::from_notation("d2", "d4", MoveType::QUIET);

            assert_eq!(m.source_idx(), 0o13);
            assert_eq!(m.target_idx(), 0o33);
            assert!(!m.is_promotion());
            assert!(m.move_metadata().is_quiet());
        }

        #[test]
        fn promotion_move_parses_correctly() {
            let pm = Move::from_notation("d7", "d8", MoveType::PROMOTION_QUEEN);
            assert_eq!(pm.source_idx(), 0o63);
            assert_eq!(pm.target_idx(), 0o73);
            assert!(pm.is_promotion());
            assert_eq!(pm.promotion_piece(), Piece::Queen)
        }
    }

    mod castling {
        use super::*;

        #[test]
        fn short_castle_parses_correctly() {
            let m = Move::short_castle(Color::WHITE);
            assert_eq!(m.source_idx(), 0o04);
            assert_eq!(m.target_idx(), 0o06);
            assert!(m.is_short_castle());
        }

        #[test]
        fn long_castle_parses_correctly() {
            let m = Move::long_castle(Color::WHITE);
            assert_eq!(m.source_idx(), 0o04);
            assert_eq!(m.target_idx(), 0o02);
            assert!(m.is_long_castle());
        }
    }

    mod proxy_methods {
        use super::*;

        #[test]
        fn is_capture() {
            let m = Move::from(D2, D4, MoveType::CAPTURE);
            assert!(m.is_capture());
        }

        #[test]
        fn is_short_castle() {
            let m = Move::short_castle(Color::WHITE);
            assert!(m.is_short_castle());
        }

        #[test]
        fn is_long_castle() {
            let m = Move::long_castle(Color::WHITE);
            assert!(m.is_long_castle());
        }

        #[test]
        fn is_en_passant() {
            let m = Move::from(D6, E7, MoveType::EP_CAPTURE);
            assert!(m.is_en_passant());
        }

        #[test]
        fn is_double_pawn_push_for() {
            let m = Move::from(D2, D4, MoveType::DOUBLE_PAWN);
            assert!(m.is_double_pawn_push_for(Color::WHITE));
        }

        #[test]
        fn is_short_castling_move_for() {
            let m = Move::short_castle(Color::WHITE);
            assert!(m.is_short_castling_move_for(Color::WHITE));
        }

        #[test]
        fn is_long_castling_move_for() {
            let m = Move::long_castle(Color::WHITE);
            assert!(m.is_long_castling_move_for(Color::WHITE));
        }
    }
}
