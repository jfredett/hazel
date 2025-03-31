// FIXME: This probably belongs in a -util crate or something like.

use std::{collections::HashMap, sync::RwLock, fmt::Debug};

// where does zobrist live?
use hazel_core::zobrist::*;

pub mod atm;

pub use atm::*;

#[derive(Default, Debug)]
pub struct Cache<E> where E : Clone {
    storage: RwLock<HashMap<Zobrist, E>>,
}


impl<E> Cache<E> where E : Clone + Debug + PartialEq {
    pub fn get(&self, zobrist: Zobrist) -> Option<E> {
        let storage = self.storage.read().unwrap();
        // FIXME: Don't love the clone here, would prefer to return the borrow and let the struct
        // borrow this?
        storage.get(&zobrist).cloned()
    }

    pub fn set(&self, zobrist: Zobrist, entry: E) {
        // TODO: Feature flag this or something, it should be excluded from a 'real' version of the
        // engine, but present for debugging.
        {
            let storage = self.storage.read().unwrap();
            if storage.contains_key(&zobrist) {
                let existing = storage.get(&zobrist).unwrap();
                if *existing != entry {
                }
            }
        }
        let mut storage = self.storage.write().unwrap();
        storage.insert(zobrist, entry);
    }

    pub fn new() -> Self {
        Cache { storage: RwLock::new(HashMap::new()) }
    }

    pub fn atm(&self) -> ATM<E> {
        self
    }
}



/*
*
#[cfg(test)]
mod tests {
    use hazel_representation::{game::position::Position, notation::ben::BEN};

    use super::*;

    impl<E> Cache<E> where E : Clone {
        fn raw_storage(&self) -> HashMap<Zobrist, E> {
            self.storage.read().unwrap().clone()
        }
    }

    #[test]
    fn cache_test() {
        let cache = Cache::new();

        let p = Position::new(BEN::start_position());

        assert_eq!(cache.raw_storage().values().len(), 0);
        cache.get(p.zobrist().position);
        assert_eq!(cache.raw_storage().values().len(), 0);
        cache.set(p.zobrist().position, p);
        assert_eq!(cache.raw_storage().values().len(), 1);
    }
}

*/
