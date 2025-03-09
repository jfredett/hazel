pub trait Cursorlike<E> {
    fn position(&self) -> usize;
    fn length(&self) -> usize;
    fn at_end(&self) -> bool;

    fn read(&self) -> &E;

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
        while self.position() != desired_position {
            if self.position() < desired_position {
                self.advance();
            } else if self.position() > desired_position {
                self.rewind();
            }
        }
    }

    fn advance_to_end(&mut self) {
        while !self.at_end() {
            self.advance()
        }
    }
}
