use std::range::Range;

use super::{cursorlike::Cursorlike, tapelike::Tapelike};


// A zipper-like type over some tapelike. I'm not worried about thread safety just yet, I think
// these things should be broadly 'okay' from a thread safety perspective, since they're mostly
// just updating their local state from some shared resource, but probably some locking is
// necessary.
//
// This is pointerlike, it hsould be possible to pass this around pretty freely, but we'd have to
// copy the reference around, which means Arc, I think.
pub struct Cursor<'a, T> where T : Tapelike {
    tape: &'a T,
    position: usize
}

impl<'a, T> Cursor<'a, T> where T : Tapelike {

    pub fn read_range(&self, range: Range<usize>) -> &'a [T::Item] {
        self.tape.read_range(range)
    }

    pub fn read_context(&self, before: usize, after: usize) -> &'a [T::Item] {
        let start = if before > self.position { 0 } else { self.position - before };
        let end = if self.position + after > self.tape.length() { self.tape.length() } else { self.position + after };

        self.read_range((start..end).into())
    }
}

impl<'a, T, E: 'a> Cursorlike<E> for Cursor<'a, T> where T : Tapelike<Item = E> {
    fn position(&self) -> usize {
        self.position
    }

    fn length(&self) -> usize {
        self.tape.length()
    }

    fn at_end(&self) -> bool {
        self.tape.length() == self.position
    }

    fn read(&self) -> &'a E {
        self.tape.read_address(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn rewind(&mut self) {
        self.position -= 1;
    }
}

pub fn cursor_for<T>(tape: &T) -> Cursor<'_, T> where T : Tapelike {
    Cursor {
        tape,
        position: 0
    }
}
