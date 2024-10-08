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


use crate::{constants::{Color, File, Piece, INDEX_TO_NOTATION, NOTATION_TO_INDEX}, movegen::Alteration};
use crate::board::{display_board, Query};
use crate::board::occupant::Occupant;
use serde::{Deserialize, Serialize};

use tracing::instrument;
use tracing::debug;

///! This module defines a compact representation of chess moves from a given ply.
///!
///! NOTE: With respect to the name of this module. Ideally, this would be named 'move', like the
///! struct it ! defines, but alas, we are limited by rust reserving the `move` keyword for silly
///! things like memory safety or something.
///!
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct Move(pub(crate) u16);

#[rustfmt::skip] const SOURCE_IDX_MASK   : u16   = 0b111111_000000_0_000;
#[rustfmt::skip] const SOURCE_IDX_SHIFT  : usize = 10;
#[rustfmt::skip] const TARGET_IDX_MASK   : u16   = 0b000000_111111_0_000;
#[rustfmt::skip] const TARGET_IDX_SHIFT  : usize = 4;
#[rustfmt::skip] const METADATA_MASK     : u16   = 0b000000_000000_1_111;

mod debug;
mod generator;
mod move_type;



pub use move_type::*;

impl Move {
    pub const fn empty() -> Move {
        Move { 0: 0 }
    }

