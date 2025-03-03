use std::sync::RwLock;
use std::fmt::Debug;

use crate::alteration::MetadataAssertion;
use crate::{Alter, Alteration};

use crate::types::zobrist::Zobrist;


#[derive(Debug)]
pub struct TapeFamiliar<'a, const SIZE: usize, S> {
    tape: &'a Tape<SIZE>,
    position: usize,
    state: S,
    update: fn(&mut S, TapeDirection, Alteration)
}

#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum TapeDirection {
    Advancing,
    Rewinding
}

impl TapeDirection {
    pub fn advancing(self) -> bool {
        self == TapeDirection::Advancing
    }

    pub fn rewinding(self) -> bool {
        self == TapeDirection::Rewinding
    }
}

impl<'a, const SIZE: usize, S> TapeFamiliar<'a, SIZE, S> {
    pub fn new(tape: &'a Tape<SIZE>, state: S, update: fn(&mut S, TapeDirection, Alteration)) -> Self {
        TapeFamiliar {
            tape,
            position: 0,
            state,
            update
        }
    }

    pub fn for_alterable_with_state(tape: &'a Tape<SIZE>, state: S) -> Self where S : Alter {
        TapeFamiliar::new(
            tape,
            state,
            |state : &mut S, direction, alter| {
                if direction.advancing() {
                    state.alter_mut(alter);
                } else {
                    state.alter_mut(alter.inverse());
                }
            }
        )
    }

    pub fn for_alterable(tape: &'a Tape<SIZE>) -> Self where S : Default + Alter {
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

    pub fn rewind(&mut self) {
        self.position -= 1;
        if let Some(alter) = self.tape.read_address(self.position) {
            (self.update)(&mut self.state, TapeDirection::Rewinding, alter);
        }
    }

    pub fn get<'b>(&'b self) -> &'b S  where 'b : 'a {
        &self.state
    }
}


#[derive(Default, Clone, Copy)]
struct PositionZobrist {
    pub current: Zobrist,
    pub position: Zobrist
}

impl Alter for PositionZobrist {
    fn alter_mut(&mut self, alter: Alteration) -> &mut Self {
        self.current.alter_mut(alter);

        if matches!(alter, Alteration::Assert(MetadataAssertion::StartTurn(_))) || matches!(alter, Alteration::InitialMetadata(_)) {
            tracing::debug!("Updating Position Hash current hash: {:?}", self.current);
            self.position = self.current;
        }
        self
    }

    fn alter(&self, alter: Alteration) -> Self {
        let mut ret = *self;
        ret.alter_mut(alter);
        ret
    }
}

// struct ZobristFamiliar<'a, const SIZE: usize> = TapeFamiliar<'a,  SIZE>;

// impl<const SIZE: usize, S, 'a> TapeFamiliar<'a, SIZE, S> {


// }

#[derive(Clone)]
pub struct Tape<const SIZE: usize> {
    data: [Option<Alteration>; SIZE],
    // this is the write head, I might need a familiar for the proceed/unwind stuff?
    head: usize
}

impl<const SIZE: usize> Debug for Tape<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nTAPE(head_hash: {:?}, position_hash: {:?}, head: {:#04x})", self.head_hash(), self.position_hash(), self.head);
        let mut running_hash = Zobrist::empty();
        for (idx, entry) in self.data.into_iter().enumerate() {
            match entry {
                None => {
                    // if self.head >= idx {
                    //     writeln!(f, "... {:?} NOOP", self.head - idx - 1);
                    //     writeln!(f, "{:#008X} | NOOP", self.head - 1);
                    //     writeln!(f, "HEAD*   | NOOP");
                    //     writeln!(f, "... {:?} NOOP", SIZE - self.head);
                    // }
                    writeln!(f, "END-OF-TAPE");
                    break;
                }
                Some(alter) => {
                    running_hash.alter_mut(alter);
                    if idx == self.head {
                        writeln!(f, "HEAD*    | {:>width$} | {:?}", alter, running_hash, width = 32);
                    } else {
                        writeln!(f, "{:>#008X} | {:>width$} | {:?}", idx, alter, running_hash, width = 32);
                    }
                }
                _ => { continue; }
            }
        }
        writeln!(f, "=================================")
    }

}

impl<const SIZE: usize> Tape<SIZE> {
    pub fn new() -> Self {
        Tape { data: [ None; SIZE ], head: 0 }
    }

    // TODO: OQ: Same as head_hash
    pub fn position_hash(&self) -> Zobrist {
        let mut familiar : TapeFamiliar<'_, SIZE, PositionZobrist> = self.conjure();
        tracing::debug!("syncing position hash");
        familiar.sync_to_read_head();
        familiar.state.position
    }

    /// the point to which the tape is valid
    pub fn read_head(&self) -> usize {
        self.write_head() - 1
    }

    /// the next empty slot to write to
    pub fn write_head(&self) -> usize {
        self.head
    }

