use std::ops::{BitAndAssign, Range};
use rand::{distributions::{Standard, uniform::SampleRange}, prelude::Distribution};
use crate::bitboard::Bitboard;
use super::*;
use tracing::{debug, info, instrument};

// 13b
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spell {
    pub(crate) magic: u64,
    pub(crate) shift: u8,
    pub(crate) shift_mask: u64,
    pub(crate) offset: u32
}

impl Spell {
    pub fn empty() -> Spell {
        Spell {
            magic: 0,
            shift: 0,
            shift_mask: 0,
            offset: 0
        }
    }
    
    pub fn new(shift_range: Range<u8>) -> Spell {
        let mut r = Spell::empty();
        r.initialize(shift_range);
        r
    }
    
    pub fn initialize(&mut self, shift_range: Range<u8>) {
        let mut rng = rand::thread_rng();
        self.magic = Spell::low_bit_random(3);
        // The shift should not allow it to create out-of-bounds indices, and the minimum shift should
        // be `5` (the minimum shift of a bishop).
        // NOTE: Make this range subject to selection?
        self.shift = shift_range.sample_single(&mut rng);
        // Used for constraining the key
        self.shift_mask = 2u64.pow(self.shift.into()) - 1;
        // The maximum offset should be less than the greater piece-table size. It also needs to fit 
        // within the table itself, which is around ~2meg in size.
        // NOTE: Make the modulus, offset value subject to selection?
        self.offset = rand::random::<u32>() & self.shift_mask as u32;
    }
    
    pub fn key_for(&self, blockers: Bitboard) -> usize {
        let mut raw_key : u64 = ((blockers * self.magic) >> (64 - self.shift as u64)).into();
        raw_key += self.offset as u64;
        (raw_key & self.shift_mask) as usize
    }

    #[instrument(skip(self))]
    pub fn mutate(&mut self) {
        self.magic = Spell::low_bit_random::<u64>(3);

        match rand::random::<u8>() % 4 {
            0 => { self.shift -= 1; }
            1 => { self.shift += 1; }
            _ => {} 
        }
        if self.shift < 5 { self.shift = 5; }
        if self.shift > MAX_SHIFT { self.shift = MAX_SHIFT; }
        
        self.shift_mask = 2u64.pow(self.shift.into()) - 1;

        self.offset ^= Spell::low_bit_random::<u32>(2);
        self.offset &= self.shift_mask as u32;
    }

    /// Produce a low-bit-count random number by generating ANDing-bitwise multiple random numbers together
    fn low_bit_random<T>(reps: usize) -> T where 
        T : BitAndAssign,
        Standard : Distribution<T> {
            let mut r = rand::random::<T>();
            for _ in 0..reps {
                r &= rand::random::<T>();
            }
            r
    }
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializing_to_bincode_round_trips() {
        let mut expected = Spell::empty();

        expected.initialize(10..MAX_SHIFT);

        let serialized = bincode::serialize(&expected).unwrap();
        let deserialized : Spell = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized, expected);
    }
    
    #[test]
    fn mutation_changes_values() {
        let original = Spell::new(5..18); 
        let mut mutated = original;
        
        assert_eq!(original, mutated);

        mutated.mutate();

        assert_ne!(original, mutated);
    }
    
    // This test expects a panic if it's broken.
    #[quickcheck]
    fn key_for_does_not_violate_table_boundaries(blockers: Bitboard) {
        let s = Spell::new(5..18);
        s.key_for(blockers);
    }
    
    // This test expects a panic if it's broken.
    #[quickcheck]
    fn key_for_does_not_violate_table_boundaries_even_after_mutation(blockers: Bitboard) {
        let mut s = Spell::new(5..18);
        s.mutate();
        s.key_for(blockers);
    }
}