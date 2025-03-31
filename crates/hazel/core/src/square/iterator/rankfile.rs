use super::*;
use std::fmt::Debug;
use std::iter::Iterator;

#[derive(Clone, Copy)]
pub struct RankFile {
    done: bool,
    rank: usize,
    file: usize,
    rank_direction: RankDirection,
    file_direction: FileDirection,
}

impl Debug for RankFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({:?}, {:?})", self.current_square(), self.rank_direction, self.file_direction)
    }
}

impl PartialEq<Square> for &mut RankFile {
    fn eq(&self, other: &Square) -> bool {
        self.current_square() == *other
    }
}

impl PartialEq<Square> for RankFile {
    fn eq(&self, other: &Square) -> bool {
        self.current_square() == *other
    }
}

impl Default for RankFile {
    /// By Default, a RankFile iterator iterates from A1 to H8
    fn default() -> Self {
        Self {
            done: false,
            rank: 0,
            file: 0,
            rank_direction: RankDirection::Upward,
            file_direction: FileDirection::LeftToRight,
        }
    }
}

impl From<Square> for RankFile {
    fn from(square: Square) -> Self {
        Self {
            done: false,
            rank: square.rank(),
            file: square.file(),
            rank_direction: RankDirection::Upward,
            file_direction: FileDirection::LeftToRight,
        }
    }
}

impl Iterator for RankFile {
    type Item = Square;

    #[cfg_attr(test, mutants::skip)]
    fn next(&mut self) -> Option<Square> {
        if self.done { return None; }

        let current = self.current_square();

        let s = Square::new(self.rank * 8 + self.file);

        if s == self.last_square() {
            self.done = true;
            return Some(current);
        }

        match self.file_direction {
            FileDirection::LeftToRight => {
                if self.file == 7 {
                    self.file = 0;
                    match self.rank_direction {
                        RankDirection::Upward => { self.rank += 1; }
                        RankDirection::Downward => { self.rank -= 1; }
                    }
                } else {
                    self.file += 1;
                }
            }
            FileDirection::RightToLeft => {
                if self.file == 0 {
                    self.file = 7;
                    match self.rank_direction {
                        RankDirection::Upward => { self.rank += 1; }
                        RankDirection::Downward => { self.rank -= 1; }
                    }
                } else {
                    self.file -= 1;
                }
            }
        }

        Some(current)
    }
}

impl RankFile {

    ///
    ///
    /// 8 W . . . . . . X
    /// 7 . . . . . . . .
    /// 6 . . . . . . . .
    /// 5 . . . . . . . .
    /// 4 . . . . . . . .
    /// 3 . . . . . . . .
    /// 2 . . . . . . . .
    /// 1 Z . . . . . . Y
    ///   a b c d e f g h
    ///
    /// In the above, you always _end_ at the corner _opposite where you start_ when starting in
    /// the default positions. So if you choose `downward` and `right_to_left` you will start on
    /// the top rank by default, meaning either W or X, and right to left means you choose the
    /// rightmost option, so you start on X (H8).
    ///
    /// You end, therefore, on Z (A1). Similar for the rest.
    ///
    fn last_square(&self) -> Square {
        match self.rank_direction {
            RankDirection::Upward => {
                match self.file_direction {
                    FileDirection::LeftToRight => H8,
                    FileDirection::RightToLeft => A8,
                }
            }
            RankDirection::Downward => {
                match self.file_direction {
                    FileDirection::LeftToRight => H1,
                    FileDirection::RightToLeft => A1,
                }
            }
        }
    }

    /// True if the iterator has reached the end and been marked as 'done'
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// Sets the iterator to go left to right, snaps the file to the leftmost value.
    /// ```
    /// # use hazel_core::square::*;
    ///
    /// let mut a = RankFile::default();
    /// a.left_to_right();
    /// assert_eq!(a, A1);
    /// a.next();
    /// assert_eq!(a, B1);
    /// ```
    pub fn left_to_right(&mut self) -> &mut Self {
        self.file_direction = FileDirection::LeftToRight;
        self.file = 0;
        self
    }

    /// Sets the iterator to go right to left, snaps the file to the rightmost value.
    /// ```
    /// # use hazel_core::square::*;
    ///
    /// let mut a = RankFile::default();
    /// a.right_to_left();
    /// assert_eq!(a, H1);
    /// a.next();
    /// assert_eq!(a, G1);
    /// ```
    pub fn right_to_left(&mut self) -> &mut Self {
        self.file_direction = FileDirection::RightToLeft;
        self.file = 7;
        self
    }

    /// Sets the iterator to go upward, snaps the rank to the bottommost value.
    /// ```
    /// # use hazel_core::square::*;
    ///
    /// let mut a = RankFile::default();
    /// a.upward();
    /// assert_eq!(a, A1);
    /// a.next();
    /// assert_eq!(a, B1);
    /// ```
    pub fn upward(&mut self) -> &mut Self {
        self.rank_direction = RankDirection::Upward;
        self.rank = 0;
        self
    }

    /// Sets the iterator to go downward, snaps the rank to the topmost value.
    /// ```
    /// # use hazel_core::square::*;
    ///
    /// let mut a = RankFile::default();
    /// a.downward();
    /// assert_eq!(a, A8);
    /// a.next();
    /// assert_eq!(a, B8);
    /// ```
    pub fn downward(&mut self) -> &mut Self {
        self.rank_direction = RankDirection::Downward;
        self.rank = 7;
        self
    }

