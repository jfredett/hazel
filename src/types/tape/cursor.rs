use std::{ops::Deref, sync::Arc};

use super::{cursorlike::Cursorlike, tapelike::Tapelike};

// A zipper-like type over some tapelike. I'm not worried about thread safety just yet, I think
// these things should be broadly 'okay' from a thread safety perspective, since they're mostly
// just updating their local state from some shared resource, but probably some locking is
// necessary.
//
// This is pointerlike, it hsould be possible to pass this around pretty freely, but we'd have to
// copy the reference around, which means Arc, I think.
pub struct Cursor<T> where T : Tapelike {
    tape: Arc<T>,
    position: usize
}

impl<T> Deref for Cursor<T> where T : Tapelike {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.tape
    }
}

impl<T> Cursor<T> where T : Tapelike {
    pub fn read_context(&self, before: usize, after: usize) -> dynamic_array::SmallArray<T::Item> {
        let start = if before > self.position { 0 } else { self.position - before };
        let end = if self.position + after > self.tape.length() { self.tape.length() } else { self.position + after };

        self.read_range(start..end)
    }

    pub fn on_tapelike(tapelike: Arc<T>) -> Self {
        Cursor {
            tape: tapelike.clone(),
            position: 0
        }
    }

    pub fn sync_to_writehead(&mut self) {
        tracing::trace!("Seeking to {:#05X}", self.tape.writehead());
        self.seek(self.tape.writehead());
    }
}

impl<T> Cursorlike for Cursor<T> where T : Tapelike {
    fn position(&self) -> usize {
        self.position
    }

    fn length(&self) -> usize {
        self.tape.length()
    }

    fn jump(&mut self, desired_position: usize) {
        self.position = if desired_position < self.length() {
            desired_position
        } else {
            self.length() - 1
        }
    }

    fn at_end(&self) -> bool {
        self.tape.length() == self.position
    }

    fn advance(&mut self) {
        tracing::trace!("in cursor advance"); 
        self.position += 1;
    }

    fn rewind(&mut self) {
        tracing::trace!("in cursor rewind"); 
        self.position -= 1;
    }
}
