use std::fmt::Display;

use super::*;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum MoveMode {
    Make,
    Unmake,
}

impl Display for MoveMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveMode::Make => write!(f, "  Make"),
            MoveMode::Unmake => write!(f, "Unmake"),
        }
    }
}

pub type MoveResult<T> = Result<T, MoveError>;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, Error)]
pub enum MoveError {
    #[error("{0}: Move {1:?} has malformed metadata")]
    UnrecognizedMove(MoveMode, Move),
    #[error("{0}: Missing friendly piece at source index of {2:?} for board {1:?}")]
    MissingSourcePiece(MoveMode, Ply, Move),
    #[error("{0}: Missing enemy piece at source index of {2:?} for board {1:?}")]
    MissingTargetPiece(MoveMode, Ply, Move),
}

impl Ply {
    pub fn make(&mut self, mov: Move) -> MoveResult<Option<Piece>> {
        let source_piece = match self.friendly_piece_at_index(mov.source_idx()) {
            Some(s) => s,
            None => {
                return if mov.move_metadata().is_en_passant() {
                    Ok(Some(Piece::Pawn))
                } else {
                    Err(MoveError::MissingSourcePiece(MoveMode::Make, *self, mov))
                }
            }
        };
        let target_piece = self.enemy_piece_at_index(mov.target_idx());
        let mut clear_ep = true;

        let result: Option<Piece> = match mov.move_metadata() {
            MoveType::QUIET => {
                self.move_piece(source_piece, mov)?;
                None
            }
            MoveType::SHORT_CASTLE => {
                self.short_castle()?;
                None
            }
            MoveType::LONG_CASTLE => {
                self.long_castle()?;
                None
            }
            MoveType::CAPTURE => {
                match target_piece {
                    Some(t) => self.remove_enemy_piece(t, mov.target_idx())?,
                    None => return Err(MoveError::MissingTargetPiece(MoveMode::Make, *self, mov)),
                }
                self.move_piece(source_piece, mov)?;
                target_piece
            }
            MoveType::EP_CAPTURE => {
                self.remove_enemy_piece(
                    Piece::Pawn,
                    self.enemy_pawn_direction().index_shift(mov.target_idx()),
                )?;
                self.move_piece(Piece::Pawn, mov)?;
                Some(Piece::Pawn)
            }
            MoveType::PROMOTION_KNIGHT => {
                self.execute_promotion(mov, Piece::Knight)?;
                None
            }
            MoveType::PROMOTION_BISHOP => {
                self.execute_promotion(mov, Piece::Bishop)?;
                None
            }
            MoveType::PROMOTION_ROOK => {
                self.execute_promotion(mov, Piece::Rook)?;
                None
            }
            MoveType::PROMOTION_QUEEN => {
                self.execute_promotion(mov, Piece::Queen)?;
                None
            }
            MoveType::PROMOTION_CAPTURE_KNIGHT => {
                Some(self.execute_promotion_capture(mov, Piece::Knight)?)
            }
            MoveType::PROMOTION_CAPTURE_BISHOP => {
                Some(self.execute_promotion_capture(mov, Piece::Bishop)?)
            }
            MoveType::PROMOTION_CAPTURE_ROOK => {
                Some(self.execute_promotion_capture(mov, Piece::Rook)?)
            }
            MoveType::PROMOTION_CAPTURE_QUEEN => {
                Some(self.execute_promotion_capture(mov, Piece::Queen)?)
            }
            MoveType::DOUBLE_PAWN => {
                // move the piece
                self.move_piece(source_piece, mov)?;
                // set the en_passant square
                self.meta.set_en_passant(Some(mov.source_idx() % 8));
                clear_ep = false;
                None
            }
            _ => return Err(MoveError::UnrecognizedMove(MoveMode::Make, mov)),
        };

        if clear_ep {
            self.meta.set_en_passant(None);
        }

        self.tick(source_piece, result.is_some())?;

        Ok(result)
    }

