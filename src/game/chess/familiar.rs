use crate::{coup::rep::Move, interface::play::Play, notation::ben::BEN, types::log::cursor::Cursor};
use super::{action::Action, delim::Delim};

#[derive(Debug, Clone)]
pub struct Familiar<'a, T> where T : Play + Default {
    // TODO: Temporarily fixing the types
    cursor: Cursor<'a, Action<Move, BEN>>,
    stack: Vec<T>,
    prev_rep: T,
    rep: T
}

impl<'a, T> Familiar<'a, T> where T : Play + Default {
    pub fn new(cursor: Cursor<'a, Action<Move, BEN>>) -> Self {
        Self { cursor, stack: vec![], prev_rep: T::default(), rep: T::default() }
    }

    pub fn rep(&self) -> &T {
        &self.rep
    }

    pub fn metadata(&self) -> T::Metadata {
        self.rep.metadata().clone()
    }

    pub fn advance(&mut self) {
        self.advance_until(|_| true);
    }

    pub fn advance_until(&mut self, predicate: impl Fn(&Self) -> bool) {
        while let Some(action) = self.cursor.next() {
            match action {
                Action::Setup(ben) => {
                    // FIXME: This is probably how I should tackle proper unapply/unmake?
                    // self.stack.push(self.rep.clone());

                    self.prev_rep = self.rep.clone();
                    self.rep = T::default();
                    self.rep.apply_mut(&Action::Setup(*ben));
                },
                Action::Make(mov) => {
                    self.prev_rep = self.rep.clone();
                    self.rep.apply_mut(&Action::Make(*mov));
                },
                Action::Variation(Delim::Start) => {
                    // save the previous state
                    self.stack.push(self.rep.clone());
                    // and unwind one move
                    self.rep = self.prev_rep.clone();
                },
                Action::Variation(Delim::End) => {
                    self.prev_rep = self.rep.clone();
                    self.rep = self.stack.pop().unwrap();
                },
                Action::Halt(_reason) => {
                    /* noop */
                    // FIXME: should this... do anything?
                },
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
                .make(Move::new(B8, C6, MoveType::QUIET))
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

        assert_eq!(familiar.rep().rep, FEN::new("r1bqkbnr/ppp1pppp/2n5/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 2 3").into());
        assert_eq!(familiar.metadata().fullmove_number, 3);
    }

    #[test]
    fn advance_moves_stepwise() {
        let log = example_game();
        let cursor = log.get_cursor();
        let mut familiar : Familiar<ChessGame<PieceBoard>> = Familiar::new(cursor);

        // Seek to just before the target, then advance by one
        familiar.advance_by(5);
        familiar.advance();

        let f = FEN::from(familiar.rep().rep);
        println!("{}", f);

        assert_eq!(familiar.rep().rep, FEN::new("r1bqkbnr/ppp1pppp/2n5/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 2 3").into());
        assert_eq!(familiar.metadata().fullmove_number, 3);
    }
}

