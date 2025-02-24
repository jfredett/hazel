use std::{cell::RefCell, collections::HashMap, sync::{Arc, Mutex}};

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
                Alteration::Place { square: sq, occupant: occ } => { 0 } // xor in the occupant
                Alteration::Remove { square: sq, occupant: occ } => { 1 } // xor out the occupant
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

struct Cache<E, B> where E : Clone, B : Fn(&Position) -> E {
    // FIXME:? I do not know for sure that this is going to work with mutliple threads calling
    // `set` simultaneously, potentially. This cache will live on the heap, it should be
    // thread-local, but there is a reasonable chance I need to do something smarter than this
    // for dealing with cache misses (maybe a hot/cold thing, IDK, this datastructure is very
    // prototype rn
    storage: RefCell<HashMap<Zobrist, E>>,
    builder: B // should this be on the cache itself?
}


impl<E, B> Cache<E, B> where E : Clone, B : Fn(&Position) -> E {
    pub fn get(&self, zobrist: Zobrist) -> Option<E> {
        if let Some(res) = self.storage.borrow().get(&zobrist) {
            Some(res.clone())
        } else {
            None
        }
    }

    pub fn set(&self, zobrist: Zobrist, entry: E) {
        self.storage.borrow_mut().insert(zobrist, entry);
    }

    pub fn atm(&self) -> ATM<E, B> where B: Fn(&Position) -> E {
        ATM::new(self)
    }

    pub fn new(builder: B) -> Self {
        Cache { storage: RefCell::new(HashMap::new()) , builder }
    }
}

/// An ATM is responsible for managing lookups/updates to a specific cache from potentially several
/// callers.
///
/// `get` here has two modes:
///
/// 1. It gets the item from cache, we are happy and get back a reference to a mutex to the item.
///    This lock will _always_ have a position behind it, because...
/// 2. On a cache miss, the _ATM_ handles calculating the cache entry, sending it to the cache,
///    then re-retrieving it.
///
/// Since there is only _one_ cache object per entry-type, all the mutability is held in the Cache
/// object
///
/// Eventually this will manage _several_ caches, both thread-local and not thread-local, and be
/// responsible for doing smart lookup between all available caches, opening books, endgame tables, etc
struct ATM<'a, E, B> where E : Clone, B : Fn(&Position) -> E {
    cache: &'a Cache<E, B>,
}



impl<'a, E, B> ATM<'a, E, B> where E : Clone, B : Fn(&Position) -> E {
    pub fn new(cache: &'a Cache<E, B>) -> Self {
        ATM { cache }
    }

    pub fn get(&self, position: &Position) -> E {
        let key = position.zobrist();

        match self.cache.get(key) {
            Some(entry) => {
                entry.clone()
            },
            None => {
                // populate the cache
                let entry : E = (self.cache.builder)(position);

                self.cache.set(key, entry);

                // NOTE: recurse one time, avoids a clone of the above entry, at the cost of a hash lookup. Since
                // entry may be large, it's generally going to be faster to re-lookup, I think.
                self.get(position)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::notation::ben::BEN;

    use super::*;

    impl<E, B> Cache<E, B> where E : Clone, B : Fn(&Position) -> E {
        fn raw_storage(&self) -> HashMap<Zobrist, E> {
            self.storage.borrow().clone()
        }
    }

    #[test]
    fn cache_test() {
        let cache = Cache::new(|p| 1u64);
        let atm = cache.atm();

        assert_eq!(cache.raw_storage().values().len(), 0);
        atm.get(&Position::new(BEN::start_position(), vec![]));
        assert_eq!(cache.raw_storage().values().len(), 1);
        atm.get(&Position::new(BEN::start_position(), vec![]));
        assert_eq!(cache.raw_storage().values().len(), 1);
    }

}
