use std::fmt::Debug;
use std::range::Range;

use crate::{Alter, Alteration};
use crate::types::zobrist::Zobrist;

use cursorlike::Cursorlike;
use familiar::state::position_zobrist::PositionZobrist;
use familiar::Familiar;
use tapelike::Tapelike;

pub mod cursor;
pub mod cursorlike;
pub mod familiar;
pub mod tape_direction;
pub mod tapelike;

#[derive(Clone)]
pub struct Tape {
    data: dynamic_array::MediumArray<Alteration>,
    // the current end of tape ("high water") mark.
    hwm: usize,
    // this is the write head, I might need a familiar for the proceed/unwind stuff?
    head: usize
}

// TODO: Configuration-by-file-or-engine-option.
pub const DEFAULT_TAPE_SIZE : u16 = 1024;

impl Default for Tape {
    fn default() -> Self {
        Self::new(DEFAULT_TAPE_SIZE)
    }
}

impl Tapelike for Tape {
    type Item = Alteration;

    fn length(&self) -> usize {
        self.hwm
    }

    fn read_address(&self, address: usize) -> &Self::Item {
        if address > self.hwm { return &Alteration::Noop; }
        &self.data[address]
    }

    fn read_range(&self, range: impl Into<Range<usize>>) -> &[Self::Item] {
        let r : Range<usize> = range.into();
        if r.end <= self.hwm {
            self.data.get(r).unwrap() // we know it is all populated because it's below the hwm.
        } else if r.start < self.hwm && r.end > self.hwm {
            // we know it starts on tape, but falls off, no worries, we just grab everything up to
            // the HWM
            self.data.get(r.start..self.hwm).unwrap() // we know it is all populated because it's below the hwm.
        } else {
            &[]
        }
    }

    fn write_address(&mut self, address: usize, data: &Self::Item) {
        if address > self.hwm {
            self.hwm = address;
        }
        self.data[address] = *data;
    }

    fn write_range(&mut self, start: usize, data: &[Self::Item]) {
        // TODO: In principle this should be a single copy to a subslice, calculate the range as
        // start .. start + len, then write in a big chunk. Actually getting hte typesystem to be
        // okay with that is nontrivial.
        for idx in 0..data.len() {
            self.write_address(idx + start, &data[idx]);
        }
    }
}

impl Debug for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nTAPE(head_hash: {:?}, position_hash: {:?}, head: {:#04x}, hwm: {:#04x})",
            self.head_hash(), self.position_hash(), self.head, self.hwm
        )?;
        let mut running_hash = Zobrist::empty();
        let iterator = self.data.as_slice().into_iter();
        for (idx, alter) in iterator.enumerate() {
            if idx >= self.hwm {
                writeln!(f, "END-OF-TAPE")?;
                break;
            }

            running_hash.alter_mut(*alter);
            if idx == self.head {
                writeln!(f, "HEAD*    | {:>64} | {:>64?}", alter, running_hash)?;
            } else {
                writeln!(f, "{:>#008X} | {:>64} | {:>64?}", idx, alter, running_hash)?;
            }
        }
        writeln!(f, "=================================")
    }

}

impl Tape {
    pub fn new(cap: u16) -> Self {
        Tape { data: dynamic_array::MediumArray::zeroed(cap), head: 0, hwm: 0 }
    }

    // TODO: OQ: Same as head_hash
    pub fn position_hash(&self) -> Zobrist {
        let mut familiar : Familiar<'_, Self, PositionZobrist> = self.conjure();
        familiar.seek(self.head);
        familiar.get().position
    }

    /// the point to which the tape is valid
    pub fn read_head(&self) -> usize {
        if self.write_head() == 0 {
            0
        } else {
            self.write_head() - 1
        }
    }

    /// the next empty slot to write to
    pub fn write_head(&self) -> usize {
        self.head
    }

    // TODO: OQ: possibly these live _on position_?
    pub fn head_hash(&self) -> Zobrist {
        // TODO: Cache this and advance on demand
        let mut familiar : Familiar<'_, Tape, Zobrist> = self.conjure();
        // this will change to look at the current write position (head) and moving towards it
        // updating the hash on the way.
        familiar.seek(self.head);
        *familiar.get()
    }

    pub fn conjure<A : Default>(&self) -> Familiar<Tape,A> {
        familiar::conjure(self)
    }

    pub fn read(&self) -> &Alteration {
        self.read_address(self.head)
    }

    pub fn write_all(&mut self, alterations: &[Alteration]) {
        for alter in alterations {
            self.write(*alter);
        }
    }

    // ## THIS SECTION NEEDS TO MAINTAIN ALL THE HASHES INCREMENTALLY ## //

    pub fn write(&mut self, alter: Alteration) {
        // if we're at the End-of-buffer, cache out
        if self.buffer_full() {
            // TODO: Actually cache out, this just blanks the buffer and recurses.
            tracing::error!("CACHE OUT CACHE OUT CACHE OUT");
            let len = self.data.len();
            self.data = dynamic_array::MediumArray::zeroed(len);
            self.head = 0;
            self.write(alter);
        }

        // write the instruction to the tape
        self.data[self.head] = alter;
        self.head += 1;
        if self.head > self.hwm {
            self.hwm = self.head;
        }
    }

    pub fn at_bot(&self) -> bool {
        self.head == 0
    }

    pub fn at_eot(&self) -> bool {
        self.head == self.hwm
    }

