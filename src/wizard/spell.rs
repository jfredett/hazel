use std::ops::Range;
use rand::distributions::uniform::SampleRange;
use crate::bitboard::Bitboard;
use super::*;
use tracing::debug;

// 13b
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spell {
    magic: u64,
    shift: u8,
    shift_mask: u64,
    offset: u32
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
    
    pub fn initialize(&mut self, shift_range: Range<u8>) {
        let mut rng = rand::thread_rng();
        self.magic = Spell::low_bit_random_u64();
        // The shift should not allow it to create out-of-bounds indices, and the minimum shift should
        // be `5` (the minimum shift of a bishop).
        // NOTE: Make this range subject to selection?
        self.shift = shift_range.sample_single(&mut rng);
        // Used for constraining the key
        self.shift_mask = 2u64.pow(self.shift.into()) - 1;
        // The maximum offset should be less than the greater piece-table size. It also needs to fit 
        // within the table itself, which is around ~2meg in size.
        // NOTE: Make the modulus, offset value subject to selection?
        self.offset = rand::random::<u32>();
    }
    
    pub fn key_for(&self, blockers: Bitboard) -> usize {
        let mut raw_key : u64 = ((blockers * self.magic) >> (64 - self.shift as u64)).into();
        raw_key += self.offset as u64;
        let ret = (raw_key & self.shift_mask) as usize;
        if ret > TABLE_SIZE {
            debug!(ret);
        }
        ret
    }

    fn low_bit_random_u64() -> u64 {
        rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
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
}