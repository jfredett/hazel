use std::{collections::HashMap, sync::RwLock};

use crate::{game::position::Position, notation::Square, types::{Bitboard, Color, Occupant, Piece}, Alteration};
use crate::notation::*;


pub struct ZobristTable<const SEED: u64>;

// TODO: Calculate this from the types, so when I add fairy pieces it should Just Work(tm)
// The math is:
//
//  64 squares
// * 6 piece types
// * 2 colors
// + 1 flag for when it's black's turn
const ZOBRIST_TABLE_SIZE : usize = 1 + (2 * 6 * 64);
impl<const SEED: u64> ZobristTable<SEED> {

    const TABLE : [u64; ZOBRIST_TABLE_SIZE] = [0; ZOBRIST_TABLE_SIZE];
    // 0-(ZTS-1) is covered by the #depth_for function
    // =ZTS is the side-to-move marker
    const BLACK_TO_MOVE : u64 = Self::random(ZOBRIST_TABLE_SIZE as u64);
    
    pub const fn random(depth: u64) -> u64 {
        // adapted from https://en.wikipedia.org/wiki/Xorshift
        let mut x = SEED;
        let mut times = 0;
        while times < depth {
            x ^= x << 13;
            x ^= x << 7;
            x ^= x << 17;
            times += 1;
        }
        x
    }

    // TODO: This comes up enough (or the equivalent (sq, occ)) that it probably should be it's own
    // type ("OccupiedSquare"?)
    pub const fn depth_for(sq: Square, color: Color, piece: Piece) -> u64 {
        (sq.index() as u64) * (color as u64) * (piece as u64)
    }


    /// A slow implementation of OccupiedSquare -> 0..ZOBRIST_TABLE_SIZE
    pub const fn slow_zobrist_mask_for(sq: Square, color: Color, piece: Piece) -> u64 {
        Self::random(Self::depth_for(sq, color, piece))
    }

    /// A fast implementation based on the cache
    pub const fn zobrist_mask_for(&self, sq: Square, color: Color, piece: Piece) -> u64 {
        let idx = Self::depth_for(sq, color, piece) as usize;
        Self::TABLE[idx]
    }

    /// A convenience function.
    pub const fn black_to_move_mask(&self) -> u64 {
        Self::BLACK_TO_MOVE
    }

    pub const fn initialize() -> Self {
        // seed the table
        let mut idx = 0;
        while idx < ZOBRIST_TABLE_SIZE {
            // It doesn't matter what these are, they're random, we just need both functions to map
            // down to the idx.
            Self::TABLE[idx] = Self::random(idx as u64);
            idx += 1;
        }

        // Technically this could be anything, because these items exist at compile time.
        ZobristTable
    }
}


// this can be calculated on any `query`able, I think.
#[derive(Eq, Hash, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Zobrist(u64);



const ZOBRIST_TABLE: ZobristTable<123_456_123> = ZobristTable::initialize();

impl Zobrist {
    // plan:
    //
    // 1. Zobrist has a const usize seed that can be set at compile time.
    // 2. Zobrist has it's own simple RNG for the bitstrings, so that zobrists are consistent up to
    //    the seed.
    // 
    // Need to research some rng. should be quick, but it's going to be used at compile time (or at
    // least lazy-static time) I think, so simple is probably the more important thing.

    pub fn new(position: &Position) -> Zobrist {
        let (_, _, alterations) = position.current_boardstate();
        *Zobrist(0).update(&alterations)
    }

    pub fn update(&mut self, alterations: &[Alteration]) -> &mut Self {
        for alter in alterations {
            let delta = match alter {
                Alteration::Place { square: sq, occupant: Occupant::Occupied(piece, color) } => { ZOBRIST_TABLE.zobrist_mask_for(*sq, *color, *piece) } // xor in the occupant
                Alteration::Remove { square: sq, occupant: Occupant::Occupied(piece, color) } => { ZOBRIST_TABLE.zobrist_mask_for(*sq, *color, *piece) } // xor in the occupant
                Alteration::Clear => { (*self) = Zobrist(0); continue; } // set to the zobrist of the empty board.
                _ => { continue } // nothing else matters, we don't care about metadata
            };
            self.0 ^= delta;
        }
        self
    }

}

impl Position {
    pub fn zobrist(&self) -> Zobrist {
        Zobrist(1)
    }
}


pub struct Cache<E> where E : Clone {
    storage: RwLock<HashMap<Zobrist, E>>,
    builder: fn(&Position) -> E
}


impl<E> Cache<E> where E : Clone {
    pub fn get(&self, position: &Position) -> E {
        let key = position.zobrist();

        { // we have to readlock to see if the key is available already
            let storage = self.storage.read().unwrap();
            if storage.contains_key(&key) {
                return storage.get(&key).unwrap().clone()
            }
        } // drop the read lock

        // populate the cache
        tracing::debug!("Building");
        let entry : E = (self.builder)(position);

        tracing::debug!("Setting");
        self.set(key, entry);

        tracing::debug!("Recursing");
        self.get(position)
    }

    pub fn set(&self, zobrist: Zobrist, entry: E) {
        self.storage.write().unwrap().insert(zobrist, entry);
    }

    pub fn new(builder: fn(&Position) -> E) -> Self {
        Cache { storage: RwLock::new(HashMap::new()), builder }
    }
}

pub type ATM<'a, E> = &'a Cache<E>;


#[cfg(test)]
mod tests {
    use crate::notation::ben::BEN;

    use super::*;

    impl<E> Cache<E> where E : Clone {
        fn raw_storage(&self) -> HashMap<Zobrist, E> {
            self.storage.read().unwrap().clone()
        }
    }

    #[test]
    #[tracing_test::traced_test]
    fn cache_test() {
        let cache = Cache::new(|_p| 1u64);

        assert_eq!(cache.raw_storage().values().len(), 0);
        cache.get(&Position::new(BEN::start_position(), vec![]));
        assert_eq!(cache.raw_storage().values().len(), 1);
        cache.get(&Position::new(BEN::start_position(), vec![]));
        assert_eq!(cache.raw_storage().values().len(), 1);
    }

}
