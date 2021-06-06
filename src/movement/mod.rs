#![allow(non_snake_case)]

use crate::constants::{NOTATION_TO_INDEX, Piece};

///! This module defines a compact representation of chess moves from a given ply.
///!
///! Note on the name of this module. Ideally, this would be named 'move', like the struct it
///! defines, but alas, we are limited by rust reserving the `move` keyword for silly things like
///! memory safety or something.
///!

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Move(u16);

const SOURCE_IDX_MASK   : u16   = 0b111111_000000_0_000;
const SOURCE_IDX_SHIFT  : usize = 10;
const TARGET_IDX_MASK   : u16   = 0b000000_111111_0_000;
const TARGET_IDX_SHIFT  : usize = 4;
const PROMOTE_BIT_MASK  : u16   = 0b000000_000000_1_000;
const PROMOTE_BIT_SHIFT : usize = 3;
const METADATA_MASK     : u16   = 0b000000_000000_0_111;

mod move_type;

use either::Either;
pub use move_type::*;


impl Move {
    pub fn empty() -> Move { Move { 0: 0 } }

    /// Creates a move from a given source and target index, 
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, false, 0b000);
    /// assert_eq!(m.source_idx(), 0o13);
    /// assert_eq!(m.target_idx(), 0o33);
    /// assert!(!m.is_promotion());
    /// assert!(m.move_metadata().is_quiet());
    /// ```
    pub fn from(source: u16, target: u16, is_promotion: bool, metadata: u16) -> Move { 
        let is_promote = if is_promotion { 1 } else { 0 };
        let move_val = source << SOURCE_IDX_SHIFT
                     | target << TARGET_IDX_SHIFT
                     | is_promote << PROMOTE_BIT_SHIFT
                     | metadata;
        Move { 0: 
            move_val 
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
    pub fn from_notation(source: &str, target: &str, metadata: Either<MoveType, Piece>) -> Move {
        match metadata {
            Either::Left(mt) => { 
                Move::from(
                    NOTATION_TO_INDEX(source) as u16, 
                    NOTATION_TO_INDEX(target) as u16,
                    false,
                    mt.bits()
                )
            },
            Either::Right(p) => {
                Move::from(
                    NOTATION_TO_INDEX(source) as u16, 
                    NOTATION_TO_INDEX(target) as u16,
                    true,
                    p as u16
                )
            }
        }
    }
    
    /// Gets the source index from the compact move representation
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, false, 0o00);
    /// assert_eq!(m.source_idx(), 0o13);
    /// ```
    pub fn source_idx(&self) -> u16 { (self.0 & SOURCE_IDX_MASK) >> SOURCE_IDX_SHIFT }
    /// Gets the target index from the compact move representation
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m = Move::from(0o13, 0o33, false, 0o00);
    /// assert_eq!(m.target_idx(), 0o33);
    /// ```
    pub fn target_idx(&self) -> u16 { (self.0 & TARGET_IDX_MASK) >> TARGET_IDX_SHIFT }
    /// True if the move indicates a promotion
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, false, 0b000);
    /// let m2 = Move::from(0o63, 0o73, true, 0b011);
    /// assert!(!m1.is_promotion());
    /// assert!(m2.is_promotion());
    /// ```
    pub fn is_promotion(&self) -> bool { (self.0 & PROMOTE_BIT_MASK) > 0 }
    /// Calculates the promotion piece is there is a promotion to be done.
    /// NOTE: Will return garbage for non-promotion moves. No checking is done ahead of time.
    /// ```
    /// # use hazel::movement::*;
    /// # use hazel::constants::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, false, 0b000);
    /// let m2 = Move::from(0o63, 0o73, true, 0b011);
    /// // assert!(m1.promotion_piece()); DON'T DO THIS! It's not a promotion so this is misinterpreting the union type.
    /// assert_eq!(m2.promotion_piece(), Piece::Queen);
    /// ```
    pub fn promotion_piece(&self) -> Piece { Piece::from(self.0 & METADATA_MASK) }
    /// Interprets the metadata bits when the piece is not a promotion. Use the provided `is_` functions
    /// on MoveType to interpret the data.
    /// ```
    /// # use hazel::movement::*;
    /// // the move from d2 -> d4
    /// let m1 = Move::from(0o13, 0o33, false, 0b000);
    /// let m2 = Move::from(0o13, 0o33, false, 0b100);
    /// assert!(m1.move_metadata().is_quiet());
    /// assert!(m2.move_metadata().is_check());
    /// ```
    pub fn move_metadata(&self) -> MoveType { MoveType::from_bits(self.0 & METADATA_MASK).unwrap() }
    
    // Some proxy methods
    #[inline(always)] pub fn is_check(&self)   -> bool { self.move_metadata().is_check() }
    #[inline(always)] pub fn is_capture(&self) -> bool { self.move_metadata().is_capture() }
    #[inline(always)] pub fn is_attack(&self)  -> bool { self.move_metadata().is_attack() }
    #[inline(always)] pub fn is_quiet(&self)   -> bool { self.move_metadata().is_quiet() }
}



#[cfg(test)]
mod test {
    use super::*;

        /// let m = Move::from_notation("d2", "d4", Either::Left(MoveType::quiet()));
    ///
    /// assert_eq!(m.source_idx(), 0o13);
    /// assert_eq!(m.target_idx(), 0o33);
    /// assert!(!m.is_promotion());
    /// assert!(m.move_metadata().is_quiet());
    ///
    /// let pm = Move::from_notation("d7", "d8", Either::Right(Piece::Queen));
    /// assert_eq!(m.source_idx(), 0o63);
    /// assert_eq!(m.target_idx(), 0o73);
    /// assert!(m.is_promotion());
    /// assert_eq!(m.promotion_piece(), Piece::Queen)
    
    mod from_notation {
        use super::*;

        #[test]
        fn quiet_move_parses_correctly() {
            let m = Move::from_notation("d2", "d4", Either::Left(MoveType::quiet()));

            assert_eq!(m.source_idx(), 0o13);
            assert_eq!(m.target_idx(), 0o33);
            assert!(!m.is_promotion());
            assert!(m.move_metadata().is_quiet());
        }
        
        #[test]
        fn promotion_move_parses_correctly() {
            let pm = Move::from_notation("d7", "d8", Either::Right(Piece::Queen));
            assert_eq!(pm.source_idx(), 0o63);
            assert_eq!(pm.target_idx(), 0o73);
            assert!(pm.is_promotion());
            assert_eq!(pm.promotion_piece(), Piece::Queen)
        }

    }
}