
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

    fn seek(&mut self, desired_position: usize) {
        loop {
            if self.position() < desired_position {
                self.advance();
            } else if self.position() > desired_position {
                self.rewind();
            } else { // equal
                break;
            }
        }
    }

    fn advance_to_end(&mut self) {
        while !self.at_end() {
            self.advance()
        }
    }
}
