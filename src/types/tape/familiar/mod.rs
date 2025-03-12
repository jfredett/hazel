use std::{ops::Deref, sync::Arc};

use crate::{Alter, Alteration};

use super::{cursor::Cursor, cursorlike::Cursorlike, tapelike::Tapelike, taperef::TapeRef};

pub mod menagerie;
pub mod state;

/// Calculates a state based on the content of some tapelike. Importantly, the `cursor` should be _replacable_, so that
/// if a familiar runs off the end of a tape, and we have a continuation for that tape in cache, we can replace it's
/// cursor with a new one on the new tape and maintain the state. These should ultimately be sendable between threads, so
/// all their state is maintained internally in a thread-safe way.
pub struct Familiar<T, S> where T : Tapelike {
    cursor: Cursor<T>,
    state: S
}

impl<T,S> Deref for Familiar<T, S> where T : Tapelike {
    type Target = Cursor<T>;

    fn deref(&self) -> &Cursor<T> {
        &self.cursor
    }
}

// OQ: I wonder if it makes sense to `deref` this down to it's state
impl<T, S> Familiar<T, S> where T : Tapelike {
    pub fn get(&self) -> &S {
        &self.state
    }

    pub fn get_mut(&mut self) -> &mut S {
        &mut self.state
    }
}

pub fn conjure_with<T, S>(tapelike: Arc<T>, state: S) -> Familiar<T, S> where T : Tapelike {
    let cursor = Cursor::on_tapelike(tapelike.clone());
    Familiar {
        cursor,
        state
    }
}

pub fn conjure<T, S>(tapelike: Arc<T>) -> Familiar<T,S> where T : Tapelike, S : Default {
    conjure_with(tapelike, S::default())
}

// This impl only covers alteration-without-broader-context updates, equivalent to tapefamiliar but
// not tied to `tape` or any particular element.
impl<T, S> Cursorlike for Familiar<T, S> where T : Tapelike<Item = Alteration>, S : Alter {
    fn position(&self) -> usize {
        self.cursor.position()
    }

    fn length(&self) -> usize {
        self.cursor.length()
    }

    fn at_end(&self) -> bool {
        self.cursor.at_end()
    }

    fn advance(&mut self) {
        self.cursor.advance();

        let alter = self.cursor.read_address(self.cursor.position());
        self.state.alter_mut(alter);
    }

    fn rewind(&mut self) {
        // NOTE: doing this backwards like this makes this an actual inverse of advance, `advance +
        // rewind` should generally be a noop -- but since state updates might not adhere to that,
        // we can't assume.
        let alter = self.cursor.read_address(self.cursor.position());
        self.state.alter_mut(alter.inverse());

        self.cursor.rewind();
    }
}

