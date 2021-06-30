use std::ops::Range;
use rand::distributions::uniform::SampleRange;
use crate::bitboard::Bitboard;
use super::consts::*;

// 13b
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Spell {
    magic: u64,
    shift: u8,
    offset: u32
}

impl Spell {
    pub fn empty() -> Spell {
        Spell {
            magic: 0,
            shift: 0,
            offset: 0
        }
    }
    
    pub fn initialize(&mut self, shift_range: Range<u8>) {
        let mut rng = rand::thread_rng();
        self.magic = Spell::low_bit_random_u64();
        // The maximum offset should be less than the greater piece-table size. It also needs to fit 
        // within the table itself, which is around ~2meg in size.
        // NOTE: Make the modulus, offset value subject to selection?
        self.offset = (rand::random::<u32>() % TABLE_SIZE as u32) - ROOK_TABLE_SIZE as u32;
        // The shift should not allow it to create out-of-bounds indices, and the minimum shift should
        // be `5` (the minimum shift of a bishop). It's max is 21 so it cannot select more bits than 
        // we can index within TABLE_SIZE (which is ~2.125MiB)
        // NOTE: Make this range subject to selection?
        self.shift = shift_range.sample_single(&mut rng);
    }
    
    pub fn key_for(&self, blockers: Bitboard) -> usize {
        let raw_key : u64 = ((blockers * self.magic) >> (self.shift as u64)).into();
        (raw_key + self.offset as u64) as usize
    }

    fn low_bit_random_u64() -> u64 {
        rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
    }
}