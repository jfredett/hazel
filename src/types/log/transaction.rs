
// TODO: Change this to have a block-passing API?

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

#[cfg(test)]
mod tests {
    use super::*;

    mod txn {
        use crate::types::log::Log;

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
