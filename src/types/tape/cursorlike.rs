
// FIXME: I might just remove this trait altogether, and use Deref to allow easier access to the
// underlying APIs? Familiar -> Cursor -> Tape?
pub trait Cursorlike {
    fn position(&self) -> usize;
    fn length(&self) -> usize;
    fn at_end(&self) -> bool;

    fn advance(&mut self);
    fn rewind(&mut self);

    fn advance_until(&mut self, pred: impl Fn(&Self) -> bool) {
        loop {
            self.advance();
            if pred(self) { break ; }
        }
    }

    fn rewind_until(&mut self, pred: impl Fn(&Self) -> bool) {
        // I think on rewind the check needs to come after, not before
        loop {
            tracing::trace!("current pos: {:#04X}", self.position());
            self.rewind();
            // HACK: This feels bad, but it works.
            if pred(self) { self.rewind(); break; }
        }
    }

    /// advance/rewind until the `desired_position` is reached, maintaining state along the way.
    fn seek(&mut self, desired_position: usize) {
        use std::cmp::Ordering::*;
        match self.position().cmp(&desired_position) {
            Less => {
                self.advance_until(|me| me.position() == desired_position + 1);
            }
            Greater => {
                self.rewind_until(|me| me.position() == desired_position);
            }
            _ => {}
        }
    }

    /// jump immediately (no state maintenance) to the `desired_position`
    fn jump(&mut self, desired_position: usize);

    fn advance_to_end(&mut self) {
        while !self.at_end() {
            self.advance()
        }
    }

}
