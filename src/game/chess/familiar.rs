// A familiar is a cursor on a ChessAction Log which maintains a gamestate and can be rolled
// forward/backward to different positions within the game. it will be responsible for talking to
// caches/doing other logic to make that process efficient, there will be many kinds of familiars,
// the most basic is the Representation Familiar, which takes some `Alter + Query` structure and
// does the default, pure-alter based approach to maintianing the gamestate. This is designed for
// maximum compatibility.
//
//
// A familiar is created from a Cursor or WriteHead on the log, it holds that reference for it's
// life and is tied to the life of the log it associates with.
//
// Examples of Familiars:
//
// - Target Position - Flies to the given `position` in the log and calculates the gamestate at
// that position, skipping any variation it needs to to get there, entering whatever variations it
// needs to as well.
//
// 
// The familiar should be relatively ignorant of it's actual backing representation, preferring to
// pass off that to the `Play` trait,
// 
//

use crate::{interface::play::Play, types::log::cursor::Cursor};

#[derive(Debug, Clone)]
pub struct Familiar<'a, T> where T : Play + Default {
    cursor: Cursor<'a, T::Rule>,
    rep: T
}

impl<'a, T> Familiar<'a, T> where T : Play + Default {
    pub fn new(cursor: Cursor<'a, T::Rule>) -> Self {
        Self { cursor, rep: T::default() }
    }

    pub fn advance(&mut self) {
        if let Some(action) = self.cursor.next() {
            self.rep.apply_mut(action);
        }
    }

    /*
    pub fn rewind(&mut self) {
        if let Some(action) = self.cursor.prev() {
            self.rep.unwind_mut(action);
        }
    }
    */

    pub fn rep(&self) -> &T {
        &self.rep
    }

    pub fn metadata(&self) -> T::Metadata {
        self.rep.metadata().clone()
    }
}

#[cfg(test)]
mod tests {

    use crate::{board::PieceBoard, constants::START_POSITION_FEN, coup::rep::{Move, MoveType}, game::{chess::PositionMetadata, variation::Variation, ChessGame}, notation::{fen::FEN, *}};

    use super::*;

    #[test]
    fn familiar_works_with_pieceboard_to_capture_gamestate() {
        let mut log = Variation::new();
        log.new_game()
           .setup(FEN::new(START_POSITION_FEN))
           .make(Move::new(D2, D4, MoveType::DOUBLE_PAWN))
           .make(Move::new(D7, D5, MoveType::DOUBLE_PAWN))
           .commit();

        let cursor = log.get_cursor();
        let mut familiar : Familiar<ChessGame<PieceBoard>> = Familiar::new(cursor);

        familiar.advance();
        familiar.advance();
        familiar.advance();
        familiar.advance();

        assert_eq!(familiar.rep().rep, FEN::new("rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 2").into());
        assert_eq!(familiar.metadata().fullmove_number, 2);
    }

}

