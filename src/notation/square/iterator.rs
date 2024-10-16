use super::*;

use std::fmt::Debug;
use std::iter::IntoIterator;
use std::iter::Iterator;

/// The Standard Iterator for Squares
impl Iterator for Square {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.index() < 64 {
            let ret = self.clone();
            self.0 += 1;
            return Some(ret);
        } else {
            return None;
        }
    }
}

impl Square {
    pub fn by_rank_and_file() -> RankFile {
        *&RankFile::default()
    }

    pub fn fenwise() -> RankFile {
        *RankFile::default().downward().left_to_right()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RankDirection {
    Upward,
    Downward,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum FileDirection {
    LeftToRight,
    RightToLeft,
}

#[derive(Clone, Copy)]
pub struct RankFile {
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
            rank: square.rank(),
            file: square.file(),
            rank_direction: RankDirection::Upward,
            file_direction: FileDirection::LeftToRight,
        }
    }
}

impl Iterator for RankFile {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        let current = self.current_square();

        match self.file_direction {
            FileDirection::LeftToRight => {
                if self.file == 7 {
                    self.file = 0;
                    match self.rank_direction {
                        RankDirection::Upward => {
                            if self.rank == 7 {
                                return None;
                            } else {
                                self.rank += 1;
                            }
                        }
                        RankDirection::Downward => {
                            if self.rank == 0 {
                                return None;
                            } else {
                                self.rank -= 1;
                            }
                        }
                    }
                } else {
                    self.file += 1;
                }
            }
            FileDirection::RightToLeft => {
                if self.file == 0 {
                    self.file = 7;
                    match self.rank_direction {
                        RankDirection::Upward => {
                            if self.rank == 7 {
                                return None;
                            } else {
                                self.rank += 1;
                            }
                        }
                        RankDirection::Downward => {
                            if self.rank == 0 {
                                return None;
                            } else {
                                self.rank -= 1;
                            }
                        }
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
    /// Sets the iterator to go left to right, snaps the file to the leftmost value.
    /// ```
    /// # use hazel::notation::*;
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
    /// # use hazel::notation::*;
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
    /// # use hazel::notation::*;
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
    /// # use hazel::notation::*;
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
    /// # use hazel::notation::*;
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
    /// # use hazel::notation::*;
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

/// The FEN Iterator for Squares
///
/// It iterates from rank 8 to 0, from A to H. So Reverse Ranks, Forward Files

/*
struct FENWise {
inner: RankFile
}

impl Default for FENWise {
fn default() -> Self {
Self {
inner: RankFile {
rank: 8,
file: 0,
rank_direction: RankDirection::Downward,
file_direction: FileDirection::LeftToRight,
}
}
}
}

impl IntoIterator for FENWise {
type Item = Square;

fn into_iter(self) -> Self::IntoIter {
self
}
}

impl Iterator for FENWise {

}
*/


#[cfg(test)]
mod tests {
    use super::*;

    mod iterator {
        use super::*;

        mod standard {
            use super::*;

            #[test]
            fn iterator() {
                let mut s = A1;


                assert_eq!(s.next(), Some(A1));
                assert_eq!(s.next(), Some(B1));
                assert_eq!(s.next(), Some(C1));

                s = H8;

                assert_eq!(s.next(), Some(H8));
                assert_eq!(s.next(), None);
            }

            #[test]
            fn for_loop() {
                let mut idx = 0;
                for s in A1 {
                    assert_eq!(s.index(), idx);
                    idx += 1;
                }
            }
        }

        mod rankfile {
            use super::*;

            mod left_to_right {
                use super::*;

                mod upward {
                    use super::*;
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

        mod square {
            use super::*;

            #[test]
            fn by_rank_and_file() {
                // FIXME: I don't like this API, pretty sure I'm doing something wrong with the
                // `intoiter` thing.
                let mut iter = Square::by_rank_and_file();
                iter.upward().left_to_right().start_on(A2);
                assert_eq!(iter, A2);
                iter.next();
                assert_eq!(iter, B2);
            }
        }

        mod display_and_debug {
            use super::*;

            #[test]
            fn rankfile_display() {
                let a = RankFile::default();
                assert_eq!(format!("{:?}", a), "a1 (0) (Upward, LeftToRight)");
            }

            #[test]
            fn rankfile_debug() {
                let mut a = RankFile::default();
                a.right_to_left().downward().start_on(H8);
                assert_eq!(format!("{:?}", a), "h8 (63) (Downward, RightToLeft)");
            }

        }
    }
}

