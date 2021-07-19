
use std::fmt::Display;

use super::*;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum MoveMode {
    Make,
    Unmake
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
    // #[error("Error when attempting to make move {1:?} on board {0:?}")]
    // MakeError(Ply, Move),
    // #[error("Error when attempting to unmake move {1:?} on board {0:?}")]
    // UnmakeError(Ply, Move),
    #[error("{0}: Move {1:?} has malformed metadata")]
    UnrecognizedMove(MoveMode, Move),
    #[error("{0}: Missing friendly piece at source index of {2:?} for board {1:?}")]
    MissingSourcePiece(MoveMode, Ply, Move),
    #[error("{0}: Missing enemy piece at source index of {2:?} for board {1:?}")]
    MissingTargetPiece(MoveMode, Ply, Move)
}

impl Ply {
    pub fn make(&mut self, mov: Move) -> MoveResult<Option<Piece>> {
        let source_piece = match self.friendly_piece_at_index(mov.source_idx()) {
            Some(s) => s,
            None => return Err(MoveError::MissingSourcePiece(MoveMode::Make, *self, mov)), 
        };
        let target_piece = self.enemy_piece_at_index(mov.target_idx());
        
        // probably want to remove the .and stuff and just resolve the error, return a None if needed.
        let result: Option<Piece> = match mov.move_metadata() {
            MoveType::QUIET                     => { self.move_piece(source_piece, mov)?; None },
            MoveType::SHORT_CASTLE              => { self.short_castle()?; None },
            MoveType::LONG_CASTLE               => { self.long_castle()?; None },
            MoveType::CAPTURE => {
                match target_piece {
                    Some(t) => self.remove_enemy_piece(t, mov.target_idx())?,
                    None => return Err(MoveError::MissingTargetPiece(MoveMode::Make, *self, mov)), 
                }
                self.move_piece(source_piece, mov)?;
                target_piece
            },
            MoveType::EP_CAPTURE => todo!(),
            MoveType::PROMOTION_KNIGHT          => { self.execute_promotion(mov, Piece::Knight)?; None },
            MoveType::PROMOTION_BISHOP          => { self.execute_promotion(mov, Piece::Bishop)?; None },
            MoveType::PROMOTION_ROOK            => { self.execute_promotion(mov, Piece::Rook)?; None  },
            MoveType::PROMOTION_QUEEN           => { self.execute_promotion(mov, Piece::Queen)?; None  },
            MoveType::PROMOTION_CAPTURE_KNIGHT  => { Some(self.execute_promotion_capture(mov, Piece::Knight)?) },
            MoveType::PROMOTION_CAPTURE_BISHOP  => { Some(self.execute_promotion_capture(mov, Piece::Bishop)?) },
            MoveType::PROMOTION_CAPTURE_ROOK    => { Some(self.execute_promotion_capture(mov, Piece::Rook)?) },
            MoveType::PROMOTION_CAPTURE_QUEEN   => { Some(self.execute_promotion_capture(mov, Piece::Queen)?) },
            MoveType::DOUBLE_PAWN               => { self.move_piece(source_piece, mov)?; None },
            _ => return Err(MoveError::UnrecognizedMove(MoveMode::Make, mov))
        };
        
        self.tick(source_piece, result.is_some())?;
        
        Ok(result)
    }

    #[instrument(skip(self))]
    pub fn unmake(&mut self, mov: Move, target_piece: Option<Piece>) -> MoveResult<()> {
        // Untick _first_ so that the 'current_player' becomes the correct color.
        // TODO: half-move clock memory
        self.untick(None)?;

        // note how it's target -- this is _after_ the mov has been made, so we have to work backwards
        let source_piece = match self.friendly_piece_at_index(mov.target_idx()) {
            Some(s) => s,
            None => { return Err(MoveError::MissingSourcePiece(MoveMode::Unmake, *self, mov)) }
        };

        match mov.move_metadata() {
            MoveType::QUIET                     => self.unmove_piece(source_piece, mov)?,
            MoveType::SHORT_CASTLE              => self.unshort_castle()?,
            MoveType::LONG_CASTLE               => self.unlong_castle()?,
            MoveType::CAPTURE => {
                match target_piece {
                    Some(t) => self.place_enemy_piece(t, mov.target_idx())?,
                    None => { return Err(MoveError::MissingTargetPiece(MoveMode::Unmake, *self, mov)) },
                }
                self.unmove_piece(source_piece, mov)?
            },
            MoveType::EP_CAPTURE => todo!(),
            MoveType::PROMOTION_KNIGHT          => self.unexecute_promotion(mov, Piece::Knight)?,
            MoveType::PROMOTION_BISHOP          => self.unexecute_promotion(mov, Piece::Bishop)?,
            MoveType::PROMOTION_ROOK            => self.unexecute_promotion(mov, Piece::Rook)?,
            MoveType::PROMOTION_QUEEN           => self.unexecute_promotion(mov, Piece::Queen)?,
            MoveType::PROMOTION_CAPTURE_KNIGHT  => self.unexecute_promotion_capture(mov, Piece::Knight, target_piece)?,
            MoveType::PROMOTION_CAPTURE_BISHOP  => self.unexecute_promotion_capture(mov, Piece::Bishop, target_piece)?,
            MoveType::PROMOTION_CAPTURE_ROOK    => self.unexecute_promotion_capture(mov, Piece::Rook, target_piece)?,
            MoveType::PROMOTION_CAPTURE_QUEEN   => self.unexecute_promotion_capture(mov, Piece::Queen, target_piece)?,
            MoveType::DOUBLE_PAWN               => self.unmove_piece(source_piece, mov)?,
            _ => return Err(MoveError::UnrecognizedMove(MoveMode::Unmake, mov))
        };
        
        Ok(())
    }

