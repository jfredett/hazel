use std::sync::RwLock;

use crate::Alteration;

use crate::types::zobrist::Zobrist;


pub struct TapeFamiliar<'a, const SIZE: usize, S> {
    tape: &'a Tape<SIZE>,
    position: usize,
    state: S,
    update: fn(&mut S, Alteration)
}

// struct ZobristFamiliar<'a, const SIZE: usize> = TapeFamiliar<'a,  SIZE>;

// impl<const SIZE: usize, S, 'a> TapeFamiliar<'a, SIZE, S> {


// }

#[derive(Clone)]
pub struct Tape<const SIZE: usize> {
    data: [Entry; SIZE],
    /// A hash corresponding to the last time we saw 
    pub position_hash: Zobrist, // hash up to the last StartTurn
    tape_hash: Zobrist, // hash of the whole tape
    head_hash: Zobrist, // hash up to the current head, from 
    // this is the write head, I might need a familiar for the proceed/unwind stuff?
    head: usize
}

impl<const SIZE: usize> Tape<SIZE> {
    pub fn new() -> Self {
        let mut data = [ Entry::Noop; SIZE ];
        let head = 0;
        let mut tape = Tape { data, head, position_hash: Zobrist::empty(), tape_hash: Zobrist::empty(), head_hash: Zobrist::empty() };
        tape.write_direct(Entry::EOT);

        tape
    }

    pub fn read(&self) -> Option<Alteration> {
        match self.read_direct() {
            Entry::Instruction(alter) => Some(alter),
            _ => None
        }
    }

    pub fn write_all(&mut self, alterations: &[Alteration]) {
        for alter in alterations {
            self.write(*alter);
        }
    }

    pub fn at_bot(&self) -> bool {
        self.head == 0
    }

    pub fn at_eot(&self) -> bool {
        self.read_direct() == Entry::EOT
    }
    
    pub fn erase(&mut self) {
        self.write_direct(Entry::Noop);
    }

    fn read_direct(&self) -> Entry {
        self.data[self.head]
    }

    fn write_direct(&mut self, entry: Entry) {
        self.data[self.head] = entry;
    }


    // ## THIS SECTION NEEDS TO MAINTAIN ALL THE HASHES INCREMENTALLY ## //

    pub fn write(&mut self, alter: Alteration) {
        let current_instruction = self.read();

        // update zobrists to remove current instruction
        // update zobrists to add new instruction

        // write the instruction to the tape
        self.write_direct(Entry::Instruction(alter));
        self.step_forward();

        // if we're at the EOT, bump it forward
        // if we're at the End-of-buffer, cache out

    }

    pub fn step_forward(&mut self) {
        if !self.at_eot() {
            self.head += 1;
            // update head hash, position / table hash as appropriate
        }
    }

    pub fn step_backward(&mut self) {
        if self.head > 0 {
            self.head -= 1;
            // update head hash, position / table hash as appropriate
        }
    }


}


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Entry {
    Instruction(Alteration),
    Noop,
    EOT
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ProceedToken {
    Continue,
    Halt
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn sketch() {
    //     let mut tape = Tape::new();
    //     //  HEAD
    //     // [None, ...]
    //     tape.write(Alteration::clear());
    //     //                           HEAD
    //     // [Some(Alteration::Clear), None, ...
    //     tape.step_back();
    //     //  HEAD
    //     // [Some(Alteration::Clear), None, ...
    //     tape.write(Alteration::Assert(PositionMetadata::default()));
    //     tape.read(); // => Alteration::Assert(PositionMetadata::default())
    //     //                                 HEAD
    //     // [Some(Alteration::Assert(...)), None, ...
    //     tape.step_forward();
    //     //                                       HEAD
    //     // [Some(Alteration::Assert(...)), None, None, ...
    //     tape.step_back(2);
    //     //  HEAD
    //     // [Some(Alteration::Assert(...)), None, None, ...
    //     tape.erase();
    //     //  HEAD
    //     // [None, None, None, ...
    //     tape.seek(2);
    //     //              HEAD
    //     // [None, None, None, ...
    //     tape.seek_to_beginning();
    //     //  HEAD
    //     // [None, None, None, ...
    //     tape.seek_to_end();
    //     //                  HEAD
    //     // ..., None, None, None]

    //     tape.tape_hash() // foldr Zobrist(0) tape (\x acc -> acc.update(x))
    //     tape.position_hash() // zobrist up to the last StartTurn
    
    //     // when writing to tape, whenever `StartTurn` is encountered, the `position zobrist` should
    //     // be updated.

    //     tape.proceed_until(|alter| {
    //         // step forward, yield the alteration to the lambda, repeat so long as this lambda
    //         // returns `Continue`, halt when lambda returns `Halt`.
    //         // This cannot modify the underlying tape while proceeding. Stops when it hits the end
    //         // of the buffer
    //     });
    // }

    // tape write
    // tape read
    // tape overwrite
    // familiar for tape?
    // hash-out strategy
    
}