    #[instrument(skip(self))]
    pub fn unmake(
        &mut self,
        mov: Move,
        captured_piece: Option<Piece>,
        metadata: Metadata,
    ) -> MoveResult<()> {
        // Untick _first_ so that the 'current_player' becomes the correct color.
        // TODO: half-move clock memory
        self.meta = metadata;

        // note how it's target -- this is _after_ the mov has been made, so we have to work backwards
        let source_piece = match self.friendly_piece_at_index(mov.target_idx()) {
            Some(s) => s,
            None => {
                if mov.is_en_passant() {
                    Piece::Pawn
                } else {
                    return Err(MoveError::MissingSourcePiece(MoveMode::Unmake, *self, mov))
                }
            }
        };

        match mov.move_metadata() {
            MoveType::QUIET => self.unmove_piece(source_piece, mov)?,
            MoveType::SHORT_CASTLE => self.unshort_castle()?,
            MoveType::LONG_CASTLE => self.unlong_castle()?,
            MoveType::CAPTURE => {
                match captured_piece {
                    Some(t) => self.place_enemy_piece(t, mov.target_idx())?,
                    None => {
                        return Err(MoveError::MissingTargetPiece(MoveMode::Unmake, *self, mov))
                    }
                }
                self.unmove_piece(source_piece, mov)?
            }
            MoveType::EP_CAPTURE => {
                //
                //  Consider the following case:
                //
                //
                //  PGN:
                //
                //  1. f4 Na6
                //  2. f5 g5
                //  (3. fxe6 ...)
                //
                // State after end of ply 2:
                //
                // 8 | r . b q k b n r
                // 7 | p p p p p p . p
                // 6 | n . . . . . . .
                // 5 | . . . . . P p .
                // 4 | . . . . . . . .
                // 3 | . . . . . . . .
                // 2 | P P P P P . P P
                // 1 | R N B Q K B N R
                //     a b c d e f g h
                //
                // State after ply 2.5:
                //
                // 8 | r . b q k b n r
                // 7 | p p p p p p . p
                // 6 | n . . . . . P .
                // 5 | . . . . . . . .
                // 4 | . . . . . . . .
                // 3 | . . . . . . . .
                // 2 | P P P P P . P P
                // 1 | R N B Q K B N R
                //     a b c d e f g h
                //
                // To undo from this state, I need to:
                //
                // 1. Uncapture the pawn on the target square
                // 2. Move the enemy pawn forward one space.
                //
                let target_idx = self.enemy_pawn_direction().index_shift(mov.target_idx());

                self.place_enemy_piece(Piece::Pawn, target_idx)?;
                self.unmove_piece(Piece::Pawn, mov)?;
            },
            MoveType::PROMOTION_KNIGHT => self.unexecute_promotion(mov, Piece::Knight)?,
            MoveType::PROMOTION_BISHOP => self.unexecute_promotion(mov, Piece::Bishop)?,
            MoveType::PROMOTION_ROOK => self.unexecute_promotion(mov, Piece::Rook)?,
            MoveType::PROMOTION_QUEEN => self.unexecute_promotion(mov, Piece::Queen)?,
            MoveType::PROMOTION_CAPTURE_KNIGHT => {
                self.unexecute_promotion_capture(mov, Piece::Knight, captured_piece)?
            }
            MoveType::PROMOTION_CAPTURE_BISHOP => {
                self.unexecute_promotion_capture(mov, Piece::Bishop, captured_piece)?
            }
            MoveType::PROMOTION_CAPTURE_ROOK => {
                self.unexecute_promotion_capture(mov, Piece::Rook, captured_piece)?
            }
            MoveType::PROMOTION_CAPTURE_QUEEN => {
                self.unexecute_promotion_capture(mov, Piece::Queen, captured_piece)?
            }
            MoveType::DOUBLE_PAWN => self.unmove_piece(source_piece, mov)?,
            _ => return Err(MoveError::UnrecognizedMove(MoveMode::Unmake, mov)),
        };

        Ok(())
    }

    pub fn make_by_notation(
        &mut self,
        source: &str,
        target: &str,
        metadata: MoveType,
    ) -> MoveResult<Option<Piece>> {
        self.make(Move::from(
            NOTATION_TO_INDEX(source) as u16,
            NOTATION_TO_INDEX(target) as u16,
            metadata,
        ))
    }

    pub fn unmake_by_notation(
        &mut self,
        source: &str,
        target: &str,
        metadata: MoveType,
        captured_piece: Option<Piece>,
        game_metadata: Metadata,
    ) -> MoveResult<()> {
        self.unmake(
            Move::from(
                NOTATION_TO_INDEX(source) as u16,
                NOTATION_TO_INDEX(target) as u16,
                metadata,
            ),
            captured_piece,
            game_metadata,
        )
    }

    pub fn pawn_direction(&self) -> Direction {
        Self::pawn_direction_for(self.current_player())
    }

    pub fn enemy_pawn_direction(&self) -> Direction {
        Self::pawn_direction_for(self.other_player())
    }

    fn pawn_direction_for(color: Color) -> Direction {
        match color {
            Color::WHITE => Direction::N,
            Color::BLACK => Direction::S,
        }
    }

