use hazel_basic::square::Square;

use super::*;

#[rustfmt::skip] pub const SOURCE_IDX_MASK   : u16   = 0b111111_000000_0_000;
#[rustfmt::skip] pub const SOURCE_IDX_SHIFT  : usize = 10;
#[rustfmt::skip] pub const TARGET_IDX_MASK   : u16   = 0b000000_111111_0_000;
#[rustfmt::skip] pub const TARGET_IDX_SHIFT  : usize = 4;
#[rustfmt::skip] pub const METADATA_MASK     : u16   = 0b000000_000000_1_111;

impl Move {

    /// Set the metadata for the move to the given MoveType.
    pub fn set_metadata(&mut self, metadata: MoveType) {
        self.0 = (self.0 & !METADATA_MASK) | (metadata as u16);
    }

    /// ```
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    /// // the move from d2 -> d4
    ///
    /// let m = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
    /// assert_eq!(m.source(), D2);
    /// ```
    pub fn source_idx(&self) -> usize {
        ((self.0 & SOURCE_IDX_MASK) >> SOURCE_IDX_SHIFT).into()
    }

    /// ```
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    /// // the move from d2 -> d4
    ///
    /// let m = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
    /// assert_eq!(m.source(), D2);
    pub fn source(&self) -> Square {
        self.source_idx().try_into().unwrap()
    }

    /// Gets the target index from the compact move representation
    /// ```
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    ///
    /// // the move from d2 -> d4
    ///
    /// let m = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
    ///
    /// assert_eq!(m.target_idx(), usize::from(D4));
    /// ```
    pub fn target_idx(&self) -> usize {
        ((self.0 & TARGET_IDX_MASK) >> TARGET_IDX_SHIFT).into()
    }

    /// Gets the target index from the compact move representation
    /// ```
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    ///
    /// // the move from d2 -> d4
    ///
    /// let m = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
    ///
    /// assert_eq!(m.target(), D4);
    /// ```
    pub fn target(&self) -> Square {
        self.target_idx().try_into().unwrap()
    }

    /// True if the move indicates a promotion
    /// ```
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    ///
    /// // the move from d2 -> d4
    /// let m1 = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
    /// let m2 = Move::new(D7, D8, MoveType::PROMOTION_QUEEN);
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
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    /// # use hazel_basic::piece::Piece;
    ///
    /// // the move from d2 -> d4
    /// let m1 = Move::new(D2, D4, MoveType::DOUBLE_PAWN);
    /// let m2 = Move::new(D7, D8, MoveType::PROMOTION_QUEEN);
    /// // assert!(m1.promotion_piece()); DON'T DO THIS! It's not a promotion so this is misinterpreting the union type.
    /// assert_eq!(m2.promotion_piece(), Piece::Queen);
    /// ```
    pub fn promotion_piece(&self) -> Piece {
        self.move_metadata().promotion_piece().unwrap()
    }

    /// Interprets the metadata bits when the piece is not a promotion. Use the provided `is_` functions
    /// on MoveType to interpret the data.
    /// ```
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    /// # use hazel_basic::piece::Piece;
    ///
    /// // the move from d2 -> d4
    /// let m1 = Move::new(D2, D4, MoveType::QUIET);
    /// assert!(m1.move_metadata().is_quiet());
    /// ```
    pub fn move_metadata(&self) -> MoveType {
        MoveType::new(self.0 & METADATA_MASK)
    }


}
