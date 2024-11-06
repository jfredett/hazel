use super::Log;

pub struct Cursor<'a, T> where T: Clone {
    log: &'a mut Log<T>,
    position: usize
}

impl<'a, T> Cursor<'a, T>  where T: Clone {
    pub fn new(log: &'a mut Log<T>) -> Self {
        Self {
            log,
            position: 0
        }
    }

    pub fn seek(&mut self, position: usize) -> Option<&mut T> {
        self.position = position;
        self.read()
    }

    // FIXME: Should this be conditional?
    pub fn jump(&mut self, offset: isize) -> Option<&mut T> {
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
    pub fn next(&mut self) -> Option<&mut T> {
        if self.position == self.log.len() {
            None
        } else {
            self.position += 1;
            self.read()
        }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        if self.position == 0 {
            None
        } else {
            self.position -= 1;
            self.read()
        }
    }

    pub fn read(&mut self) -> Option<&mut T> {
        self.log.get(self.position)
    }
}

#[cfg(test)]
mod tests {
    mod cursor {
        use crate::types::log::Log;

        use super::*;

        #[test]
        fn cursor_seeks() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.cursor(|cursor| {
                assert_eq!(*cursor.seek(0).unwrap(), 1);
                assert_eq!(*cursor.seek(1).unwrap(), 2);
                assert_eq!(cursor.seek(2), None);
                assert_eq!(cursor.seek(3), None);
            });

            log.record(3).record(4).commit();

            log.cursor(|cursor| {
                assert_eq!(*cursor.seek(0).unwrap(), 1);
                assert_eq!(*cursor.seek(1).unwrap(), 2);
                assert_eq!(*cursor.seek(3).unwrap(), 4);
                assert_eq!(*cursor.seek(2).unwrap(), 3);
                assert_eq!(cursor.seek(4), None);
            });
        }

        #[test]
        fn cursor_prev_and_next() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.cursor(|cursor| {
                assert_eq!(*cursor.read().unwrap(), 1);
                assert_eq!(*cursor.next().unwrap(), 2);
                assert_eq!(cursor.next(), None);
                assert_eq!(*cursor.prev().unwrap(), 2);
                assert_eq!(*cursor.prev().unwrap(), 1);
                assert_eq!(cursor.prev(), None);
            });

            log.record(3).record(4).commit();

            log.cursor(|cursor| {
                assert_eq!(*cursor.read().unwrap(), 1);
                assert_eq!(*cursor.next().unwrap(), 2);
                assert_eq!(*cursor.next().unwrap(), 3);
                assert_eq!(*cursor.next().unwrap(), 4);
                assert_eq!(cursor.next(), None);
                assert_eq!(*cursor.prev().unwrap(), 4);
                assert_eq!(*cursor.prev().unwrap(), 3);
                assert_eq!(*cursor.prev().unwrap(), 2);
                assert_eq!(*cursor.prev().unwrap(), 1);
                assert_eq!(cursor.prev(), None);
            });
        }


        #[test]
        fn cursor_jumps() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.cursor(|cursor| {
                assert_eq!(*cursor.seek(0).unwrap(), 1);
                assert_eq!(*cursor.jump(1).unwrap(), 2);
                assert_eq!(cursor.jump(1), None);
                assert_eq!(*cursor.jump(-1).unwrap(), 2);
                assert_eq!(*cursor.jump(-1).unwrap(), 1);
                assert_eq!(cursor.jump(-1), None);
            });

            log.record(3).record(4).commit();

            log.cursor(|cursor| {
                assert_eq!(*cursor.seek(0).unwrap(), 1);
                assert_eq!(*cursor.jump(1).unwrap(), 2);
                assert_eq!(*cursor.jump(1).unwrap(), 3);
                assert_eq!(*cursor.jump(1).unwrap(), 4);
                assert_eq!(cursor.jump(1), None);
                assert_eq!(*cursor.jump(-1).unwrap(), 4);
                assert_eq!(*cursor.jump(-1).unwrap(), 3);
                assert_eq!(*cursor.jump(-1).unwrap(), 2);
                assert_eq!(*cursor.jump(-1).unwrap(), 1);
                assert_eq!(cursor.jump(-1), None);
            });
        }
    }
}
