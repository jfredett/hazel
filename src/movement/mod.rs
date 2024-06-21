#![allow(non_snake_case)]

use crate::constants::{Color, Piece, NOTATION_TO_INDEX};
use serde::{Deserialize, Serialize};

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
    pub fn empty() -> Move {
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
    pub fn from(source: u16, target: u16, metadata: MoveType) -> Move {
        #[rustfmt::skip] Move {
            0: source << SOURCE_IDX_SHIFT
             | target << TARGET_IDX_SHIFT
             | metadata as u16,
        }
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
    /// let m = Move::from_notation("d2", "d4", Either::Left(MoveType::quiet()));
    ///
    /// assert_eq!(m.source_idx(), 0o13);
    /// assert_eq!(m.target_idx(), 0o33);
    /// assert!(!m.is_promotion());
    /// assert!(m.move_metadata().is_quiet());
    ///
    /// let pm = Move::from_notation("d7", "d8", Either::Right(Piece::Queen));
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
    /// let m = Move::from(0o13, 0o33, false, 0o00);
    /// assert_eq!(m.source_idx(), 0o13);
    /// ```
    pub fn source_idx(&self) -> usize {
        ((self.0 & SOURCE_IDX_MASK) >> SOURCE_IDX_SHIFT).into()
    }

    /// Gets the target index from the compact move representation
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, false, 0o00);
    /// assert_eq!(m.target_idx(), 0o33);
    /// ```
    pub fn target_idx(&self) -> usize {
        ((self.0 & TARGET_IDX_MASK) >> TARGET_IDX_SHIFT).into()
    }

    /// True if the move indicates a promotion
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, false, 0b000);
    /// let m2 = Move::from(0o63, 0o73, true, 0b011);
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
}
