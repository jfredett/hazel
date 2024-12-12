use super::Log;


#[derive(Debug, Clone)]
pub struct Cursor<'a, T> where T: Clone {
    log: &'a Log<T>,
    position: Option<usize>
}

impl<'a, T> Cursor<'a, T>  where T: Clone {
    pub fn new(log: &'a Log<T>) -> Self {
        Self {
            log,
            position: None
        }
    }

    pub fn position(&self) -> usize {
        self.position.unwrap_or(0)
    }

    pub fn seek(&mut self, position: usize) -> Option<&T> {
        self.position = Some(position);
        self.read()
    }

    pub fn jump(&mut self, offset: isize) -> Option<&T> {
        // FIXME: I think this should probably not be None, but I don't know what the convenient
        // API should be, so for now this is what it is.
        let new_position = self.position? as isize + offset;

        if new_position < 0 {
            self.position = Some(0);
            None
        } else {
            if new_position as usize >= self.log.log.len() {
                self.position = Some(self.log.len());
            } else {
                self.position = Some(new_position as usize);
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
    pub fn next(&mut self) -> Option<&T> {
        match self.position {
            None => {
                self.position = Some(0);
                self.read()
            }
            Some(position) => {
                if position == self.log.len() {
                    None
                } else {
                    self.position = Some(position + 1);
                    self.read()
                }
            }
        }
    }

    pub fn prev(&mut self) -> Option<&T> {
        match self.position {
            None => {
                self.position = Some(self.log.len() - 1);
                self.read()
            }
            Some(position) => {
                if position == 0 {
                    None
                } else {
                    self.position = Some(position - 1);
                    self.read()
                }
            }
        }
    }

    pub fn read(&mut self) -> Option<&T> {
        match self.position {
            None => None,
            Some(position) => self.log.get(position)
        }
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
                assert_eq!(*cursor.next().unwrap(), 1);
                assert_eq!(*cursor.next().unwrap(), 2);
                assert_eq!(cursor.next(), None);
                assert_eq!(*cursor.prev().unwrap(), 2);
                assert_eq!(*cursor.prev().unwrap(), 1);
                assert_eq!(cursor.prev(), None);
            });

            log.record(3).record(4).commit();

            log.cursor(|cursor| {
                assert_eq!(*cursor.next().unwrap(), 1);
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
