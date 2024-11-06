use transaction::Transaction;
use write_head::Cursor;

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

    fn write(&mut self, cache: T) {
        self.log.insert(self.write_head, cache);
        self.write_head += 1;
    }

    pub fn get(&mut self, position: usize) -> Option<&mut T> {
        self.log.get_mut(position)
    }

    pub fn is_empty(&self) -> bool {
        self.log.is_empty()
    }

    pub fn len(&self) -> usize {
        self.log.len()
    }

    /// `` TODO: Get this working.
    /// # use hazel::game::log::Log;
    /// # use hazel::game::compiles_to::CompilesTo;
    ///
    /// let mut log = Log::default();
    /// log.record(1).record(2).commit();
    /// log.record(3).record(4).commit();
    ///
    /// log.cursor(|cursor| {
    ///     let current = cursor
    ///         .read()     // read the value under the cursor
    ///         .unwrap()   // unwrap it as it may not exist if the cursor is past the end of the log
    ///         .get(&());  // the value is in a cache that expects a compile context, in this
    ///                     //   trivial example we don't need one, so we pass unit.
    ///     assert_eq!(current, Some(1));
    /// });
    /// // The cursor is single-use.
    /// ``
    pub fn cursor(&mut self, block: impl Fn(&mut Cursor<T>)) {
        let mut cursor = Cursor::new(self);
        block(&mut cursor);
    }

    #[cfg(test)]
    fn log(&self) -> Vec<T> {
        self.log.clone()
    }

    #[cfg(test)]
    fn txn_state(&self) -> Vec<T> {
        self.current_txn.content().clone()
    }
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

}