    fn tick(&mut self, piece_moved: Piece, had_capture: bool) -> MoveResult<()> {
        // Half-move clock resets on capture or pawn move.
        self.meta.half_move_tick();
        match piece_moved {
            Piece::Rook => {
                let rooks =
                    self.rooks_for(self.current_player()) | self.rooks_for(self.other_player());
                // PEXT here using the CORNERS mask will map a1 -> bit 0, h1 -> bit 2, a8 -> bit 3, h8 -> bit 4.
                // If the bit is high, the rook is still in place, so we retain castling rights.
                match rooks.pext(*CORNERS) {
                    0b0000 => self.meta.rook_moved(true, true, true, true),
                    0b0001 => self.meta.rook_moved(false, true, true, true),
                    0b0010 => self.meta.rook_moved(true, false, true, true),
                    0b0011 => self.meta.rook_moved(false, false, true, true),
                    0b0100 => self.meta.rook_moved(true, true, false, true),
                    0b0101 => self.meta.rook_moved(false, true, false, true),
                    0b0110 => self.meta.rook_moved(true, false, false, true),
                    0b0111 => self.meta.rook_moved(false, false, false, true),
                    0b1000 => self.meta.rook_moved(true, true, true, false),
                    0b1001 => self.meta.rook_moved(false, true, true, false),
                    0b1010 => self.meta.rook_moved(true, false, true, false),
                    0b1011 => self.meta.rook_moved(false, false, true, false),
                    0b1100 => self.meta.rook_moved(true, true, false, false),
                    0b1101 => self.meta.rook_moved(false, true, false, false),
                    0b1110 => self.meta.rook_moved(true, false, false, false),
                    0b1111 => self.meta.rook_moved(false, false, false, false),
                    _ => unreachable!(),
                }
            }
            Piece::King => self.meta.king_moved(self.current_player()),
            Piece::Pawn => {
                self.meta.half_move_reset();
            }
            _ => (),
        }

        if had_capture {
            self.meta.half_move_reset();
        }

        // TODO: Track castling rights, if the piece is a rook, we need to know which

        // Full Move tick
        self.meta.full_move_tick();

        Ok(())
    }

    fn castle_rank_mask(&self) -> usize {
        if self.current_player().is_black() {
            0o70
        } else {
            0o00
        }
    }

    fn friendly_piece_mut(&mut self, piece: Piece) -> &mut Bitboard {
        self.get_mut_piece(self.current_player(), piece)
    }

    fn enemy_piece_mut(&mut self, piece: Piece) -> &mut Bitboard {
        self.get_mut_piece(self.other_player(), piece)
    }

    /// Executes the Short-Castle move for the current player.
    fn long_castle(&mut self) -> MoveResult<()> {
        let rank_mask = self.castle_rank_mask();
        self.friendly_piece_mut(Piece::King)
            .move_piece(0o04 | rank_mask, 0o02 | rank_mask);
        // NOTE: since the source for the rook is 0, we can omit the 0o00.
        self.friendly_piece_mut(Piece::Rook)
            .move_piece(rank_mask, 0o03 | rank_mask);
        Ok(())
    }

    /// Executes the Short-Castle move for the current player.
    fn short_castle(&mut self) -> MoveResult<()> {
        let rank_mask = self.castle_rank_mask();
        self.friendly_piece_mut(Piece::King)
            .move_piece(0o04 | rank_mask, 0o06 | rank_mask);
        self.friendly_piece_mut(Piece::Rook)
            .move_piece(0o07 | rank_mask, 0o05 | rank_mask);
        Ok(())
    }

    fn unshort_castle(&mut self) -> MoveResult<()> {
        let rank_mask = self.castle_rank_mask();
        self.friendly_piece_mut(Piece::King)
            .move_piece(0o06 | rank_mask, 0o04 | rank_mask);
        self.friendly_piece_mut(Piece::Rook)
            .move_piece(0o05 | rank_mask, 0o07 | rank_mask);
        Ok(())
    }

    fn unlong_castle(&mut self) -> MoveResult<()> {
        let rank_mask = self.castle_rank_mask();
        self.friendly_piece_mut(Piece::King)
            .move_piece(0o02 | rank_mask, 0o04 | rank_mask);
        self.friendly_piece_mut(Piece::Rook)
            .move_piece(0o03 | rank_mask, rank_mask);
        Ok(())
    }

    fn move_piece(&mut self, piece: Piece, mov: Move) -> MoveResult<()> {
        self.friendly_piece_mut(piece)
            .move_piece(mov.source_idx(), mov.target_idx());
        Ok(())
    }

