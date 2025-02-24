use std::fmt::Debug;
use transaction::Transaction;
use write_head::WriteHead;
use cursor::Cursor;

pub mod cursor;
pub mod transaction;
pub mod write_head;

#[derive(Clone)]
pub struct Log<T> where T: Clone {
    log: Vec<T>,
    // TODO: Extract this
    current_txn: Transaction<T>,
    stack: Vec<Transaction<T>>,

    write_head: usize
}

impl<T> PartialEq for Log<T> where T : Clone + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.log == other.log
    }
}

impl<T> Debug for Log<T> where T: Debug + Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Log: {:?}", self.log)
    }
}

impl<T> Default for Log<T> where T: Clone {
    fn default() -> Self {
        Self::start()
    }
}

impl<T> Log<T> where T: Clone {
    pub fn start() -> Self {
        Self {
            log: vec![],
            current_txn: Transaction::begin(),
            stack: vec![],
            write_head: 0
        }
    }

    pub fn seek(&mut self, position: usize) {
        if position < self.log.len() {
            self.write_head = position;
        } else {
            // FIXME: I think this should actually not constrain the write head to the length of
            // the current buffer, but rather store it as a sparse log or some kind of tree. This
            // would allow writing to arbitrary positions in an 'infinite' log, and then jumping
            // around to those positions.
            panic!("Attempted to seek past the end of the log.");
        }
    }

    pub fn begin(&mut self) -> &mut Self {
        self.stack.push(self.current_txn.clone());
        self.current_txn = Transaction::begin();
        self
    }

    pub fn read(&self) -> Option<&T> {
        self.log.get(self.write_head)
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
                self.write(action);
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

    pub fn commit_all(&mut self) -> &mut Self {
        while !self.stack.is_empty() {
            self.commit();
        }
        self
    }

    fn write(&mut self, cache: T) {
        self.log.insert(self.write_head, cache);
        self.write_head += 1;
    }

    pub fn get(&self, position: usize) -> Option<&T> {
        self.log.get(position)
    }

    pub fn get_mut(&mut self, position: usize) -> Option<&mut T> {
        self.log.get_mut(position)
    }

    pub fn is_empty(&self) -> bool {
        self.log.is_empty()
    }

    pub fn len(&self) -> usize {
        self.log.len()
    }

    /// Cursor offers a readonly view of the current state of the log. The cursor object lives for
    /// as long as the provided block.
    pub fn cursor<A>(&self, block: impl Fn(&mut Cursor<T>) -> A) -> A {
        let mut cursor = self.raw_cursor();
        block(&mut cursor)
    }

    pub fn write_head(&mut self, block: impl Fn(&mut WriteHead<T>)) {
        let mut write_head = WriteHead::new(self);
        block(&mut write_head);
    }

    pub(crate) fn raw_cursor(&self) -> Cursor<T> {
        Cursor::new(self)
    }

    /*
    pub(crate) fn raw_writehead(&mut self) -> WriteHead<T> {
        WriteHead::new(self)
    }
    */
}

impl<T> IntoIterator for Log<T> where T: Clone {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.log.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<T> Log<T> where T : Clone {
        pub fn log(&self) -> Vec<T> {
            self.log.clone()
        }

        pub fn txn_state(&self) -> Vec<T> {
            self.current_txn.content().clone()
        }
    }


    #[test]
    fn seek_to_end() {
        let mut log = Log::start();
        log.record(1).record(2).record(3).commit();

        log.seek(2);
        assert_eq!(log.read(), Some(&3));
    }

    #[test]
    #[should_panic]
    fn seek_past_end() {
        let mut log = Log::start();
        log.record(1).record(2).record(3).commit();

        log.seek(4);
    }

    #[test]
    fn is_empty() {
        let mut log = Log::<i32>::start();
        assert!(log.is_empty());
        log.record(1).commit();
        assert!(!log.is_empty());
    }
}
