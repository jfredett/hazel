use crate::{Alter, Alteration};

use super::{cursor::Cursor, cursorlike::Cursorlike, Tape, tape_direction::TapeDirection, tapelike::Tapelike};

/// Calculates a state based on the content of some tapelike. Importantly, the `cursor` should be _replacable_, so that
/// if a familiar runs off the end of a tape, and we have a continuation for that tape in cache, we can replace it's
/// cursor with a new one on the new tape and maintain the state. These should ultimately be sendable between threads, so
/// all their state is maintained internally in a thread-safe way.
// OQ: I wonder if it makes sense to `deref` this down to it's state
pub struct Familiar<'a, T, S> where T : Tapelike {
    cursor: Cursor<'a, T>,
    state: S
}

pub fn conjure<S, T>(tape: &T) -> Familiar<T, S> where T : Tapelike, S : Default {
    Familiar {
        cursor: super::cursor::cursor_for(tape),
        state: S::default()
    }
}

// TODO: This is legacy, it should get folded into the more generic familiar thing above

#[derive(Debug)]
pub struct TapeFamiliar<'a, S> {
    tape: &'a Tape,
    position: usize,
    state: S,
    update: fn(&mut S, TapeDirection, Alteration)
}


impl<'a, S> TapeFamiliar<'a, S> {
    pub fn new(tape: &'a Tape, state: S, update: fn(&mut S, TapeDirection, Alteration)) -> Self {
        TapeFamiliar {
            tape,
            position: 0,
            state,
            update
        }
    }

    pub fn for_alterable_with_state(tape: &'a Tape, state: S) -> Self where S : Alter {
        TapeFamiliar::new(
            tape,
            state,
            |state : &mut S, direction, alter| {
                if direction.advancing() {
                    state.alter_mut(alter);
                } else {
                    // NOTE: I ~think~ thought this might be double-inverting some stuff? I need to decide
                    // where the inversion happens, here or unmake
                    // - update: Not an issue, because the board isn't maintained as a familiar
                    // (yet).
                    state.alter_mut(alter.inverse());
                }
            }
        )
    }

    pub fn for_alterable(tape: &'a Tape) -> Self where S : Default + Alter {
        TapeFamiliar::for_alterable_with_state(tape, S::default())
    }

    pub fn advance(&mut self) {
        self.position += 1;
        if let Some(alter) = self.tape.read_address(self.position) {
            (self.update)(&mut self.state, TapeDirection::Advancing, alter);
        }
    }

    pub fn sync_to_read_head(&mut self) {
        while self.position != self.tape.read_head() {
            if self.position < self.tape.read_head() {
                self.advance();
            } else if self.position > self.tape.read_head() {
                self.rewind();
            }
        }
    }

    pub fn rewind_by(&mut self, count: usize) {
        for _ in 0..count {
            self.rewind();
        }
    }

    pub fn rewind_until(&mut self, predicate: fn(Alteration) -> bool) {
        while let Some(alter) = self.tape.read_address(self.position) && !predicate(alter) {
            self.rewind();
        }
        self.rewind(); // rewind is inclusive
    }

    pub fn rewind(&mut self) {
        self.position -= 1;
        if let Some(alter) = self.tape.read_address(self.position) {
            (self.update)(&mut self.state, TapeDirection::Rewinding, alter);
        }
    }

    pub fn get<'b>(&'b self) -> &'b S  where 'b : 'a {
        &self.state
    }

    pub fn get_mut<'b>(&'b mut self) -> &'b mut S where 'b : 'a {
        &mut self.state
    }
}

// This impl only covers alteration-without-broader-context updates, equivalent to tapefamiliar but
// not tied to `tape` or any particular element.
impl<T, S> Cursorlike for Familiar<'_, T, S> where T : Tapelike<Item = Alteration>, S : Alter {
    fn position(&self) -> usize {
        self.cursor.position()
    }

    fn advance(&mut self) {
        self.cursor.advance();
        if let Some(alter) = self.cursor.read() {
            self.state.alter_mut(*alter);
        }
    }

    fn rewind(&mut self) {
        // NOTE: doing this backwards like this makes this an actual inverse of advance, `advance +
        // rewind` should generally be a noop -- but since state updates might not adhere to that,
        // we can't assume.
        if let Some(alter) = self.cursor.read() {
            self.state.alter_mut(alter.inverse());
        }
        self.cursor.rewind();
    }
}