    // TODO: OQ: possibly these live _on position_?
    pub fn head_hash(&self) -> Zobrist {
        // TODO: Cache this and advance on demand
        let mut familiar : TapeFamiliar<'_, SIZE, Zobrist> = self.conjure();
        // this will change to look at the current write position (head) and moving towards it
        // updating the hash on the way.
        familiar.sync_to_read_head();
        familiar.state
    }

    pub fn conjure<A : Alter + Default>(&self) -> TapeFamiliar<SIZE, A> {
        TapeFamiliar::for_alterable(&self)
    }

    pub fn conjure_with_initial_state<A : Alter>(&self, state: A) -> TapeFamiliar<SIZE, A> {
        TapeFamiliar::for_alterable_with_state(&self, state)
    }

    pub fn read(&self) -> Option<Alteration> {
        self.read_address(self.head)
    }

    pub fn write_all(&mut self, alterations: &[Alteration]) {
        for alter in alterations {
            self.write(*alter);
        }
    }

    pub fn read_address(&self, idx: usize) -> Option<Alteration> {
        if idx >= SIZE {
            None
        } else {
            self.data[idx]
        }
    }


    fn write_all_direct(&mut self, entries: &[Option<Alteration>]) {
        for entry in entries {
            self.write_direct(*entry);
            self.head += 1;
        }
    }

    fn write_direct(&mut self, entry: Option<Alteration>) {
        self.data[self.head] = entry;
    }

    // ## THIS SECTION NEEDS TO MAINTAIN ALL THE HASHES INCREMENTALLY ## //

    pub fn write(&mut self, alter: Alteration) {
        tracing::debug!("Writing {:?}", alter);
        // if we're at the End-of-buffer, cache out
        if self.at_eot() {
            tracing::trace!("CACHE OUT CACHE OUT CACHE OUT");
            // TODO: Actually cache out, this just blanks the buffer and recurses.
            self.data = [None; SIZE];
            self.head = 0;
            self.write(alter);
        }

        // write the instruction to the tape
        self.write_direct(Some(alter));
        self.head += 1;
    }

    pub fn at_bot(&self) -> bool {
        self.head == 0
    }

    pub fn at_eot(&self) -> bool {
        // watch out for off-by-ones!
        self.head == (SIZE - 1)
    }

    pub fn step_forward(&mut self) {
        if !self.at_eot() {
            self.head += 1;
        }
    }

    pub fn step_backward(&mut self) {
        if !self.at_bot() {
            self.head -= 1;
        }
    }


}

impl<const SIZE: usize> Default for Tape<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ProceedToken {
    Continue,
    Halt
}

#[cfg(test)]
mod tests {
    use crate::alteration::MetadataAssertion;
    use crate::types::zobrist::HazelZobrist;
    use crate::types::Color;
    use crate::{notation::ben::BEN, types::Occupant};
    use crate::notation::*;

    use super::*;

    fn tape_with_startpos_and_d4() -> Tape<128> {
        let mut raw_tape = Tape::new();

        let start_pos_alts : Vec<Alteration> = BEN::start_position().to_alterations().collect();
        raw_tape.write_all(&start_pos_alts);

        raw_tape.write_all(&[
            Alteration::Assert(MetadataAssertion::StartTurn(Color::WHITE)),
            Alteration::remove(D2, Occupant::white_pawn()),
            Alteration::place(D4, Occupant::white_pawn()),
            Alteration::Assert(MetadataAssertion::EndTurn)
        ]);

        raw_tape
    }


    fn zobrist_for_startpos() -> Zobrist {
        let start_pos_alts : Vec<Alteration> = BEN::start_position().to_alterations().collect();
        let mut z = Zobrist::empty();
        for alt in start_pos_alts {
            z.alter_mut(alt);
        }
        z
    }

    fn zobrist_for_startpos_and_d4() -> Zobrist {
        let mut z : Zobrist = zobrist_for_startpos();
        let alts = vec![
            Alteration::Assert(MetadataAssertion::StartTurn(Color::WHITE)),
            Alteration::remove(D2, Occupant::white_pawn()),
            Alteration::place(D4, Occupant::white_pawn()),
            Alteration::Assert(MetadataAssertion::EndTurn)
        ];
        for alt in alts {
            z.alter_mut(alt);
        }
        z
    }

    impl Tape<128> {
        pub fn head(&self) -> usize {
            self.head
        }
    }



    #[test]
    #[tracing_test::traced_test]
    fn hash_familiar_works() {
        let mut tape = tape_with_startpos_and_d4();
        let mut familiar : TapeFamiliar<128, Zobrist> = tape.conjure();
        familiar.sync_to_read_head();
        tracing::debug!("\n{:?}", familiar);
        assert_eq!(zobrist_for_startpos_and_d4(), familiar.state);
        familiar.rewind_by(3);
        tracing::debug!("\n{:?}", familiar);
        assert_eq!(zobrist_for_startpos(), familiar.state);

    }


