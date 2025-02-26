use std::{collections::HashMap, sync::RwLock, fmt::Debug};

use crate::{game::position::Position, notation::Square, types::{Bitboard, Color, Occupant, Piece}, Alteration};
use crate::notation::*;
use crate::types::zobrist::*;


pub struct Cache<E> where E : Clone {
    storage: RwLock<HashMap<Zobrist, E>>,
    builder: fn(&Position) -> E
}


impl<E> Cache<E> where E : Clone + Debug + PartialEq {
    pub fn get(&self, position: &Position) -> E {
        let key = position.zobrist();

        { // we have to readlock to see if the key is available already
            let storage = self.storage.read().unwrap();
            if storage.contains_key(&key) {
                return storage.get(&key).unwrap().clone()
            }
        } // drop the read lock

        // populate the cache
        let entry : E = (self.builder)(position);

        self.set(key, entry);

        self.get(position)
    }

    pub fn set(&self, zobrist: Zobrist, entry: E) {
        // TODO: Feature flag this or something, it should be excluded from a 'real' version of the
        // engine, but present for debugging.
        {
            let storage = self.storage.read().unwrap();
            if storage.contains_key(&zobrist) {
                tracing::debug!("Potential Collision!");
                let existing = storage.get(&zobrist).unwrap();
                if *existing != entry {
                    tracing::debug!("Collision found! Cached: {:?} != Inserted {:?} with Hash {:?}", &existing, &entry, &zobrist);
                }
            }
        }
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
