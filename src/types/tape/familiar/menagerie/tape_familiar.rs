
// TODO: This is legacy, it should get folded into the more generic familiar thing above

use crate::{types::tape::{tape_direction::TapeDirection, tapelike::Tapelike, Tape}, Alter, Alteration};

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
        let alter = self.tape.read_address(self.position);
        (self.update)(&mut self.state, TapeDirection::Advancing, *alter);
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
        loop {
            let alter = self.tape.read_address(self.position);
            self.rewind(); // rewind is inclusive
            if !predicate(*alter) { break; }
        }
    }

    pub fn rewind(&mut self) {
        let alter = self.tape.read_address(self.position);
        (self.update)(&mut self.state, TapeDirection::Rewinding, *alter);

        self.position -= 1;
    }

    pub fn get<'b>(&'b self) -> &'b S  where 'b : 'a {
        &self.state
    }

    pub fn get_mut<'b>(&'b mut self) -> &'b mut S where 'b : 'a {
        &mut self.state
    }
}
