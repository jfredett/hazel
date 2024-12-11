use crate::{interface::play::Play, play::Unplay, types::log::cursor::Cursor, Alter, Query};
use super::delim::Delim;
use super::{action::Action, ChessGame};
use crate::notation::ben::BEN;
use crate::coup::rep::Move;

#[derive(Debug, Clone)]
pub struct Familiar<'a, T> where T : Play + Default {
    // TODO: Temporarily fixing the types
    cursor: Cursor<'a, Action<Move, BEN>>,
    stack: Vec<T>,
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
    pub fn new(cursor: Cursor<'a, Action<Move, BEN>>) -> Self {
        Self { cursor, stack: vec![], rep: T::default() }
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

    pub fn advance_until(&mut self, predicate: impl Fn(&Self) -> bool) {
        while let Some(coup) = self.cursor.next() {
            match coup {
                Action::Setup(ben) => {
                    self.rep = T::default();
                    self.rep.apply_mut(&Action::Setup(*ben));
                },
                Action::Make(mov) => {
                    self.rep.apply_mut(&Action::Make(*mov));
                },
                Action::Variation(Delim::Start) => {
                    self.stack.push(self.rep.clone());
                },
                Action::Variation(Delim::End) => {
                    self.rep = self.stack.pop().unwrap();
                },
                _ => { todo!(); }
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
        let target = self.cursor.position() + count;
        self.advance_until(|f| f.cursor.position() == target);
    }

    pub fn restart(&mut self) {
        // note this seeks the _cursor_, not the familiar
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

    fn apply(&self, action: &Action<Move, BEN>) -> Self {
        let mut new_game = self.clone();
        new_game.apply_mut(action);
        new_game
    }

    fn apply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self {
        self.rep.apply_mut(action);
        self
    }

    fn metadata(&self) -> Self::Metadata {
        self.rep.metadata()
    }
}

impl<'a, T> Unplay for Familiar<'a, ChessGame<T>> where T : Query + Alter + Clone + Default {
    fn unapply(&self, action: &Action<Move, BEN>) -> Self {
        let mut new_game = self.clone();
        new_game.unapply_mut(action);
        new_game
    } 

    fn unapply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self {
        let target_position = self.cursor.position() - 1;
        self.seek(target_position);
        self
    }

}

#[cfg(test)]
mod tests {

    use crate::{board::PieceBoard, constants::START_POSITION_FEN, coup::rep::{Move, MoveType}, game::{chess::PositionMetadata, variation::Variation, ChessGame}, notation::{fen::FEN, *}};

    use super::*;

    fn example_game() -> Variation {
        let mut log = Variation::default();
        log.new_game()
           .make(Move::new(D2, D4, MoveType::DOUBLE_PAWN))
           .make(Move::new(D7, D5, MoveType::DOUBLE_PAWN))
           .make(Move::new(C1, F4, MoveType::QUIET)) 
           .make(Move::new(G8, F6, MoveType::QUIET)) 
           .start_variation()
                .make(Move::new(B1, C3, MoveType::QUIET))
           .end_variation()
           .make(Move::new(E2, E3, MoveType::QUIET))
           .make(Move::new(E7, E6, MoveType::QUIET))
           .commit();
        log
    }

    #[test]
    fn familiar_works_with_pieceboard_to_capture_gamestate() {
        let log = example_game();
        let cursor = log.get_cursor();
        let mut familiar : Familiar<ChessGame<PieceBoard>> = Familiar::new(cursor);

        // Setup is the 'zeroth' action, and we proceed actionwise. At the moment that also
        // corresponds to ply number, but the example game has variations so that is not reliable.
        familiar.advance_by(2);

        assert_eq!(familiar.rep().rep, FEN::new("rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 2").into());
        assert_eq!(familiar.metadata().fullmove_number, 2);
    }

    #[test]
    fn familiar_finds_the_variation_position() {
        let log = example_game();
        let cursor = log.get_cursor();
        let mut familiar : Familiar<ChessGame<PieceBoard>> = Familiar::new(cursor);

        // We advance _over_ the variation opening, but stop inside.
        familiar.advance_by(6);

        let f = FEN::from(familiar.rep().rep);
        println!("{}", f);

        assert_eq!(familiar.rep().rep, FEN::new("rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 1").into());
        assert_eq!(familiar.metadata().fullmove_number, 4);

    }
}

