use crate::{coup::rep::Move, game::line::Line, notation::Square};


/// Implementors must provide a bijective mapping to a Square
pub trait SquareNotation where Self : Into<Square> + From<Square> {

    fn index(self) -> usize {
        let s : Square = self.into();
        s._index()
    }

    fn file(self) -> usize {
        let s : Square = self.into();
        s._file()
    }

    fn rank(self) -> usize {
        let s : Square = self.into();
        s._rank()
    }
}

/// Implementors must provide a bijective mapping to a Move
pub trait MoveNotation where Self : Into<Move> + From<Move> { }

/// Implementors must provide a bijective mapping to a Line
pub trait LineNotation where Self : Into<Line> + From<Line> { }

/* equivalent to above, but also imports variations
pub trait StudyNotation {
}
*/

