use crate::constants::File;

use super::*;

mod rankfile;


pub use rankfile::*;


/// The Standard Iterator for Squares
impl Iterator for Square {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.index() < 64 {
            let ret = *self;
            self.0 += 1;
            Some(ret)
        } else {
            None
        }
    }
}

impl Square {
    pub fn by_rank_and_file() -> RankFile {
        RankFile::default()
    }

    pub fn fenwise() -> RankFile {
        *RankFile::default().downward().left_to_right()
    }

    pub fn along_rank(rank: usize) -> RankFile {
        *RankFile::default().downward().left_to_right().start_on(Square::new(rank * 8))
    }

    pub fn along_file(file: File) -> RankFile {
        *RankFile::default().downward().left_to_right().start_on(Square::new(file.to_index()))
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


#[cfg(test)]
mod tests {
    use super::*;



    mod iterator {
        use super::*;

        // FIXME: Ideally this mode gets deprecated in favor of the RankFile iterator
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
                for (idx, s) in A1.enumerate() {
                    assert_eq!(s.index(), idx);
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

            #[test]
            fn reaches_all_squares() {
                let mut iter = Square::by_rank_and_file();
                iter.start_on(G8);
                assert_eq!(iter, G8);
                iter.next();
                assert_eq!(iter, H8);
                iter.next();
                assert_eq!(iter.next(), None);
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

