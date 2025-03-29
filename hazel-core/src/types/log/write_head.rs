use super::Log;

// Cursor should be a read-only version of this, which can then be used as a component of a replay
// engine that can interpret a log of chessactions including choosing between variations, etc.


pub struct WriteHead<'a, T> where T: Clone {
    log: &'a mut Log<T>,
    position: usize
}

impl<'a, T> WriteHead<'a, T>  where T: Clone {
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
            if new_position as usize >= self.log.len() {
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
        self.log.get_mut(self.position)
    }

    // FIXME: I think these are going to be necessary, but I haven't adapted the tests below to
    // exercise the writer part of this thing since pulling it from cursor. Cursor and WriteHead
    // themselves are probably some common underlying abstract class that differs only in
    // mutability, but mutability generics aren't a thing so *shrug*
    #[cfg(test)]
    #[allow(dead_code)]
    fn position(&self) -> usize {
        self.position
    }

    #[cfg(test)]
    #[allow(dead_code)]
    fn log(&self) -> &Log<T> {
        self.log
    }
}

#[cfg(test)]
mod tests {
    mod cursor {
        use crate::types::log::Log;

        use super::*;



        #[test]
        fn write_head_seeks() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.write_head(|write_head| {
                assert_eq!(*write_head.seek(0).unwrap(), 1);
                assert_eq!(*write_head.seek(1).unwrap(), 2);
                assert_eq!(write_head.seek(2), None);
                assert_eq!(write_head.seek(3), None);
            });

            log.record(3).record(4).commit();

            log.write_head(|write_head| {
                assert_eq!(*write_head.seek(0).unwrap(), 1);
                assert_eq!(*write_head.seek(1).unwrap(), 2);
                assert_eq!(*write_head.seek(3).unwrap(), 4);
                assert_eq!(*write_head.seek(2).unwrap(), 3);
                assert_eq!(write_head.seek(4), None);
            });
        }

        #[test]
        fn write_head_prev_and_next() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.write_head(|write_head| {
                assert_eq!(*write_head.read().unwrap(), 1);
                assert_eq!(*write_head.next().unwrap(), 2);
                assert_eq!(write_head.next(), None);
                assert_eq!(*write_head.prev().unwrap(), 2);
                assert_eq!(*write_head.prev().unwrap(), 1);
                assert_eq!(write_head.prev(), None);
            });

            log.record(3).record(4).commit();

            log.write_head(|write_head| {
                assert_eq!(*write_head.read().unwrap(), 1);
                assert_eq!(*write_head.next().unwrap(), 2);
                assert_eq!(*write_head.next().unwrap(), 3);
                assert_eq!(*write_head.next().unwrap(), 4);
                assert_eq!(write_head.next(), None);
                assert_eq!(*write_head.prev().unwrap(), 4);
                assert_eq!(*write_head.prev().unwrap(), 3);
                assert_eq!(*write_head.prev().unwrap(), 2);
                assert_eq!(*write_head.prev().unwrap(), 1);
                assert_eq!(write_head.prev(), None);
            });
        }


        #[test]
        fn write_head_jumps() {
            let mut log = Log::default();

            log.record(1).record(2).commit();

            log.write_head(|write_head| {
                assert_eq!(*write_head.seek(0).unwrap(), 1);
                assert_eq!(*write_head.jump(1).unwrap(), 2);
                assert_eq!(write_head.jump(1), None);
                assert_eq!(*write_head.jump(-1).unwrap(), 2);
                assert_eq!(*write_head.jump(-1).unwrap(), 1);
                assert_eq!(write_head.jump(-1), None);
            });

            log.record(3).record(4).commit();

            log.write_head(|cursor| {
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