    /// Creates a move from a given source and target index,
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, MoveType::QUIET);
    /// assert_eq!(m.source_idx(), 0o13);
    /// assert_eq!(m.target_idx(), 0o33);
    /// assert!(!m.is_promotion());
    /// assert!(m.move_metadata().is_quiet());
    /// ```
    pub const fn from(source: u16, target: u16, metadata: MoveType) -> Move {
        #[rustfmt::skip] Move {
            0: source << SOURCE_IDX_SHIFT
            | target << TARGET_IDX_SHIFT
            | metadata as u16,
        }
    }

    pub const fn null() -> Move {
        Move::from(0,0, MoveType::NULLMOVE)
    }

    pub fn from_uci(uci: &str) -> Move {
        if uci == "0000" {
            return Move::null();
        }
        let source = NOTATION_TO_INDEX(&uci[0..2]) as u16;
        let target = NOTATION_TO_INDEX(&uci[2..4]) as u16;
        // NOTE: Without context, we cannot determine, e.g., if `d2d4` is a double pawn move, or a
        // move of a rook, or a move of a bishop. It depends on the piece at d2, and we don't know
        // that. So instead we mark the move 'ambiguous' and defer disambiguation till later, when
        // we explicitly ask for a context.
        let metadata = MoveType::UCI_AMBIGUOUS;
        Move::from(source, target, metadata)
    }

    /// Creates a move from the given source and target squares (given in notation), and
    /// the provided metadata. If a Right(Piece) is provided, the move is assumed to be a
    /// valid promotion. No error checking is done.
    ///
    /// NOTE: do not use this internally, this is for testing convenience!
    /// ```
    /// # use hazel::movement::*;
    /// # use hazel::constants::*;
    /// # use either::Either;
    /// // the move from d2 -> d4
    /// let m = Move::from_notation("d2", "d4", MoveType::DOUBLE_PAWN);
    ///
    /// assert_eq!(m.source_idx(), 0o13);
    /// assert_eq!(m.target_idx(), 0o33);
    /// assert!(!m.is_promotion());
    ///
    /// let pm = Move::from_notation("d7", "d8", MoveType::PROMOTION_QUEEN);
    /// assert_eq!(pm.source_idx(), 0o63);
    /// assert_eq!(pm.target_idx(), 0o73);
    /// assert!(pm.is_promotion());
    /// assert_eq!(pm.promotion_piece(), Piece::Queen);
    /// ```
    pub fn from_notation(source: &str, target: &str, metadata: MoveType) -> Move {
        Move::from(
            NOTATION_TO_INDEX(source) as u16,
            NOTATION_TO_INDEX(target) as u16,
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


        let source = context.get(self.source_idx());
        let target = context.get(self.target_idx());

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
                    } else if INDEX_TO_NOTATION[self.target_idx()].ends_with("8") || INDEX_TO_NOTATION[self.target_idx()].ends_with("1") {
                        // If we are moving to the backrank, and we are capturing, we must be capture-promoting
                        return Some(MoveType::PROMOTION_CAPTURE_QUEEN);
                    } else {
                        // Otherwise, we are just capturing as normal
                        return Some(MoveType::CAPTURE);
                    }
                } else {
                    if INDEX_TO_NOTATION[self.target_idx()].ends_with("8") || INDEX_TO_NOTATION[self.target_idx()].ends_with("1") {
                        return Some(MoveType::PROMOTION_QUEEN);
                    } else {
                        // No capture, no double-pawn, no promotion, no en passant, just a quiet move.
                        return Some(MoveType::QUIET);
                    }
                }
            },
            Piece::King => {
                // Castling is a king move in UCI, so it's a king move as far as I'm concerned.
                match self.source_idx() {
                    0o04 => {
                        if self.target_idx() == 0o06 {
                            return Some(MoveType::SHORT_CASTLE);
                        } else if self.target_idx() == 0o02 {
                            return Some(MoveType::LONG_CASTLE);
                        }
                    },
                    0o74 => {
                        if self.target_idx() == 0o76 {
                            return Some(MoveType::SHORT_CASTLE);
                        } else if self.target_idx() == 0o72 {
                            return Some(MoveType::LONG_CASTLE);
                        }
                    },
                    _ => { },
                }
            },
            _ => {
            },
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
        debug!("Generating PGN for move: {:?}", self);
        debug!("Context: {}", display_board(context));
        if self.is_null() {
            return "".to_string();
        }

        let metadata = self.disambiguate(context).unwrap();
        debug!("Disambiguated metadata: {:?}", metadata);

        if metadata.is_short_castle() {
            return "O-O".to_string();
        } else if metadata.is_long_castle() {
            return "O-O-O".to_string();
        }

        let mut result = String::new();

        let source_idx = self.source_idx();
        let target_idx = self.target_idx();

        let source = context.get(source_idx);

        let source_file = File::from_index(source_idx).to_pgn();

        result.push_str(match source.piece().unwrap() {
            Piece::Pawn => if metadata.is_capture() { source_file } else { "" },
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Rook => "R",
            Piece::Queen => "Q",
            Piece::King => "K",
        });

        if metadata.is_capture() {
            result.push_str("x");
        }

        result.push_str(&INDEX_TO_NOTATION[target_idx]);

        if metadata.is_promotion() {
            result.push_str("=");
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
                NOTATION_TO_INDEX("e1") as u16,
                NOTATION_TO_INDEX("c1") as u16,
                MoveType::LONG_CASTLE,
            ),
            Color::BLACK => Move::from(
                NOTATION_TO_INDEX("e8") as u16,
                NOTATION_TO_INDEX("c8") as u16,
                MoveType::LONG_CASTLE,
            ),
        }
    }

    pub fn short_castle(color: Color) -> Move {
        match color {
            Color::WHITE => Move::from(
                NOTATION_TO_INDEX("e1") as u16,
                NOTATION_TO_INDEX("g1") as u16,
                MoveType::SHORT_CASTLE,
            ),
            Color::BLACK => Move::from(
                NOTATION_TO_INDEX("e8") as u16,
                NOTATION_TO_INDEX("g8") as u16,
                MoveType::SHORT_CASTLE,
            ),
        }
    }

    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    ///
    /// let m = Move::from(0o13, 0o33, MoveType::DOUBLE_PAWN);
    /// assert_eq!(m.source_idx(), 0o13);
    /// ```
    pub fn source_idx(&self) -> usize {
        ((self.0 & SOURCE_IDX_MASK) >> SOURCE_IDX_SHIFT).into()
    }

    /// Gets the target index from the compact move representation
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, MoveType::DOUBLE_PAWN);
    /// assert_eq!(m.target_idx(), 0o33);
    /// ```
    pub fn target_idx(&self) -> usize {
        ((self.0 & TARGET_IDX_MASK) >> TARGET_IDX_SHIFT).into()
    }

    /// True if the move indicates a promotion
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, MoveType::DOUBLE_PAWN);
    /// let m2 = Move::from(0o63, 0o73, MoveType::PROMOTION_QUEEN);
    /// assert!(!m1.is_promotion());
    /// assert!(m2.is_promotion());
    /// ```
    pub fn is_promotion(&self) -> bool {
        self.move_metadata().is_promotion()
    }

    /// Calculates the promotion piece is there is a promotion to be done.
    /// NOTE: Will return garbage for non-promotion moves. No checking is done ahead of time.
    /// ```
    /// # use hazel::movement::*;
    /// # use hazel::constants::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, MoveType::QUIET);
    /// let m2 = Move::from(0o63, 0o73, MoveType::PROMOTION_QUEEN);
    /// // assert!(m1.promotion_piece()); DON'T DO THIS! It's not a promotion so this is misinterpreting the union type.
    /// assert_eq!(m2.promotion_piece(), Piece::Queen);
    /// ```
    pub fn promotion_piece(&self) -> Piece {
        self.move_metadata().promotion_piece().unwrap()
    }

    /// Interprets the metadata bits when the piece is not a promotion. Use the provided `is_` functions
    /// on MoveType to interpret the data.
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, MoveType::QUIET);
    /// assert!(m1.move_metadata().is_quiet());
    /// ```
    pub fn move_metadata(&self) -> MoveType {
        MoveType::new(self.0 & METADATA_MASK)
    }

    #[instrument(skip(context))]
    pub fn compile<C>(&self, context: &C) -> Vec<Alteration> where C : Query {
        let source = self.source_idx();
        let target = self.target_idx();

        let source_occupant = context.get(source);
        let target_occupant = context.get(target);

        let mut alterations = match self.disambiguate(context).unwrap() {
            MoveType::QUIET => vec![
                Alteration::place(target, source_occupant),
                Alteration::remove(source, source_occupant)
            ],
            MoveType::DOUBLE_PAWN => vec![
                Alteration::place(target, source_occupant),
                Alteration::remove(source, source_occupant)
            ],
            MoveType::SHORT_CASTLE => {
                let color = source_occupant.color().unwrap();
                let rook_source_idx = match color {
                    Color::WHITE => NOTATION_TO_INDEX("h1"),
                    Color::BLACK => NOTATION_TO_INDEX("h8"),
                };
                let rook_target_idx = match color {
                    Color::WHITE => NOTATION_TO_INDEX("f1"),
                    Color::BLACK => NOTATION_TO_INDEX("f8"),
                };
                return vec![
                    // remove the rook
                    Alteration::remove(rook_source_idx, Occupant::rook(color)),
                    // remove the king
                    Alteration::remove(source, source_occupant),
                    // place the king
                    Alteration::place(target, source_occupant),
                    // place the rook
                    Alteration::place(rook_target_idx, Occupant::rook(color))
                ];
            },
            MoveType::LONG_CASTLE => { 
                let color = source_occupant.color().unwrap();
                let rook_source_idx = match color {
                    Color::WHITE => NOTATION_TO_INDEX("a1"),
                    Color::BLACK => NOTATION_TO_INDEX("a8"),
                };
                let rook_target_idx = match color {
                    Color::WHITE => NOTATION_TO_INDEX("d1"),
                    Color::BLACK => NOTATION_TO_INDEX("d8"),
                };
                return vec![
                    // remove the rook
                    Alteration::remove(rook_source_idx, Occupant::rook(color)),
                    // remove the king
                    Alteration::remove(source, source_occupant),
                    // place the king
                    Alteration::place(target, source_occupant),
                    // place the rook
                    Alteration::place(rook_target_idx, Occupant::rook(color))
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
            ],
            MoveType::PROMOTION_BISHOP => vec![
            ],
            MoveType::PROMOTION_ROOK => vec![
            ],
            MoveType::PROMOTION_QUEEN => vec![
            ],
            MoveType::PROMOTION_CAPTURE_KNIGHT => vec![
            ],
            MoveType::PROMOTION_CAPTURE_BISHOP => vec![
            ],
            MoveType::PROMOTION_CAPTURE_ROOK => vec![
            ],
            MoveType::PROMOTION_CAPTURE_QUEEN => vec![
            ],
            _ => todo!()
        };

        alterations.push(Alteration::comment(self.to_uci()));
        alterations.push(Alteration::done());

        return alterations;
    }


    pub fn to_uci(&self) -> String {
        let source = self.source_idx();
        let target = self.target_idx();
        let metadata = self.move_metadata();
        if metadata.is_null() {
            return "0000".to_string();
        } else {
            format!("{}{}{}", INDEX_TO_NOTATION[source], INDEX_TO_NOTATION[target], metadata.to_uci())
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
    #[inline(always)]
    pub fn is_double_pawn_push_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => { self.target_idx() - self.source_idx() == 0o20 && self.source_idx() & 0o70 == 0o10 },
            Color::BLACK => { self.source_idx() - self.target_idx() == 0o20 && self.source_idx() & 0o70 == 0o60 },
        }
    }

    /// This checks that the move is a valid short castle for the color, assuming the piece at the
    /// source square is a king and that there are no castle blocks.
    #[inline(always)]
    pub fn is_short_castling_move_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => self.source_idx() == 0o04 && self.target_idx() == 0o06,
            Color::BLACK => self.source_idx() == 0o74 && self.target_idx() == 0o76,
        }
    }

    /// This checks that the move is a valid long castle for the color, assuming the piece at the
    /// source square is a king and that there are no castle blocks.
    #[inline(always)]
    pub fn is_long_castling_move_for(&self, color: Color) -> bool {
        match color {
            Color::WHITE => self.source_idx() == 0o04 && self.target_idx() == 0o02,
            Color::BLACK => self.source_idx() == 0o74 && self.target_idx() == 0o72,
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
            let m = Move::from(0o13, 0o33, MoveType::CAPTURE);
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
            let m = Move::from(0o13, 0o23, MoveType::EP_CAPTURE);
            assert!(m.is_en_passant());
        }

        #[test]
        fn is_double_pawn_push_for() {
            let m = Move::from(0o13, 0o33, MoveType::DOUBLE_PAWN);
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
