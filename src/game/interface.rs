use crate::board::interface::{Alter, Query};
use crate::coup::rep::Move;

/// The implementor understands the rules of chess and can make/unmake moves.
///
/// implementing Chess states that the implementor can interpret and produce the result of
/// chess moves as represented by the `Move` type. The `make` and `unmake` methods should be
/// implemented to apply and reverse the move, respectively.
///
/// implementors must also provide a `Default` implementation which represents the starting state
/// of an _empty_ chessboard (no pieces).
pub trait Chess where Self: Sized + Default + Alter + Query {
    fn make(&self, mov: Move) -> Self;
    fn unmake(&self, mov: Move) -> Self;

    fn make_mut(&mut self, mov: Move) -> &mut Self {
        *self = self.make(mov);
        self
    }

    fn unmake_mut(&mut self, mov: Move) -> &mut Self {
        *self = self.unmake(mov);
        self
    }
}

/// The canonical implementation of Chess for any type which is Alterable and Queryable. The
/// algorithm is straightforward:
/// 1. Compile the move in the context of the board, yielding a vector of Alterations.
/// 2. Apply each alteration in sequence to the board, returning the final board state.
///
/// Unmaking is trivial because Alterations are reversible. It's the same algorithm, but applying
/// `inverse` first.
impl<T> Chess for T where T: Alter + Query + Clone + Default {
    fn make(&self, mov: Move) -> T {
        let alterations = mov.compile(self);
        alterations.iter().fold(self.clone(), |board, alteration| board.alter(*alteration))
    }

    fn unmake(&self, mov: Move) -> T {
        let alterations = mov.compile(self);
        alterations.iter().fold(self.clone(), |board, alteration| board.alter(alteration.inverse()))
    }
}
