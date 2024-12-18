use crate::{coup::rep::Move, interface::play::Play, notation::ben::BEN, types::{log::cursor::Cursor, movesheet::MoveSheet}};
use super::{action::Action, delim::Delim};

#[derive(Debug, Clone)]
pub struct Familiar<'a, T> where T : Play + Default {
    // TODO: Temporarily fixing the types
    //
    // I think the solution here is to think about BEN/FEN etc in the context of turning into a
    // 'gamestate' of some kind. ChessGame is an example of a GameState. a vector of usizes would
    // be a gamestate for nim. A vector of bool? would be a gamestate for tic-tac-toe. A game state
    //
    // The familiar should probably handle the action destructuring, and maintain a cache of
    // gamestates that are available to the various strategies? I'm thinking in the context of the
    // advance/rewind strategies, which I guess are really "calculate the representation at a point
    // relative to the 'current' one." So the state of the cursor would naturally drift, as it
    // expects to be at a certain location. I suppose the cursor can simply jump over to whereever
    // the 'advance/rewind' strategy wants it to be as a preparation step.
    //
    // Ultimately it'd be nice to write metadata into the log via a familiar with a WriteHead,
    // strategies should ignore any action it doesn't care about.
    //
    // 
    cursor: Cursor<'a, Action<Move, BEN>>,
    movesheet: MoveSheet,
    setup_stack: Vec<BEN>,
    stack: Vec<T>,
    prev_rep: T,
    rep: T,
}

// cursor
//

