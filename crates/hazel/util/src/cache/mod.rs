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
        let mut storage = self.storage.write().unwrap();
        storage.insert(zobrist, entry);
    }

    pub fn size(&self) -> usize {
        self.storage.read().unwrap().values().len()
    }

    pub fn new() -> Self {
        Cache { storage: RwLock::new(HashMap::new()) }
    }

    pub fn atm(&self) -> ATM<E> {
        self
    }
}