    pub fn make_by_notation(&mut self, source: &str, target: &str, metadata: MoveType) -> MoveResult<Option<Piece>> {
        self.make(Move::from(
            NOTATION_TO_INDEX(source) as u16,
            NOTATION_TO_INDEX(target) as u16,
            metadata
        ))
    }
    
    pub fn unmake_by_notation(&mut self, source: &str, target: &str, metadata: MoveType, captured_piece: Option<Piece>) -> MoveResult<()> {
        self.unmake(Move::from(
            NOTATION_TO_INDEX(source) as u16,
            NOTATION_TO_INDEX(target) as u16,
            metadata
        ), captured_piece)
    }

    
    fn tick(&mut self, piece_moved: Piece, had_capture: bool) -> MoveResult<()> {
        // Half-move clock resets on capture or pawn move.
        self.half_move_clock += 1;
        if piece_moved == Piece::Pawn || had_capture { self.half_move_clock = 0; }
        
        // Full Move Clock
        if self.current_player() == Color::BLACK { self.full_move_clock += 1; }
        // Current Player Switch
        self.meta ^= Metadata::BLACK_TO_MOVE;
    
        Ok(())
    }
    
    fn untick(&mut self, previous_half_move_count: Option<usize>) -> MoveResult<()> {
        // Half move clock unwinds
        if let Some(count) = previous_half_move_count { self.half_move_clock = count as u8; }
        
        // Full Move Clock
        if self.current_player() == Color::WHITE { self.full_move_clock -= 1; }

        // Current Player Switch
        self.meta ^= Metadata::BLACK_TO_MOVE;
        
        Ok(())
    }

    fn castle_rank_mask(&self) -> usize {
        if self.current_player().is_black() {
            0o07
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
        self.friendly_piece_mut(Piece::King).move_piece(0o04 | rank_mask, 0o02 | rank_mask);
        // NOTE: since the source for the rook is 0, we can omit the 0o00.
        self.friendly_piece_mut(Piece::Rook).move_piece(rank_mask, 0o03 | rank_mask);
        Ok(())
    }

    /// Executes the Short-Castle move for the current player.
    fn short_castle(&mut self) -> MoveResult<()> {
        let rank_mask = self.castle_rank_mask();
        self.friendly_piece_mut(Piece::King).move_piece(0o04 | rank_mask, 0o06 | rank_mask);
        self.friendly_piece_mut(Piece::Rook).move_piece(0o07 | rank_mask, 0o05 | rank_mask);
        Ok(())
    }
    
    fn unshort_castle(&mut self) -> MoveResult<()> {
        let rank_mask = self.castle_rank_mask();
        self.friendly_piece_mut(Piece::King).move_piece(0o06 | rank_mask, 0o04 | rank_mask);
        self.friendly_piece_mut(Piece::Rook).move_piece(0o05 | rank_mask, 0o07 | rank_mask);
        Ok(())
    }
    
    fn unlong_castle(&mut self) -> MoveResult<()> {
        let rank_mask = self.castle_rank_mask();
        self.friendly_piece_mut(Piece::King).move_piece(0o02 | rank_mask, 0o04 | rank_mask);
        self.friendly_piece_mut(Piece::Rook).move_piece(0o03 | rank_mask,        rank_mask);
        Ok(())
    }
    
    fn move_piece(&mut self, piece: Piece, mov: Move) -> MoveResult<()> {
        self.friendly_piece_mut(piece).move_piece(mov.source_idx(), mov.target_idx());
        Ok(())
    }
    
    /// Unmoves a piece, does not restore captures or anything
    fn unmove_piece(&mut self, piece: Piece, mov: Move) -> MoveResult<()> {
        self.friendly_piece_mut(piece).move_piece(mov.target_idx(), mov.source_idx());
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
    
    fn execute_promotion_capture(&mut self, mov: Move, promotion_piece: Piece) -> MoveResult<Piece> {
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
    
    fn unexecute_promotion_capture(&mut self, mov: Move, promotion_piece: Piece, target_piece: Option<Piece>) -> MoveResult<()> {
        let t = match target_piece {
            Some(t) => t,
            None => return Err(MoveError::MissingTargetPiece(MoveMode::Make, *self, mov)), 
        };

        self.remove_friendly_piece(promotion_piece, mov.target_idx())?;
        self.place_friendly_piece(Piece::Pawn, mov.source_idx())?;
        self.place_enemy_piece(t, mov.target_idx())
    }

}