    pub fn buffer_full(&self) -> bool {
        // watch out for off-by-ones
        self.head + 1 == self.length()
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ProceedToken {
    Continue,
    Halt
}

#[cfg(test)]
mod tests {
    use cursorlike::Cursorlike;

    use crate::alteration::MetadataAssertion;
    use crate::types::zobrist::HazelZobrist;
    use crate::types::Color;
    use crate::{notation::ben::BEN, types::Occupant};
    use crate::notation::*;
    use crate::constants::File;
    use crate::coup::rep::MoveType;
    use crate::game::castle_rights::CastleRights;

    use super::*;

    fn tape_with_startpos_and_d4() -> Tape {
        let mut raw_tape = Tape::default();

        let start_pos_alts : Vec<Alteration> = BEN::start_position().to_alterations().collect();
        raw_tape.write_all(&start_pos_alts);

        raw_tape.write_all(&[
            Alteration::Turn,
                Alteration::Assert(MetadataAssertion::SideToMove(Color::WHITE)),
                Alteration::Assert(MetadataAssertion::CastleRights(CastleRights::default())),
                Alteration::Assert(MetadataAssertion::FiftyMoveCount(0u8)),
                Alteration::Assert(MetadataAssertion::FullMoveCount(1u16)),

                Alteration::remove(D2, Occupant::white_pawn()),
                Alteration::place(D4, Occupant::white_pawn()),

                Alteration::Inform(MetadataAssertion::EnPassant(File::D)),
                Alteration::Inform(MetadataAssertion::FiftyMoveCount(0u8)),
                Alteration::Inform(MetadataAssertion::MoveType(MoveType::DOUBLE_PAWN)),
                Alteration::Inform(MetadataAssertion::SideToMove(Color::BLACK)),
            Alteration::End,
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
            Alteration::Turn,
                Alteration::Assert(MetadataAssertion::SideToMove(Color::WHITE)),
                Alteration::Assert(MetadataAssertion::CastleRights(CastleRights::default())),
                Alteration::Assert(MetadataAssertion::FiftyMoveCount(0u8)),
                Alteration::Assert(MetadataAssertion::FullMoveCount(1u16)),

                Alteration::remove(D2, Occupant::white_pawn()),
                Alteration::place(D4, Occupant::white_pawn()),

                Alteration::Inform(MetadataAssertion::EnPassant(File::D)),
                Alteration::Inform(MetadataAssertion::FiftyMoveCount(0u8)),
                Alteration::Inform(MetadataAssertion::MoveType(MoveType::DOUBLE_PAWN)),
                Alteration::Inform(MetadataAssertion::SideToMove(Color::BLACK)),
            Alteration::End,
        ];
        for alt in alts {
            z.alter_mut(alt);
        }
        z
    }

    impl Tape {
        pub fn head(&self) -> usize {
            self.head
        }
    }



    #[test]
    #[tracing_test::traced_test]
    fn hash_familiar_works() {
        let tape = tape_with_startpos_and_d4();
        let mut familiar : Familiar<Tape, Zobrist> = tape.conjure();
        familiar.seek(tape.write_head());
        assert_eq!(zobrist_for_startpos_and_d4(), *familiar.get());
        familiar.rewind_until(|a| matches!(a.read(), Alteration::Turn));
        assert_eq!(zobrist_for_startpos(), *familiar.get());
    }

    #[test]
    fn write_and_read() {
        let mut tape = Tape::default();
        let alteration = Alteration::place(D4, Occupant::white_pawn());
        tape.write(alteration);
        tape.step_backward();

        assert_eq!(tape.read(), &alteration);
    }

    #[test]
    fn at_bot() {
        let mut tape = Tape::default();
        assert!(tape.at_bot());

        tape.write(Alteration::Lit(0));
        assert!(!tape.at_bot());
    }

    #[test]
    fn at_eot() {
        let mut tape = Tape::default();
        assert!(tape.at_eot());
        tape.write_all(&[
            Alteration::Noop,
            Alteration::Noop,
            Alteration::Noop,
            Alteration::Noop,
        ]);
        assert!(tape.at_eot());
        tape.step_backward();
        assert!(!tape.at_eot());
        tape.step_forward();
        assert!(tape.at_eot());
    }

    #[test]
    fn tape_can_read_addresses() {
        let mut tape = Tape::default();
        let alteration = Alteration::place(D4, Occupant::white_pawn());
        tape.write(alteration);

        assert_eq!(tape.read_address(0), &alteration);
    }


    #[test]
    fn tape_can_read_address_ranges() {
        let mut tape = Tape::default();
        let alterations = vec![
            Alteration::place(D4, Occupant::white_pawn()),
            Alteration::place(E5, Occupant::black_king())
        ];
        tape.write_all(&alterations);

        let range = 0..2;
        assert_eq!(tape.read_range(range), &alterations);
    }

    #[test]
    fn tape_can_write_addresses() {
        let mut tape = Tape::default();
        let alteration = Alteration::place(D4, Occupant::white_pawn());
        tape.write_address(0, &alteration);

        assert_eq!(tape.read_address(0), &alteration);
    }

    #[test]
    fn tape_can_write_ranges() {
        let mut tape = Tape::default();
        let alterations = vec![
            Alteration::place(D4, Occupant::white_pawn()),
            Alteration::place(E5, Occupant::black_king())
        ];
        tape.write_range(0, &alterations);

        let range = 0..2;
        assert_eq!(tape.read_range(range), &alterations);
    }

    #[test]
    fn tape_can_get_length() {
        let mut tape = Tape::default();
        let alterations = vec![
            Alteration::place(D4, Occupant::white_pawn()),
            Alteration::place(E5, Occupant::black_king())
        ];
        tape.write_all(&alterations);

        assert_eq!(tape.length(), alterations.len());
    }
}
