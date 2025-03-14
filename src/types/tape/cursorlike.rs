
// FIXME: I might just remove this trait altogether, and use Deref to allow easier access to the
// underlying APIs? Familiar -> Cursor -> Tape?
pub trait Cursorlike {
    fn position(&self) -> usize;
    fn length(&self) -> usize;
    fn at_end(&self) -> bool;

    fn advance(&mut self);
    fn rewind(&mut self);

    fn advance_until(&mut self, pred: fn(&Self) -> bool) {
        while !pred(self) {
            self.advance()
        }
    }

    fn rewind_until(&mut self, pred: fn(&Self) -> bool) {
        while !pred(self) {
            self.rewind()
        }
    }

    /// advance/rewind until the `desired_position` is reached, maintaining state along the way.
    fn seek(&mut self, desired_position: usize) {
        loop {
            match self.position().cmp(&desired_position) {
                // TODO: Replace with an advance_until and remove the outer loop.
                std::cmp::Ordering::Less => self.advance(),
                std::cmp::Ordering::Equal => break,
                // TODO: Replace with an rewind_until and remove the outer loop.
                std::cmp::Ordering::Greater => self.rewind(),
            }
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
