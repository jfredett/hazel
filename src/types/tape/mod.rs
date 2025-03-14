use std::cmp;
use std::fmt::Debug;
use std::ops::Deref;
use std::range::Range;
use std::sync::{Arc, RwLock};

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


// TODO: probably instead of this, the RwLock should be hidden by the Cursor
impl Tapelike for RwLock<Tape> {
    type Item = Alteration;

    fn length(&self) -> usize {
        self.read().unwrap().hwm
    }

    fn read_address(&self, address: usize) -> Self::Item {
        let tape = self.read().unwrap();
        tape.read_address(address)
    }

    // FIXME: I dislike this, I wish I was sending back something that didn't require an
    // allocation, but I think ultimately this is probably the best way to do it for now.
    //
    // At some point, I think Tapes will probably be further abstracted to be 'infinite' in a way
    // they aren't, presently. At that point there will be some other entity holding the reference
    // to a specific section of tape, and I think _that thing_ will hand out, e.g., Arcs to those
    // things.
    //
    // Essentially a big Tape Allocator (the "TapeDeck" if you will). Which handles caching and
    // sharing chunks of tape, and 2-phase updating them, etc. Tapes can then be held immutably
    // there, and when updates are made they are CoW, and so references will always be to some
    // static memory that can be shared freely.
    fn read_range(&self, range: impl Into<Range<usize>>) -> dynamic_array::SmallArray<Self::Item> {
        let r = range.into();
        tracing::trace!(target="hazel::tapelike", "reading range {:?}", &r);
        let tape = self.read().unwrap();
        tape.read_range(r)
    }

    fn write_address(&mut self, _address: usize, _data: &Self::Item) {
        todo!()
    }

    fn write_range(&mut self, _start: usize, _data: &[Self::Item]) {
        todo!()
    }
}

impl Tapelike for Tape {
    type Item = Alteration;

    fn length(&self) -> usize {
        self.hwm
    }

    fn read_address(&self, address: usize) -> Self::Item {
        if address > self.hwm {
            Alteration::Noop
        } else {
            self.data[address]
        }
    }

    fn read_range(&self, range: impl Into<Range<usize>>) -> dynamic_array::SmallArray<Self::Item> {
        let r : Range<usize> = range.into();

        // (start..end), hwm, length
        //
        // Correct values:
        //
        // (min(start,hwm)..min(end,hwm))
        let corrected_range_start = cmp::min(r.start, self.hwm);
        let corrected_range_end = cmp::min(r.end, self.hwm) + 1;
        let corrected_range = corrected_range_start..corrected_range_end;

        tracing::debug!("Given Range: {:?}, Corrected Range: {:?}, HWM: {:#04X}", r, corrected_range, self.hwm);
        // NOTE: This is closer to what I'd prefer, but it doesn't work through the RwLock impl.
        // OwningRef::new(self).map(|e| e.data.get(corrected_range).unwrap())

        // We have to do an additional check since we return a small-array, which is indexed by a
        // u8.
        let u8max : usize = u8::MAX.into();
        let data = if corrected_range.len() > u8max {
            self.data.get(corrected_range_start..(corrected_range_start + u8max)).unwrap()
        } else {
            self.data.get(corrected_range).unwrap()
        };

        // HACK: Gross.
        dynamic_array::SmallArray::from_vec(data.to_vec())
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
        let iterator = self.data.as_slice().iter();
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

    // TODO: This should be moved up to Position, it gets handed the Arc<Tape> from there.
    pub fn position_hash(&self) -> Zobrist {
        // this is not the right way to do this, it should definitely live up on position
        let mut familiar : Familiar<Self, PositionZobrist> = familiar::conjure(Arc::new(self.clone()));
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

    // This should maybe live here, but the familiar is over the dynamic array type instead of the
    // tape, and Tape should Arc<RwLock> it's underlying data. We'll hand that Arc<RwLock> to the
    // head-hash familiar and manage it that way.
    //
    // Alternately externalize all this stuff and manage caching tapes from some external system.
    pub fn head_hash(&self) -> Zobrist {
        // TODO: Cache this and advance on demand
        // TOO: refactor all this 
        let mut familiar : Familiar<Tape, Zobrist> = familiar::conjure(Arc::new(self.clone()));
        // this will change to look at the current write position (head) and moving towards it
        // updating the hash on the way.
        familiar.seek(self.head);
        *familiar.get()
    }

    pub fn read(&self) -> Alteration {
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

impl Deref for Tape {
    type Target = [Alteration];

    fn deref(&self) -> &[Alteration] {
        self.data.as_slice()
    }
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
        let mut familiar : Familiar<Tape, Zobrist> = familiar::conjure(Arc::new(tape.clone()));
        familiar.seek(tape.write_head());
        assert_eq!(zobrist_for_startpos_and_d4(), *familiar.get());
        familiar.rewind_until(|a| matches!(a.read_address(a.position()), Alteration::Turn));
        assert_eq!(zobrist_for_startpos(), *familiar.get());
    }

    #[test]
    fn write_and_read() {
        let mut tape = Tape::default();
        let alteration = Alteration::place(D4, Occupant::white_pawn());
        tape.write(alteration);
        tape.step_backward();

        assert_eq!(tape.read(), alteration);
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

        assert_eq!(tape.read_address(0), alteration);
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
        assert_eq!(*tape.read_range(range), alterations);
    }

    #[test]
    fn tape_can_write_addresses() {
        let mut tape = Tape::default();
        let alteration = Alteration::place(D4, Occupant::white_pawn());
        tape.write_address(0, &alteration);

        assert_eq!(tape.read_address(0), alteration);
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
        assert_eq!(*tape.read_range(range), alterations);
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
