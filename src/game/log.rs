
use std::{fmt::{self, Debug, Formatter}, rc::Rc};
use super::{action::chess::ChessAction, compiles_to::CompilesTo};

///////////////////////////////////////////////////////////////////////////
/////////////////////////// TRANSACTION ///////////////////////////////////
///////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Transaction<T> {
    content: Vec<T>,
    finished: bool
}

impl<T: Clone> Transaction<T> {
    pub fn begin() -> Self {
        Self {
            content: vec![],
            finished: false
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn record(&mut self, action: T) {
        self.content.push(action);
    }

    pub fn commit(&mut self) -> Vec<T> {
        self.finished = true;
        self.content.clone()
    }

    #[cfg(test)]
    pub fn content(&self) -> Vec<T> {
        self.content.clone()
    }
}

impl<T, R> Default for Log<T, R> where T: CompilesTo<R> + Clone, R : Clone {
    fn default() -> Self {
        Self::start()
    }
}

///////////////////////////////////////////////////////////////////////////
///////////////////////////   CACHE   /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////
///////////////////////////   LOG     /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////

pub struct Log<T, R> where T: CompilesTo<R> + Clone, R : Clone {
    log: Vec<Cache<T, R>>,
    // TODO: Extract this
    current_txn: Transaction<T>,
    stack: Vec<Transaction<T>>,

    write_head: usize
}

impl<T, R> Log<T, R> where T: CompilesTo<R> + Clone, R : Clone {
    pub fn start() -> Self {
        Self {
            log: vec![],
            current_txn: Transaction::begin(),
            stack: vec![],
            write_head: 0
        }
    }

    pub fn seek(&mut self, position: usize) -> Option<&mut Cache<T, R>> {
        if position >= self.log.len() {
            None
        } else {
            self.write_head = position;
            self.read()
        }
    }

    pub fn begin(&mut self) -> &mut Self {
        self.stack.push(self.current_txn.clone());
        self.current_txn = Transaction::begin();
        self
    }

    pub fn record(&mut self, action: T) -> &mut Self {
        self.current_txn.record(action);
        self
    }

    pub fn commit(&mut self) -> &mut Self {
        if self.stack.is_empty() {
            // then we aren't nested.
            let actions = self.current_txn.commit();
            for action in actions.into_iter() {
                let cache = Cache::from(action);
                self.write(cache);
            }

            self.current_txn = Transaction::begin();
        } else {
            // then we are nested
            let actions = self.current_txn.commit();

            self.current_txn = self.stack.pop().unwrap();

            for action in actions.into_iter() {
                self.record(action);
            }
        }
        self
    }

    fn write(&mut self, cache: Cache<T, R>) {
        self.log.insert(self.write_head, cache);
        self.write_head += 1;
    }

    fn read(&mut self) -> Option<&mut Cache<T, R>> {
        self.log.get_mut(self.write_head)
    }

    pub fn get(&mut self, position: usize) -> Option<&mut Cache<T, R>> {
        self.log.get_mut(position)
    }

    pub fn is_empty(&self) -> bool {
        self.log.is_empty()
    }

    pub fn len(&self) -> usize {
        self.log.len()
    }

    /// ```
    /// # use gamerep::game::log::Log;
    ///
    /// let mut log = Log::default();
    /// log.record(1).record(2).commit();
    /// log.record(3).record(4).commit();
    ///
    /// log.cursor(|cursor| {
    ///     let current = cursor.read()
    /// }
    /// ```
    ///
    ///

    pub fn cursor(&mut self, block: impl Fn(&mut Cursor<T, R>)) {
        let mut cursor = Cursor::new(self);
        block(&mut cursor);
    }

    #[cfg(test)]
    fn log(&self) -> Vec<T> {
        let copy = self.log.clone();

        copy.into_iter().map(|cache| cache.entry).collect::<Vec<T>>()
    }

    #[cfg(test)]
    fn txn_state(&self) -> Vec<T> {
        self.current_txn.content.clone()
    }
}

impl<T, R> IntoIterator for Log<T, R> where T: CompilesTo<R> + Clone, R : Clone {
    type Item = Cache<T, R>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.log.into_iter()
    }
}

///////////////////////////////////////////////////////////////////////////
///////////////////////////   CURSOR  /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////

pub struct Cursor<'a, T, R> where T: CompilesTo<R> + Clone, R : Clone {
    log: &'a mut Log<T, R>,
    position: usize
}

impl<'a, T, R> Cursor<'a, T, R>  where T: CompilesTo<R> + Clone, R : Clone {
    pub fn new(log: &'a mut Log<T, R>) -> Self {
        Self {
            log,
            position: 0
        }
    }

    pub fn seek(&mut self, position: usize) -> Option<&mut Cache<T, R>> {
        self.position = position;
        self.read()
    }

    // FIXME: Should this be conditional?
    pub fn jump(&mut self, offset: isize) -> Option<&mut Cache<T, R>> {
        let new_position = self.position as isize + offset;

        if new_position < 0 {
            self.position = 0;
            None
        } else {
            if new_position as usize >= self.log.log.len() {
                self.position = self.log.len();
            } else {
                self.position = new_position as usize;
            }
            self.read()
        }
    }

