use crate::{Alter, Alteration};

use super::{cursor::Cursor, cursorlike::Cursorlike, tapelike::Tapelike};

pub mod menagerie;
pub mod position_zobrist;

/// Calculates a state based on the content of some tapelike. Importantly, the `cursor` should be _replacable_, so that
/// if a familiar runs off the end of a tape, and we have a continuation for that tape in cache, we can replace it's
/// cursor with a new one on the new tape and maintain the state. These should ultimately be sendable between threads, so
/// all their state is maintained internally in a thread-safe way.
pub struct Familiar<'a, T, S> where T : Tapelike {
    cursor: Cursor<'a, T>,
    state: S
}

// FIXME: This API is trash, it's type signature -- wack, it's leaking abstractions? Wack. It's
// qualified reference to other functions which also have a bad API. Wack.
// This is truly the Drake of APIs.
pub fn conjure<S, T>(tape: &T) -> Familiar<T, S> where T : Tapelike, S : Default {
    Familiar {
        cursor: super::cursor::cursor_for(tape),
        state: S::default()
    }
}


// OQ: I wonder if it makes sense to `deref` this down to it's state
impl<'a, T, S> Familiar<'a, T ,S> where T : Tapelike {
    pub fn get<'b>(&'b self) -> &'b S  where 'b : 'a {
        &self.state
    }

    pub fn get_mut<'b>(&'b mut self) -> &'b mut S where 'b : 'a {
        &mut self.state
    }
}


// This impl only covers alteration-without-broader-context updates, equivalent to tapefamiliar but
// not tied to `tape` or any particular element.
impl<T, S> Cursorlike<Alteration> for Familiar<'_, T, S> where T : Tapelike<Item = Alteration>, S : Alter {
    fn position(&self) -> usize {
        self.cursor.position()
    }

    fn read(&self) -> &'_ Alteration {
        self.cursor.read()
    }

    fn advance(&mut self) {
        self.cursor.advance();

        let alter = self.cursor.read();
        self.state.alter_mut(*alter);
    }

    fn rewind(&mut self) {
        // NOTE: doing this backwards like this makes this an actual inverse of advance, `advance +
        // rewind` should generally be a noop -- but since state updates might not adhere to that,
        // we can't assume.
        let alter = self.cursor.read();
        self.state.alter_mut(alter.inverse());

        self.cursor.rewind();
    }
}

