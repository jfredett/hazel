
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
        let source_piece = self.friendly_piece_at_index(mov.source_idx());
        let target_piece = self.enemy_piece_at_index(mov.target_idx());
        
        // probably want to remove the .and stuff and just resolve the error, return a None if needed.
        let result: Option<Piece> = match mov.move_metadata() {
            MoveType::QUIET                     => { 
                match source_piece {
                    Some(s) => self.move_piece(s, mov)?,
                    None => return Err(MoveError::MissingSourcePiece(MoveMode::Make, *self, mov)), 
                };
                None
            }
            MoveType::SHORT_CASTLE              => { self.short_castle()?; None },
            MoveType::LONG_CASTLE               => { self.long_castle()?; None },
            MoveType::CAPTURE => {
                match target_piece {
                    Some(t) => self.remove_enemy_piece(t, mov.target_idx())?,
                    None => return Err(MoveError::MissingTargetPiece(MoveMode::Make, *self, mov)), 
                }
                match source_piece {
                    Some(s) => self.move_piece(s, mov)?,
                    None => return Err(MoveError::MissingSourcePiece(MoveMode::Make, *self, mov)), 
                };
                None
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
            MoveType::DOUBLE_PAWN               => None, // NOTE: This is unused right now, may be changed later
            _ => return Err(MoveError::UnrecognizedMove(MoveMode::Make, mov))
        };
        
        /*
         * TODO: Half-move clock
         * This is a rough version, the #and_then call doesn't work. I really want a way to say, "If the thing is_some, then do this thing, otherwise do nothing"
        self.half_move_clock += 1;
        source_piece.and_then(|p| { if p == Piece::Pawn { self.half_move_clock = 0; }; None });
        target_piece.and_then(|_| { self.half_move_clock = 0; None });
        */

        // just completed black's turn, another full move down
        if self.meta.contains(Metadata::BLACK_TO_MOVE) { self.full_move_clock += 1; }

        // flip the player-turn bit
        self.meta ^= Metadata::BLACK_TO_MOVE;
        
        Ok(result)
    }

    pub fn unmake(&mut self, mov: Move, target_piece: Option<Piece>) -> MoveResult<()> {
        let source_piece = self.friendly_piece_at_index(mov.source_idx());
        
        // probably want to remove the .and stuff and just resolve the error, return a None if needed.
        match mov.move_metadata() {
            MoveType::QUIET                     => {
                match source_piece {
                    Some(s) => self.unmove_piece(s, mov)?,
                    None => { return Err(MoveError::MissingSourcePiece(MoveMode::Unmake, *self, mov)) }
                }
            },
            MoveType::SHORT_CASTLE              => self.unshort_castle()?,
            MoveType::LONG_CASTLE               => self.unlong_castle()?,
            MoveType::CAPTURE => {
                match target_piece {
                    Some(t) => self.place_enemy_piece(t, mov.target_idx())?,
                    None => { return Err(MoveError::MissingTargetPiece(MoveMode::Unmake, *self, mov)) },
                }
                match source_piece {
                    Some(s) => self.unmove_piece(s, mov)?,
                    None => { return Err(MoveError::MissingSourcePiece(MoveMode::Unmake, *self, mov)) }
                }
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
            MoveType::DOUBLE_PAWN               => (), // NOTE: This is unused right now, may be changed later
            _ => return Err(MoveError::UnrecognizedMove(MoveMode::Unmake, mov))
        };
        
        /*
         * TODO: Half-move clock
         * This is a rough version, the #and_then call doesn't work. I really want a way to say, "If the thing is_some, then do this thing, otherwise do nothing"
        self.half_move_clock -= 1;
        source_piece.and_then(|p| { if p == Piece::Pawn { self.half_move_clock = 0; }; None });
        target_piece.and_then(|_| { self.half_move_clock = 0; None });
        */

        // just completed black's turn, another full move down
        if self.meta.contains(Metadata::BLACK_TO_MOVE) { self.full_move_clock += 1; }

        // flip the player-turn bit
        self.meta ^= Metadata::BLACK_TO_MOVE;
        
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