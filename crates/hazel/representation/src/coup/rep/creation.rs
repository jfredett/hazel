use super::*;

impl Move {
    pub const fn empty() -> Move {
        Move (0)
    }

    /// Creates a move from a given source and target index,
    /// ```
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    /// // the move from d2 -> d4
    /// let m = Move::new(D2, D4, MoveType::QUIET);
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


    pub fn null() -> Move {
        // We only care about the metadata bits for a null move. So the source/target are just
        // whatever is convenient.
        Move::new(A1, A1, MoveType::NULLMOVE)
    }

    /// Creates a move from the given source and target squares (given in notation), and
    /// the provided metadata. If a Right(Piece) is provided, the move is assumed to be a
    /// valid promotion. No error checking is done.
    ///
    /// NOTE: do not use this internally, this is for testing convenience!
    ///
    /// ```
    /// # use hazel_representation::coup::rep::*;
    /// # use hazel_basic::square::*;
    /// # use hazel_basic::piece::Piece;
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
        Move::new(
            Square::try_from(source).unwrap(),
            Square::try_from(target).unwrap(),
            metadata,
        )
    }
}