    /// Unmoves a piece, does not restore captures or anything
    fn unmove_enemy_piece(&mut self, piece: Piece, mov: Move) -> MoveResult<()> {
        self.enemy_piece_mut(piece)
            .move_piece(mov.target_idx(), mov.source_idx());
        Ok(())
    }

    /// Unmoves a piece, does not restore captures or anything
    fn unmove_piece(&mut self, piece: Piece, mov: Move) -> MoveResult<()> {
        self.friendly_piece_mut(piece)
            .move_piece(mov.target_idx(), mov.source_idx());
        Ok(())
    }

    fn remove_enemy_piece(&mut self, piece: Piece, idx: usize) -> MoveResult<()> {
        self.enemy_piece_mut(piece).unset_by_index(idx);
        Ok(())
    }

    fn place_enemy_piece(&mut self, piece: Piece, idx: usize) -> MoveResult<()> {
        self.enemy_piece_mut(piece).set_by_index(idx);
        Ok(())
    }

    fn remove_friendly_piece(&mut self, piece: Piece, idx: usize) -> MoveResult<()> {
        self.friendly_piece_mut(piece).unset_by_index(idx);
        Ok(())
    }

    fn place_friendly_piece(&mut self, piece: Piece, idx: usize) -> MoveResult<()> {
        self.friendly_piece_mut(piece).set_by_index(idx);
        Ok(())
    }

    fn execute_promotion(&mut self, mov: Move, promotion_piece: Piece) -> MoveResult<()> {
        self.remove_friendly_piece(Piece::Pawn, mov.source_idx())?;
        self.place_friendly_piece(promotion_piece, mov.target_idx())
    }

    fn execute_promotion_capture(
        &mut self,
        mov: Move,
        promotion_piece: Piece,
    ) -> MoveResult<Piece> {
        let target_piece = match self.enemy_piece_at_index(mov.target_idx()) {
            Some(t) => t,
            None => return Err(MoveError::MissingTargetPiece(MoveMode::Make, *self, mov)),
        };

        self.remove_enemy_piece(target_piece, mov.target_idx())?;
        self.remove_friendly_piece(Piece::Pawn, mov.source_idx())?;
        self.place_friendly_piece(promotion_piece, mov.target_idx())?;

        Ok(target_piece)
    }

    fn unexecute_promotion(&mut self, mov: Move, promotion_piece: Piece) -> MoveResult<()> {
        self.remove_friendly_piece(promotion_piece, mov.target_idx())?;
        self.place_friendly_piece(Piece::Pawn, mov.source_idx())
    }

