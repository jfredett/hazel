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

use crate::{interface::play::Play, play::Unplay, types::log::cursor::Cursor, Alter, Query};
use super::{action::Action, ChessGame};

#[derive(Debug, Clone)]
pub struct Familiar<'a, T> where T : Play + Default {
    cursor: Cursor<'a, T::Action>,
    rep: T
}


/*
*
* a game like:
*
* 1. d4 d5
* 2. Bf4 Nf6 (2. ... Nc6)
* 3. e3 e6
*
* Would lay out like:
*
* 0 Setup(STARTPOS)
* 1 Make(D2, D4)
* 2 Make(D7, D5)
* 3 Make(C1, F4)
* 4 Make(G8, F6)
* 5 Variation(Delim::Start)
* 6   Make(B1, C3)
* 7 Variation(Delim::End)
* 8 Make(E2, E3)
* 9 Make(E7, E6)
*
*
* So seeking to position 6 would mean jumping into the variation. But jumping to pos 8 should not
* include the variation step.
*
* A good test would be to calculate the FEN at each position and then check. Could have stockfish
* do the same and compare for a big game with lots of variations. Ultimately I should be able to
* produce a `UCI` `position` command from any position in the log.
*
* I'll also need 'seek to start' and 'seek to end'. This requires the unwind functionality be
* there, and that gets tricky. A recalc approach to start I think makes the most sense.
*
* Right now I have the familiar only know about a Cursor, not the whole Log, I could reduce this to
* just have a reference to the log so it can 'restart' easily. I could also manually 'restart' just
* by resetting the position to 0.
*
* Perhaps I need familiars for generating indicies first? IDK.
*
*
*/

impl<'a, T> Familiar<'a, T> where T : Play + Default {
    pub fn new(cursor: Cursor<'a, Action<T::Coup>>) -> Self {
        Self { cursor, rep: T::default() }
    }

    pub fn rep(&self) -> &T {
        &self.rep
    }

    pub fn metadata(&self) -> T::Metadata {
        self.rep.metadata().clone()
    }

    pub fn advance(&mut self) {
        // NOTE: I could do `self.advance_until(|_| true)` here, but I think this is probably
        // faster? Whenever I get around to caring about how fast this is I'll take a look.
        if let Some(action) = self.cursor.next() {
            self.rep.apply_mut(action);
        }
    }

    // I need to fix T::Rule to a ChessAction. Then this can contain the variation-skipping logic,
    // in general, I need to save state right before a variation opens, and then enter the
    // variation. When I close the variation I pop the state and continue. When I break, I return
    // the current state at the top of the stack.
    //
    // I could add a trait bound on rule, which would specify something like
    // `should_start_variation` and `should_end_variation` which would maybe have a generic
    // delimiter type for metadata associated with the ending variation. That way an 'end
    // variation' on the mainline would indicate a final halt to the game.
    //
    // I don't know the performance effect of that extra method call, but I can't imagine it's
    // great, I think maybe building a bespoke enum that is something like:
    //
    // enum Action<T> {
    //    Make(T),
    //    Variation(Delim),
    //    Halt(Reason)
    // }
    //
    // Then we're parametric over the Move Representation, instead of the Rule. Unmake is handled
    // by recalculation, so we don't need an additional enum variant for that.
    //
    // Then the traitbound becomes some kind of `Move` represnetation, so I suppose T is bound with
    // at least Alter, probably Query, and _maybe_ Copy?
    //
    // Whatever it works out to should be wrapped in the `Coup` trait alias. 
    //
    //
    // Ah shit this is where I need a Compiles-To, I suppose I can do this with a type function for
    // now?
    //
    //
    pub fn advance_until(&mut self, predicate: impl Fn(&Self) -> bool) {
        while let Some(coup) = self.cursor.next() {
            match coup {
                Action::Make(mov) => {
                    self.rep.make(mov);
                }

            }

            if predicate(self) {
                break;
            }
        }
    }

    pub fn advance_to_end(&mut self) {
        self.advance_until(|_| false);
    }

    pub fn advance_by(&mut self, count: usize) {
        for _ in 0..count {
            self.advance();
        }
    }

    pub fn restart(&mut self) {
        self.cursor.seek(0);
        self.rep = T::default();
    }

    pub fn seek(&mut self, position: usize) {
        self.restart();
        self.advance_by(position);
    }


}

impl<T> Play for Familiar<'_, T> where T : Play + Default {
    type Metadata = T::Metadata;

    fn apply(&self, action: &Action<Self::Coup>) -> Self {
        let mut new_game = self.clone();
        new_game.apply_mut(action);
        new_game
    }

    fn apply_mut(&mut self, action: &Action<Self::Coup>) -> &mut Self {
        self.rep.apply_mut(action);
        self
    }

    fn metadata(&self) -> Self::Metadata {
        self.rep.metadata()
    }
}

impl<'a, T> Unplay for Familiar<'a, ChessGame<T>> where T : Query + Alter + Clone + Default {
    fn unapply(&self, action: &Self::Action) -> Self {
        let mut new_game = self.clone();
        new_game.unapply_mut(action);
        new_game
    } 

    fn unapply_mut(&mut self, action: &Self::Action) -> &mut Self {
        let target_position = self.cursor.position() - 1;
        self.seek(target_position);
        self
    }

}

#[cfg(test)]
mod tests {

    use crate::{board::PieceBoard, constants::START_POSITION_FEN, coup::rep::{Move, MoveType}, game::{chess::PositionMetadata, variation::Variation, ChessGame}, notation::{fen::FEN, *}};

    use super::*;

    #[test]
    fn familiar_works_with_pieceboard_to_capture_gamestate() {
        let mut log = Variation::default();
        log.new_game()
           .setup(FEN::new(START_POSITION_FEN))
           .make(Move::new(D2, D4, MoveType::DOUBLE_PAWN))
           .make(Move::new(D7, D5, MoveType::DOUBLE_PAWN))
           .commit();

        let cursor = log.get_cursor();
        let mut familiar : Familiar<ChessGame<PieceBoard>> = Familiar::new(cursor);

        familiar.advance_by(4);

        assert_eq!(familiar.rep().rep, FEN::new("rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 2").into());
        assert_eq!(familiar.metadata().fullmove_number, 2);
    }

}