    // This is a bad test.
    // #[test]
    #[tracing_test::traced_test]
    fn head_hash_is_maintained_correctly() {
        let mut tape = tape_with_startpos_and_d4();

        // The position is calculated to start and matches an external calculation.
        assert_eq!(zobrist_for_startpos_and_d4(), tape.head_hash());
        assert_eq!(zobrist_for_startpos_and_d4(), tape.position_hash());
        tracing::debug!("Tape after setup: {:?}", tape);


        let actual_initial_position_hash = tape.position_hash();
        let expected_initial_position_hash = zobrist_for_startpos_and_d4();
        let expected_initial_head_hash = zobrist_for_startpos_and_d4();
        let expected_middle_hash = zobrist_for_startpos_and_d4() ^ Zobrist::from(HazelZobrist::side_to_move_mask());
        let expected_final_position_hash = zobrist_for_startpos();

        // The initial step just takes us off the EOT (which is just the first `None` after all the
        // instructions, we write a bit like a stack, in that we don't allow internal `none`s
        // (yet).
        //
        //
        // The Tape Right Now:
        //
        //       POSITION(1. d4)                POSITION ^ SIDE_TO_MOVE     POSITION(1. d4 ..)
        //                                                                  HEAD_HASH, POSITION_HASH        HEAD
        // ..., START_TURN(WHITE), REMOVE(D2),  PLACE(D4),                  START_TURN(BLACK),              None]
        //
        tape.step_backward();

        tracing::debug!("Tape stepping back 1: {:?}", tape);
        //
        //       POSITION(1. d4)                POSITION ^ SIDE_TO_MOVE     POSITION(1. d4 ..)
        //                                                                  HEAD_HASH, POSITION_HASH, HEAD
        // ..., START_TURN(WHITE), REMOVE(D2),  PLACE(D4),                  START_TURN(BLACK),              None]
        //
        assert_eq!(expected_initial_head_hash, tape.head_hash());

        tape.step_backward();
        //
        //       POSITION(1. d4)                POSITION ^ SIDE_TO_MOVE        POSITION(1. d4 ..)
        //                                      HEAD_HASH, POSITION_HASH, HEAD
        // ..., START_TURN(WHITE), REMOVE(D2),  PLACE(D4),                     START_TURN(BLACK), EOT]
        //
        // We are at the unenviable middle position w/ position_hash for a moment while we unwind
        // back to the next StartTurn.
        assert_eq!(tape.head_hash(), tape.position_hash());
        let step_1_head_hash = tape.head_hash();
        assert_eq!(expected_middle_hash, tape.position_hash());

        tape.step_backward();
        tracing::debug!("Tape stepping back 2: {:?}", tape);
        //
        //       POSITION(1. d4)                         POSITION ^ SIDE_TO_MOVE     POSITION(1. d4 ..)
        //                                  HEAD_HASH    POSITION_HASH
        // ..., START_TURN(WHITE),          REMOVE(D2),  PLACE(D4),                  START_TURN(BLACK), EOT]
        //
        assert_ne!(step_1_head_hash, tape.head_hash());
        let step_2_head_hash = tape.head_hash();
        assert_eq!(expected_middle_hash, tape.position_hash());

        tape.step_backward();
        tracing::debug!("Tape stepping back 3: {:?}", tape);
        // None of these are matching u\ with previous rounds
        //
        //       POSITION(1. d4)                         POSITION ^ SIDE_TO_MOVE     POSITION(1. d4 ..)
        //      HEAD_HASH, POSITION_HASH
        // ..., START_TURN(WHITE),          REMOVE(D2),  PLACE(D4),                  START_TURN(BLACK), EOT]
        //
        assert_ne!(zobrist_for_startpos_and_d4(), tape.head_hash()); // this should compare to previous HEAD
        assert_ne!(zobrist_for_startpos_and_d4(), tape.position_hash());
        let middle_hash = tape.position_hash() ^ Zobrist::from(HazelZobrist::side_to_move_mask());
        assert_ne!(zobrist_for_startpos_and_d4(), middle_hash);

        // The position is back to the startpos, and the position_hash is updated
        assert_eq!(zobrist_for_startpos(), tape.head_hash());
        assert_eq!(zobrist_for_startpos(), tape.position_hash());
    }

    #[test]
    fn write_and_read() {
        let mut tape = Tape::<128>::new();
        let alteration = Alteration::place(D4, Occupant::white_pawn());
        tape.write(alteration);
        tape.step_backward();

        assert_eq!(tape.read(), Some(alteration));
    }

    #[test]
    fn at_bot() {
        let mut tape = Tape::<128>::new();
        assert!(tape.at_bot());

        tape.write(Alteration::Lit(0));
        assert!(!tape.at_bot());
    }

    #[test]
    fn at_eot() {
        let mut tape = Tape::<2>::new();
        assert!(!tape.at_eot());
        tape.step_forward();
        tape.step_forward();
        dbg!(&tape);
        assert!(tape.at_eot());

    }
}