    fn unexecute_promotion_capture(
        &mut self,
        mov: Move,
        promotion_piece: Piece,
        target_piece: Option<Piece>,
    ) -> MoveResult<()> {
        let t = match target_piece {
            Some(t) => t,
            None => return Err(MoveError::MissingTargetPiece(MoveMode::Make, *self, mov)),
        };

        self.remove_friendly_piece(promotion_piece, mov.target_idx())?;
        self.place_friendly_piece(Piece::Pawn, mov.source_idx())?;
        self.place_enemy_piece(t, mov.target_idx())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tracing::info;
    use tracing_test::traced_test;

    #[test]
    fn short_castle_is_done_correctly() -> MoveResult<()> {
        let mut p = Ply::from_fen(START_POSITION_FEN);
        // This encodes the following game:
        // 1. d4 d5
        // 2. e3 e6
        // 3. Be2 Be7
        // 4. Nf3 Nf6
        // 5. O-O O-O
        p.make_by_notation("d2", "d4", MoveType::QUIET)?;
        p.make_by_notation("d7", "d5", MoveType::QUIET)?;
        p.make_by_notation("e2", "e3", MoveType::QUIET)?;
        p.make_by_notation("e7", "e6", MoveType::QUIET)?;
        p.make_by_notation("f1", "e2", MoveType::QUIET)?;
        p.make_by_notation("f8", "e7", MoveType::QUIET)?;
        p.make_by_notation("g1", "f3", MoveType::QUIET)?;
        p.make_by_notation("g8", "f6", MoveType::QUIET)?;

        // Before castle, white king is on e1 and kingside rook on h1
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("g1")), None);
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("f1")), None);
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("h1")),
            Some((Color::WHITE, Piece::Rook))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("e1")),
            Some((Color::WHITE, Piece::King))
        );

        p.make(Move::short_castle(Color::WHITE))?;

        // After, king is on g1 and rook on f1
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("g1")),
            Some((Color::WHITE, Piece::King))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("f1")),
            Some((Color::WHITE, Piece::Rook))
        );
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("h1")), None);
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("e1")), None);

        // Black King on e8, Black KS Rook on h8
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("g8")), None);
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("f8")), None);
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("h8")),
            Some((Color::BLACK, Piece::Rook))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("e8")),
            Some((Color::BLACK, Piece::King))
        );

        p.make(Move::short_castle(Color::BLACK))?;

        // After, black king on g8 and rook on f8
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("g8")),
            Some((Color::BLACK, Piece::King))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("f8")),
            Some((Color::BLACK, Piece::Rook))
        );
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("h8")), None);
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("e8")), None);

        Ok(())
    }

    #[traced_test]
    #[test]
    fn en_passant_capture_is_undone_correctly() -> MoveResult<()> {
        let mut p = Ply::from_fen(START_POSITION_FEN);
        // This encodes the following game:
        // 1. d4 h6
        // 2. d5 e5
        // 3. dxe6
        p.make_by_notation("d2", "d4", MoveType::QUIET)?;
        p.make_by_notation("h7", "h6", MoveType::QUIET)?;
        p.make_by_notation("d4", "d5", MoveType::QUIET)?;
        p.make_by_notation("e7", "e5", MoveType::QUIET)?;
        p.make_by_notation("d5", "e6", MoveType::EP_CAPTURE)?;

        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("e5")), None);
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("e6")),
            Some((Color::WHITE, Piece::Pawn))
        );

        // Now I want to undo the en passant capture
        p.unmake_by_notation("d5", "e6", MoveType::EP_CAPTURE, Some(Piece::Pawn), p.meta)?;

        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("e5")),
            Some((Color::BLACK, Piece::Pawn))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("e6")),
            None
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("d5")), 
            Some((Color::WHITE, Piece::Pawn))
        );

        Ok(())
    }

    #[test]
    fn long_castle_is_done_correctly() -> MoveResult<()> {
        let mut p = Ply::from_fen(START_POSITION_FEN);
        // This encodes the following game:
        // 1. d4 d5
        // 2. Bf4 Bf5
        // 3. Nc3 Nc6
        // 4. Qd2 Qd7
        // 5. O-O-O O-O-O
        p.make_by_notation("d2", "d4", MoveType::QUIET)?;
        p.make_by_notation("d7", "d5", MoveType::QUIET)?;
        p.make_by_notation("c1", "f4", MoveType::QUIET)?;
        p.make_by_notation("c8", "f5", MoveType::QUIET)?;
        p.make_by_notation("b1", "c3", MoveType::QUIET)?;
        p.make_by_notation("b8", "c6", MoveType::QUIET)?;
        p.make_by_notation("d1", "d2", MoveType::QUIET)?;
        p.make_by_notation("d8", "d7", MoveType::QUIET)?;

        // Before castle, white king is on e1 and queenside rook on a1
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("c1")), None);
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("d1")), None);
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("a1")),
            Some((Color::WHITE, Piece::Rook))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("e1")),
            Some((Color::WHITE, Piece::King))
        );

        p.make(Move::long_castle(Color::WHITE))?;

        // After, king is on g1 and rook on f1
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("c1")),
            Some((Color::WHITE, Piece::King))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("d1")),
            Some((Color::WHITE, Piece::Rook))
        );
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("a1")), None);
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("e1")), None);

        // Black King on e8, Black KS Rook on h8
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("c8")), None);
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("d8")), None);
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("a8")),
            Some((Color::BLACK, Piece::Rook))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("e8")),
            Some((Color::BLACK, Piece::King))
        );

        p.make(Move::long_castle(Color::BLACK))?;

        // After, black king on g8 and rook on f8
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("c8")),
            Some((Color::BLACK, Piece::King))
        );
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("d8")),
            Some((Color::BLACK, Piece::Rook))
        );
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("a8")), None);
        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("e8")), None);

        Ok(())
    }

    #[test]
    fn en_passant_capture() -> MoveResult<()> {
        let mut p = Ply::from_fen(START_POSITION_FEN);
        // This encodes the following game:
        // 1. d4 Nh6
        // 2. d5 e5
        // 3. dxe6
        p.make_by_notation("d2", "d4", MoveType::QUIET)?;
        p.make_by_notation("g8", "h6", MoveType::QUIET)?;
        p.make_by_notation("d4", "d5", MoveType::QUIET)?;
        p.make_by_notation("e7", "e5", MoveType::QUIET)?;
        p.make_by_notation("d5", "e6", MoveType::EP_CAPTURE)?;

        assert_eq!(p.piece_at_index(NOTATION_TO_INDEX("e5")), None);
        assert_eq!(
            p.piece_at_index(NOTATION_TO_INDEX("e6")),
            Some((Color::WHITE, Piece::Pawn))
        );

        Ok(())
    }
}
