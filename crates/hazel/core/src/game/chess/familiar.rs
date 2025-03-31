
use hazel_basic::ben::BEN;

use crate::{coup::rep::Move, interface::play::Play, types::{log::cursor::Cursor, movesheet::MoveSheet}};
use super::{action::Action, delim::Delim};

// TODO: port this to the new system.
#[derive(Debug, Clone)]
pub struct Familiar<'a> {
    // TODO: Temporarily fixing the types
    cursor: Cursor<'a, Action<Move, BEN>>,
    movesheets: Vec<MoveSheet>,
}


impl<'a> Familiar<'a> {
    pub fn new(cursor: Cursor<'a, Action<Move, BEN>>) -> Self {
        Self { cursor, movesheets: vec![MoveSheet::default()] }
    }

    pub fn rep<T>(&'a self) -> T where T: Play + From<&'a MoveSheet> {
        let movesheet = self.movesheets.last().unwrap();
        T::from(movesheet)
    }

    pub fn metadata<T>(&'a self) -> T::Metadata where T: Play + From<&'a MoveSheet> {
        self.rep::<T>().metadata()
    }

    pub fn current_move(&self) -> Option<Move> {
        let movesheet = self.movesheet();
        movesheet.current_move()
    }

    pub fn rewind_until(&mut self, mut predicate: impl FnMut(&Self) -> bool) {
        // verify we don't already satisfy the predicate
        if predicate(self) { return; }

        while let Some(action) = self.cursor.prev() {
            match action {
                Action::Setup(_) => {
                    self.movesheets.pop();
                }
                _ => { self.movesheet_mut().unwind(); }
            }

            if predicate(self) {
                break;
            }
        }
    }

    pub fn rewind(&mut self) {
        self.rewind_by(1);
    }

    pub fn rewind_to_start(&mut self) {
        self.rewind_until(|_| false);
    }

    pub fn rewind_by(&mut self, count: usize) {
        let target = self.cursor_position() - count;
        self.rewind_until(|f| f.cursor_position() == target);
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor.position()
    }

    /// Given a predicate, advance the underlying cursor until the predicate is satisfied.
    /// As each action is touched, update the representation.
    pub fn advance_until(&mut self, mut predicate: impl FnMut(&Self) -> bool) {
        // verify we don't already satisfy the predicate
        if predicate(self) { return; }

        loop {
            let Some(action) = self.cursor.next() else {
                break;
            };

            match action.clone() {
                Action::Setup(ben) => {
                    let mut new_sheet = MoveSheet::default();
                    new_sheet.set_initial_state(ben);
                    self.movesheets.push(new_sheet);
                },
                Action::Make(mov) => {
                    self.movesheet_mut().record(mov);
                },
                Action::Variation(Delim::Start) => {
                    self.movesheet_mut().branch();
                },
                Action::Variation(Delim::End) => {
                    self.movesheet_mut().prune();
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

    pub fn advance(&mut self) {
        self.advance_by(1);
    }

    pub fn advance_to_end(&mut self) {
        self.advance_until(|_| false);
    }

    pub fn advance_by(&mut self, count: usize) {
        let target = self.cursor_position() + count;
        self.advance_until(|f| f.cursor_position() == target);
    }

    pub fn move_string(&self) -> String {
        self.movesheet().last_move_string().unwrap_or_default()
    }

    fn movesheet_mut(&mut self) -> &mut MoveSheet {
        self.movesheets.last_mut().unwrap()
    }

    fn movesheet(&self) -> &MoveSheet {
        self.movesheets.last().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use hazel_basic::square::*;
    use crate::{board::PieceBoard, constants::START_POSITION_FEN, coup::rep::{Move, MoveType}, game::{chess::PositionMetadata, variation::Variation, ChessGame}};

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

    #[quickcheck]
    fn rewind_and_advance_are_inverses(amt: usize) -> bool {
        let log = example_game();
        let cursor = log.get_cursor();
        let mut familiar = Familiar::new(cursor);

        if ![1, 6].contains(&amt) {
            return true;
        }


        let initial_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

        familiar.advance_by(amt);
        familiar.rewind_by(amt);

        let after_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;


        initial_rep == after_rep
    }



    mod rewind {
        use crate::constants::EMPTY_POSITION_FEN;

        use super::*;

        #[quickcheck]
        fn rewinding_by_nothing_is_a_noop(amt: usize) -> bool {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            if ![1, 6].contains(&amt) {
                return true;
            }

            familiar.advance_by(amt);

            let initial_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

            familiar.rewind_by(0);

            let after_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

            initial_rep == after_rep
        }
        #[test]
        fn rewind_can_rewind_past_a_variation() {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            // find the variation
            familiar.advance_by(5);

            let prevariation_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

            // enter it
            familiar.advance();

            let variation_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

            assert_ne!(prevariation_rep, variation_rep);

            familiar.rewind();

            let postvariation_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

            assert_eq!(prevariation_rep, postvariation_rep);
        }

        #[test]
        fn rewind_can_rewind() {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            // Setup is the 'zeroth' action, and we proceed actionwise. At the moment that also
            // corresponds to ply number, but the example game has variations so that is not reliable.
            familiar.advance_by(3);
            familiar.rewind();

            let ben = BEN::new("rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq d6 0 2");
            let mut expected_board = PieceBoard::default();
            expected_board.set_fen(ben);

            assert_eq!(familiar.rep::<ChessGame<PieceBoard>>().rep, expected_board);

            let metadata : PositionMetadata = familiar.metadata::<ChessGame<PieceBoard>>();

            assert_eq!(metadata, ben.metadata());
        }

        #[test]
        fn rewind_to_beginning_rewinds_to_beginning() {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);


            // Setup is the 'zeroth' action, and we proceed actionwise. At the moment that also
            // corresponds to ply number, but the example game has variations so that is not reliable.
            familiar.advance_by(3);
            familiar.rewind_to_start();

            let ben = BEN::new(EMPTY_POSITION_FEN);
            let mut expected_board = PieceBoard::default();
            expected_board.set_fen(ben);

            assert_eq!(familiar.rep::<ChessGame<PieceBoard>>().rep, expected_board);

            let metadata : PositionMetadata = familiar.metadata::<ChessGame<PieceBoard>>();

            assert_eq!(metadata, ben.metadata());
        }

        #[test]
        fn rewind_is_rewind_by_one() {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            // calculate once by rewind_by
            familiar.advance_by(3);
            familiar.rewind_by(1);

            let first_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

            // then again by rewind
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            familiar.advance_by(3);
            familiar.rewind();

            let second_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;
            assert_eq!(first_rep, second_rep);
        }
    }

    mod advance {
        use super::*;

        #[quickcheck]
        fn advancing_by_nothing_is_a_noop(amt: usize) -> bool {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            if ![1, 6].contains(&amt) {
                return true;
            }

            familiar.advance_by(amt);

            let initial_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

            familiar.advance_by(0);

            let after_rep = familiar.rep::<ChessGame<PieceBoard>>().rep;

            initial_rep == after_rep
        }

        #[test]
        fn familiar_works_with_pieceboard_to_capture_gamestate() {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            // Setup is the 'zeroth' action, and we proceed actionwise. At the moment that also
            // corresponds to ply number, but the example game has variations so that is not reliable.
            familiar.advance_by(2);

            let ben = BEN::new("rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq d6 0 2");
            let mut expected_board = PieceBoard::default();
            expected_board.set_fen(ben);

            assert_eq!(familiar.rep::<ChessGame<PieceBoard>>().rep, expected_board);

            let metadata : PositionMetadata = familiar.metadata::<ChessGame<PieceBoard>>();

            assert_eq!(metadata, ben.metadata());
        }

        #[test]
        fn familiar_finds_the_variation_position() {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            // We advance _over_ the variation opening, but stop inside.
            familiar.advance_by(6);

            let ben = BEN::new("r1bqkbnr/ppp1pppp/2n5/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 2 3");
            let mut expected_board = PieceBoard::default();
            expected_board.set_fen(ben);

            let metadata : PositionMetadata = familiar.metadata::<ChessGame<PieceBoard>>();

            assert_eq!(metadata, ben.metadata());
        }

        #[test]
        fn advance_moves_stepwise() {
            let log = example_game();
            let cursor = log.get_cursor();
            let mut familiar = Familiar::new(cursor);

            // Seek to just before the target, then advance by one
            familiar.advance_by(5);
            familiar.advance();

            let ben = BEN::new("r1bqkbnr/ppp1pppp/2n5/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 2 3");
            let mut expected_board = PieceBoard::default();
            expected_board.set_fen(ben);

            let metadata : PositionMetadata = familiar.metadata::<ChessGame<PieceBoard>>();

            assert_eq!(metadata, ben.metadata());
        }
    }

}