impl<'a, T> Familiar<'a, T> where T : Play + Default + From<&'a MoveSheet> {
    pub fn new(cursor: Cursor<'a, Action<Move, BEN>>) -> Self {
        Self { cursor, stack: vec![], setup_stack: vec![], movesheet: MoveSheet::default(), prev_rep: T::default(), rep: T::default() }
    }

    pub fn rep(&'a self) -> T {
        T::from(&self.movesheet)
    }

    pub fn metadata(&'a self) -> T::Metadata {
        self.rep().metadata()
    }

    pub fn advance(&mut self) {
        self.advance_until(|_| true);
    }

    // pub fn scan_backward(&mut self, predicate: impl Fn(&Self) -> bool) -> (usize, Action<Move, BEN>) {
    //     let original = self.clone();

    //     while !predicate(self) {
    //         self.cursor.prev();
    //     }
    //     let found_position = self.cursor.position();

    //     *self = original;

    //     (found_position, action)
    // }

    // pub fn scan_forward(&mut self, predicate: impl Fn(&Self) -> bool) -> (usize, Action<Move, BEN>) {
    //     let original = self.clone();

    //     while !predicate(self) {
    //         self.cursor.next();
    //     }

    //     let found_position = self.cursor.position();
    //     let action = self.cursor.current().unwrap();

    //     *self = original;

    //     (found_position, action)
    // }

    // Rewind really wants it's own stack/prevrep thing.
    //
    // What if I approached this and advance more as a lazy evaluation thing. Push a stack of
    // actions to the 'do' or 'undo' stack. When I call `rep`, do all the stuff in the stack, if
    // it's a big difference, I can jump to a nearby saved position and replay from there.
    //
    // This would make it so that these functions are much simpler, and I can extract all the state
    // calculation logic to a single function that reconciles those stacks.
    //
    // alternatively, I could extract this to a function that takes a predicate that produces a
    // _direction_. The predicate would evaluate the current state and return a direction to move
    // in. The function would then move the cursor in the direction indicated either by:
    //
    // if the 'todo' stack is empty, pushing the instruction onto the do stack and marking the
    // direction we are travelling (forward or backward). If backward, this is a sequence of
    // actions to _undo_, if forward, it's actions to _do_ to the state.
    //
    // if the todo stack is not empty, and we are moving in the opposite direction, we pop the
    // stack and do nothing, if we are moving in the same direction, we push the instruction onto
    // the stack.
    //
    // The function can potentially nest, that is, it can recursively call the 'step' function.
    // This should be fine.
    //
    // I suppose this could be done with a lagging cursor, or just recording the position of the
    // last evaluation of the rep? Then undoing should be trivial.
    //
    // Yah, hold on.
    //
    // If I have a rep like:
    //
    // F {
    //   cursor: Cursor<...>
    //   rep: (usize, T)
    //   stack: Vec<(usize, T)>
    // }
    //
    // Then we just delay calculating the rep until we need it, at which time we either roll
    // forward, building up the variation stack as needed. Then when we roll back over the
    // variation delimiter, we pop the stack and continue. Unmaking moves will require the current
    // rep, so most of the time the cursor will be kept current, but while calculating some other final
    // position, we only pay for cursor moves until the end, when we pay to calculate the
    // representation once.
    //
    // This would centralize all that to one point so it's easy to access caches and the like
    // later.
    //
    // The predicate should be (self -> Proceed) Proceed is an enum, it can be Forward, Backward,
    // Directly(position), etc. Eventually the predicate should be specified in a DSL.
    pub fn rewind_until(&mut self, predicate: impl Fn(&Self) -> bool) {
        while let Some(action) = self.cursor.prev() {
            match action {
                Action::Setup(ben) => {
                    todo!();

                    // // mark where we are
                    // let cur_pos = self.cursor.position();
                    // // find the previous setup instruction
                    // let (pos, setup) = self.scan_backward(|f| matches!(f.cursor.current(), Some(Action::Setup(_))));
                    // // the previous rep is probably 'wrong' now, this points towards the strategy
                    // // impl
                    // self.prev_rep = ben.into();

                    // // move the cursor back to the setup
                    // self.cursor.seek(pos);
                    // // set up the board
                    // self.rep = T::default().apply_mut(&setup);

                    // // FIXME: Maybe offbyone?

                    // // advance back to the instruction we were at less one
                    // self.advance_by(cur_pos - (pos + 1));
                }
                Action::Make(mov) => {
                    todo!()
                }
                Action::Variation(Delim::Start) => {
                    todo!()
                }
                Action::Variation(Delim::End) => {
                    todo!()
                }
                Action::Halt(_reason) => {
                    todo!()
                }
            }

            if predicate(self) {
                break;
            }
        }

    }

    /// Given a predicate, advance the underlying cursor until the predicate is satisfied.
    /// As each action is touched, update the representation.
    ///
    /// TODO: This and #rewind_until are very similar, and could be refactored into a single
    /// function that takes a direction. Additionally, the action handling code can and probably
    /// should be a strategy that the familiar can use. So that the eventual API would be something
    /// like:
    ///
    /// let f = Familiar::new(cursor);
    /// f.proceed_via(Strategy::Advance::new(), |f| f.cursor.position() == 5);
    ///
    /// The strategy would include a direction parameter and action handler. This could replace the
    /// current pile of methods I have below. Strategy _could_ be built to hold the predicate as
    /// well, so maybe something like:
    ///
    /// f.proceed_via(Strategy::Advance::new(|f| f.cursor.position() == 5));
    ///
    /// is better?
    ///
    /// This would also allow for doing:
    ///
    /// f.proceed_via(Strategy::AdvanceToEnd::new());
    ///
    /// with no extra parameter to None out.
    ///
    /// The strategy can then _also_ contain all the gamestate information. A familiar could cache
    /// certain strategies, or the strategy could outlive the familiar that runs is and capture any
    /// state information to be re-injected later, potentially. The idea is that there would be
    /// strategies for indexing and otherwise analyzing the Variation. So there might be a
    /// strategy like, "Write every move to a BEN in this file" or "Print the evaluation at each
    /// position for White" or "Turn this Log into a PGN".
    ///
    /// Hazel would then spawn a bunch of familiars in threads, and then pass a cache of strategies
    /// to them. The strategy would configure the familiar, run it's proceed-via function, then
    /// return to the Hazel engine with an updated state. Hazel then can do whatever secondary
    /// strategy stuff it needs to do with the information it gathers. It can 'query plan' to
    /// update whatever cached strategies it has and then return whatever result it needs to
    /// return.
    pub fn advance_until(&mut self, predicate: impl Fn(&Self) -> bool) {
        while let Some(action) = self.cursor.next() {
            match action {
                Action::Setup(ben) => {
                    self.setup_stack.push(*ben);
                    self.movesheet.set_initial_state(*ben);
                },
                Action::Make(mov) => {
                    self.movesheet.record(*mov);
                },
                Action::Variation(Delim::Start) => {
                    self.movesheet.branch();
                },
                Action::Variation(Delim::End) => {
                    self.movesheet.prune();
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

    // another advance/rewind design:
    //
    // stack-of-moves that is lazily evaluated. As we progress through the variation, we push each
    // move onto a stack, when we ask for the rep, we iterate over the stack of moves. When
    // entering a variation, we:
    //
    // make a copy of the stack and push it onto the history stack
    // pop the top move
    // push a 'stop' token
    // proceed
    //
    // a this should probably be an abstract type
    //
    // At any time, the current stack is the correct current mainline
    //
    // Rewinding can only take place if advancing has, and it's always just a matter of
    // manipulating the move stack. When rewining, if I hit the stop token, I know I need to pull
    // from the history stack.


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

