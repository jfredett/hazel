///////////////////////////////////////////////////////////////////////////
///////////////////////////   CACHE   /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////

/*
#[derive(Clone)]
pub struct Cache<T, R> where T: CompilesTo<R> + Clone, R : Clone {
    pub entry: T,
    pub store: Option<R>
}

impl<T, R> From<T> for Cache<T, R> where T: CompilesTo<R> + Clone, R : Clone {
    fn from(entry: T) -> Self {
        Self {
            entry,
            store: None
        }
    }
}

/// NOTE: I'm choosing to say that two equivalent actions are equal even if their compiled
/// representations aren't, because those representations depend on context. The _abstract_
/// equality is what I want here... I think.
impl<T, R> PartialEq for Cache<T, R> where T: CompilesTo<R> + PartialEq + Clone, R : Clone {
    fn eq(&self, other: &Self) -> bool {
        self.entry == other.entry
    }
}

impl<T, R> Debug for Cache<T, R> where T: CompilesTo<R> + Debug + Clone, R : Debug + Clone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cache")
            .field("entry", &self.entry)
            .field("store", &self.store)
            .finish()
    }
}

impl<T, R> Cache<T, R> where T: CompilesTo<R> + Clone, R : Clone {
    /// Get the cached value, or compile it and cache it. This should generally be used when
    /// possible so that the cache fills. Times to avoid it would be hot loops or when you don't
    /// have a convenient mutable reference.
    pub fn get(&mut self, context: &T::Context) -> Option<R> {
        match &self.store {
            Some(cached) => Some(cached.clone()),
            None => {
                let compiled = self.entry.compile(context);
                self.store = Some(compiled.clone());
                Some(compiled)
            }
        }
    }

    #[cfg(test)]
    /// Bypass the cache and get the stored value directly. Test only.
    pub fn get_store(&self) -> Option<R> {
        self.store.clone()
    }
}


    mod cache {
        use super::*;

        #[test]
        fn cache_gets() {
            let mut cache = Cache::from(1);
            let context = ();
            assert_eq!(cache.get_store(), None);
            assert_eq!(cache.get(&context), Some(1));
            assert_eq!(cache.get_store(), Some(1));
        }
    }
*/