    /// Now listen for a second, I know this looks bad.
    ///
    /// I'm not implementing `Iterator` because of weird lifetime stuff that happens that I'm too
    /// scared to try to fix.
    ///
    /// It looks bad because it is bad. Don't be like me, be brave.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut Cache<T, R>> {
        if self.position == self.log.len() {
            None
        } else {
            self.position += 1;
            self.read()
        }
    }

    pub fn prev(&mut self) -> Option<&mut Cache<T, R>> {
        if self.position == 0 {
            None
        } else {
            self.position -= 1;
            self.read()
        }
    }

    pub fn read(&mut self) -> Option<&mut Cache<T, R>> {
        self.log.get(self.position)
    }
}

///////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    /// A trivial implementation to test with.
    impl CompilesTo<i32> for i32 {
        fn compile(&self, _: &()) -> i32 {
            *self
        }
    }

    mod cursor {
        use super::*;

        #[test]
        fn cursor_seeks() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.cursor(|cursor| {
                assert_eq!(cursor.seek(0).unwrap().get(&()), Some(1));
                assert_eq!(cursor.seek(1).unwrap().get(&()), Some(2));
                assert_eq!(cursor.seek(2), None);
                assert_eq!(cursor.seek(3), None);
            });

            log.record(3).record(4).commit();

            log.cursor(|cursor| {
                assert_eq!(cursor.seek(0).unwrap().get(&()), Some(1));
                assert_eq!(cursor.seek(1).unwrap().get(&()), Some(2));
                assert_eq!(cursor.seek(3).unwrap().get(&()), Some(4));
                assert_eq!(cursor.seek(2).unwrap().get(&()), Some(3));
                assert_eq!(cursor.seek(4), None);
            });
        }

        #[test]
        fn cursor_prev_and_next() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.cursor(|cursor| {
                assert_eq!(cursor.read().unwrap().get(&()), Some(1));
                assert_eq!(cursor.next().unwrap().get(&()), Some(2));
                assert_eq!(cursor.next(), None);
                assert_eq!(cursor.prev().unwrap().get(&()), Some(2));
                assert_eq!(cursor.prev().unwrap().get(&()), Some(1));
                assert_eq!(cursor.prev(), None);
            });

            log.record(3).record(4).commit();

            log.cursor(|cursor| {
                assert_eq!(cursor.read().unwrap().get(&()), Some(1));
                assert_eq!(cursor.next().unwrap().get(&()), Some(2));
                assert_eq!(cursor.next().unwrap().get(&()), Some(3));
                assert_eq!(cursor.next().unwrap().get(&()), Some(4));
                assert_eq!(cursor.next(), None);
                assert_eq!(cursor.prev().unwrap().get(&()), Some(4));
                assert_eq!(cursor.prev().unwrap().get(&()), Some(3));
                assert_eq!(cursor.prev().unwrap().get(&()), Some(2));
                assert_eq!(cursor.prev().unwrap().get(&()), Some(1));
                assert_eq!(cursor.prev(), None);
            });
        }


        #[test]
        fn cursor_jumps() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.cursor(|cursor| {
                assert_eq!(cursor.seek(0).unwrap().get(&()), Some(1));
                assert_eq!(cursor.jump(1).unwrap().get(&()), Some(2));
                assert_eq!(cursor.jump(1), None);
                assert_eq!(cursor.jump(-1).unwrap().get(&()), Some(2));
                assert_eq!(cursor.jump(-1).unwrap().get(&()), Some(1));
                assert_eq!(cursor.jump(-1), None);
            });

            log.record(3).record(4).commit();

            log.cursor(|cursor| {
                assert_eq!(cursor.seek(0).unwrap().get(&()), Some(1));
                assert_eq!(cursor.jump(1).unwrap().get(&()), Some(2));
                assert_eq!(cursor.jump(1).unwrap().get(&()), Some(3));
                assert_eq!(cursor.jump(1).unwrap().get(&()), Some(4));
                assert_eq!(cursor.jump(1), None);
                assert_eq!(cursor.jump(-1).unwrap().get(&()), Some(4));
                assert_eq!(cursor.jump(-1).unwrap().get(&()), Some(3));
                assert_eq!(cursor.jump(-1).unwrap().get(&()), Some(2));
                assert_eq!(cursor.jump(-1).unwrap().get(&()), Some(1));
                assert_eq!(cursor.jump(-1), None);
            });
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

    mod txn {
        use super::*;

        #[test]
        fn commit_records_changes() {
            let mut log = Log::default();
            log.record(1).record(2);
            assert_eq!(log.log(), vec![]);
            log.commit();
            assert_eq!(log.log(), vec![1, 2]);
        }

        #[test]
        fn multiple_txns() {
            let mut log = Log::default();
            log.record(1).record(2);
            log.commit();
            log.record(3).record(4);
            log.commit();
            assert_eq!(log.log(), vec![1, 2, 3, 4]);
        }

        #[test]
        fn nested_txns() {
            let mut log = Log::default();

            assert_eq!(log.log(), vec![]);

            log.record(1).record(2);
            log.commit();

            assert_eq!(log.log(), vec![1, 2]);

            log.record(3);

            assert_eq!(log.txn_state(), vec![3]);

            log.begin()
                .record(4);

            assert_eq!(log.txn_state(), vec![4]);

            assert_eq!(log.log(), vec![1, 2]);

            log.commit();

            assert_eq!(log.txn_state(), vec![3, 4]);

            assert_eq!(log.log(), vec![1, 2]);

            log.commit();

            assert_eq!(log.log(), vec![1, 2, 3, 4]);
        }
    }
}
