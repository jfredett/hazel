use std::ops::BitAndAssign;
use rand::{distributions::Standard, prelude::Distribution};
use super::*;
use tracing::instrument;

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
    
    // NOTE: This is really just for tests. It might be worth refactoring away later.
    #[allow(dead_code)]
    pub fn new(shift: u8) -> Spell {
        let mut r = Spell::empty();
        r.initialize(shift);
        r
    }
    
    pub fn initialize(&mut self, shift: u8) {
        self.magic = Spell::low_bit_random(3);
        self.shift = shift;
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

        self.shift_mask = 2u64.pow(self.shift.into()) - 1;

        self.offset ^= Spell::low_bit_random::<u32>(2);
        self.offset = (self.offset % TABLE_SIZE as u32) - 8192;
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

        expected.initialize(10);

        let serialized = bincode::serialize(&expected).unwrap();
        let deserialized : Spell = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized, expected);
    }
    
    #[test]
    fn mutation_changes_values() {
        let original = Spell::new(5); 
        let mut mutated = original;
        
        assert_eq!(original, mutated);

        mutated.mutate();

        assert_ne!(original, mutated);
    }
    
    // This test expects a panic if it's broken.
    #[quickcheck]
    fn key_for_does_not_violate_table_boundaries(blockers: Bitboard) {
        let s = Spell::new(5);
        s.key_for(blockers);
    }
    
    // This test expects a panic if it's broken.
    #[quickcheck]
    fn key_for_does_not_violate_table_boundaries_even_after_mutation(blockers: Bitboard) {
        let mut s = Spell::new(5);
        s.mutate();
        s.key_for(blockers);
    }
}