    /// Sets the start square for the iterator
    /// ```
    /// # use hazel_core::square::*;
    ///
    /// let mut a = RankFile::default();
    /// a.start_on(H1);
    /// assert_eq!(a, H1);
    ///
    /// ```
    pub fn start_on(&mut self, square: Square) -> &mut Self {
        self.rank = square.rank();
        self.file = square.file();
        self
    }

    /// Returns the current square of the iterator, generally not needed (the `PartialEq`
    /// implementation allows direct comparison with a `Square`).
    /// ```
    /// # use hazel_core::square::*;
    ///
    /// let mut a = RankFile::default();
    /// a.start_on(H1);
    /// assert_eq!(a.current_square(), H1);
    ///
    /// ```
    pub fn current_square(&self) -> Square {
        Square::new(self.rank * 8 + self.file)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    mod rankfile {
        use super::*;

        #[quickcheck]
        fn from_square(s: usize) -> bool {
            let square = Square::new(s % 64);
            let rankfile = RankFile::from(square);
            rankfile == square
        }

        mod touches_all_squares {
            use super::*;

            #[test]
            fn when_default() {
                let a = RankFile::default();
                let mut v = vec![];
                for s in a {
                    v.push(s);
                }
                assert_eq!(v.len(), 64);
            }

            #[test]
            fn when_downwards() {
                let mut a = RankFile::default();
                a.downward();
                let mut v = vec![];
                for s in a {
                    v.push(s);
                }
                assert_eq!(v.len(), 64);
            }

            #[test]
            fn when_right_to_left() {
                let mut a = RankFile::default();
                a.right_to_left();
                let mut v = vec![];
                for s in a {
                    v.push(s);
                }
                assert_eq!(v.len(), 64);
            }

            #[test]
            fn when_downward_and_right_to_left() {
                let mut a = RankFile::default();
                a.downward().right_to_left();
                let mut v = vec![];
                for s in a {
                    v.push(s);
                }
                assert_eq!(v.len(), 64);
            }

            #[test]
            fn when_start_on_h8() {
                let mut a = RankFile::default();
                a.downward().right_to_left().start_on(H8);
                let mut v = vec![];
                for s in a {
                    v.push(s);
                }
                assert_eq!(v.len(), 64);
            }

        }

        mod left_to_right {
            use super::*;

            mod upward {
                use super::*;

                #[test]
                fn first_square_is_a1() {
                    let a = RankFile::default();
                    assert_eq!(a, A1);
                }

                #[test]
                fn last_square_is_h8() {
                    let a = RankFile::default();
                    assert_eq!(a.last_square(), H8);
                }

                #[test]
                fn left_to_right_and_upward_is_default_starts_on_a1() {
                    let mut a = RankFile::default();
                    assert_eq!(a, A1);

                    a.next();

                    assert_eq!(a, B1);
                }

                #[test]
                fn left_to_right_and_upward_wraps() {
                    let mut a = RankFile::default();
                    a.start_on(H1);
                    assert_eq!(a, H1);
                    a.next();
                    assert_eq!(a, A2);

                }
            }

            mod downward {
                use super::*;

                #[test]
                fn first_square_is_a8() {
                    let mut a = RankFile::default();
                    a.downward();
                    assert_eq!(a, A8);
                }

                #[test]
                fn last_square_is_h1() {
                    let mut a = RankFile::default();
                    a.downward();
                    assert_eq!(a.last_square(), H1);
                }

                #[test]
                fn left_to_right_and_downward_starts_on_a8() {
                    let mut a = RankFile::default();
                    a.left_to_right().downward();

                    assert_eq!(a, A8);
                    a.next();
                    assert_eq!(a, B8);
                }

                #[test]
                fn left_to_right_and_downward_wraps() {
                    let mut a = RankFile::default();
                    a.downward().start_on(H8);
                    assert_eq!(a, H8);
                    a.next();
                    assert_eq!(a, A7);
                }
            }
        }

        mod right_to_left {
            use super::*;

            mod upward {
                use super::*;

                #[test]
                fn first_square_is_h1() {
                    let mut a = RankFile::default();
                    a.right_to_left();
                    assert_eq!(a, H1);
                }

                #[test]
                fn last_square_is_a8() {
                    let mut a = RankFile::default();
                    a.right_to_left();
                    assert_eq!(a.last_square(), A8);
                }

                #[test]
                fn right_to_left_and_upward_starts_on_h1() {
                    let mut a = RankFile::default();
                    a.right_to_left().upward();

                    assert_eq!(a, H1);
                    a.next();
                    assert_eq!(a, G1);
                }

                #[test]
                fn right_to_left_and_upward_wraps() {
                    let mut a = RankFile::default();
                    a.right_to_left().upward().start_on(A1);

                    assert_eq!(a, A1);
                    a.next();
                    assert_eq!(a, H2);
                }
            }

            mod downward {
                use super::*;

                #[test]
                fn first_square_is_h8() {
                    let mut a = RankFile::default();
                    a.downward().right_to_left();
                    assert_eq!(a, H8);
                }

                #[test]
                fn last_square_is_a1() {
                    let mut a = RankFile::default();
                    a.downward().right_to_left();
                    assert_eq!(a.last_square(), A1);
                }


                #[test]
                fn right_to_left_and_downward_starts_on_h8() {
                    let mut a = RankFile::default();
                    a.right_to_left().downward();

                    assert_eq!(a, H8);
                    a.next();
                    assert_eq!(a, G8);
                }

                #[test]
                fn right_to_left_and_downward_wraps() {
                    let mut a = RankFile::default();
                    a.right_to_left().downward().start_on(A8);

                    assert_eq!(a, A8);
                    a.next();
                    assert_eq!(a, H7);
                }
            }
        }
    }
}
