use std::{collections::HashMap, sync::RwLock};

use crate::{game::position::Position, Alteration};


// this can be calculated on any `query`able, I think.
#[derive(Eq, Hash, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Zobrist(u64);

impl Zobrist {
    const TABLE_SIZE : usize = 2 * 6 * 64;
    const TABLE : [u64; Self::TABLE_SIZE] = [0; Self::TABLE_SIZE];
    const EMPTY_ZOBRIST: Self = Zobrist(0); // TODO This is almost certainly wrong.

    pub fn update(&mut self, alterations: &[Alteration]) -> &mut Self {
        for alter in alterations {
            let delta = match alter {
                Alteration::Place { square: _sq, occupant: _occ } => { 0 } // xor in the occupant
                Alteration::Remove { square: _sq, occupant: _occ } => { 1 } // xor out the occupant
                Alteration::Clear => { (*self) = Self::EMPTY_ZOBRIST; continue; } // set to the zobrist of the empty board.
                _ => { continue } // nothing else matters, we don't care about metadata
            };
            (*self).0 ^= delta;
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
