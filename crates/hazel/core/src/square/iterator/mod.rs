use crate::file::File;

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

    pub fn along_rank(rank: usize) -> RankIterator {
        RankIterator {
            rank,
            file: None,
            direction: FileDirection::LeftToRight,
        }
    }

    pub fn along_file(file: File) -> FileIterator {
        FileIterator {
            rank: None,
            file,
            direction: RankDirection::Upward,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
/// Fix a rank, proceed in the given file-direction
pub struct RankIterator {
    rank: usize,
    file: Option<File>,
    direction: FileDirection,
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// Fix a file, proceed in the given rank-direction
pub struct FileIterator {
    rank: Option<usize>,
    file: File,
    direction: RankDirection,
}

impl Iterator for RankIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        match self.direction {
            FileDirection::LeftToRight => {
                if self.file.is_none() { self.file = Some(File::A); }
                else if self.file == Some(File::H) { return None; }
                else { self.file = self.file.unwrap().next(); }
            }
            FileDirection::RightToLeft => {
                if self.file.is_none() { self.file = Some(File::H); }
                else if self.file == Some(File::A) { return None; }
                else { self.file = self.file.unwrap().prev(); }
            }
        }
        Some(Square::from((self.rank, self.file.unwrap())))
    }
}

impl Iterator for FileIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        match self.direction {
            RankDirection::Upward => {
                if self.rank.is_none() { self.rank = Some(0); }
                else if self.rank == Some(7) { return None; }
                else if let Some(r) = self.rank.as_mut() { *r += 1; }
            }
            RankDirection::Downward => {
                if self.rank.is_none() { self.rank = Some(8); }
                else if self.rank == Some(0) { return None; }
                else if let Some(r) = self.rank.as_mut() { *r -= 1; }
            }
        }
        Some(Square::from((self.rank.unwrap(), self.file)))
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

    mod rank_iterator {
        use super::*;

        #[test]
        fn rank_iterator() {
            let mut iter = Square::along_rank(2);
            assert_eq!(iter.next(), Some(A3));
            assert_eq!(iter.next(), Some(B3));
            assert_eq!(iter.next(), Some(C3));
            assert_eq!(iter.next(), Some(D3));
            assert_eq!(iter.next(), Some(E3));
            assert_eq!(iter.next(), Some(F3));
            assert_eq!(iter.next(), Some(G3));
            assert_eq!(iter.next(), Some(H3));
            assert_eq!(iter.next(), None);
        }
    }

    mod file_iterator {
        use super::*;

        #[test]
        fn file_iterator() {
            let mut iter = Square::along_file(File::D);
            assert_eq!(iter.next(), Some(D1));
            assert_eq!(iter.next(), Some(D2));
            assert_eq!(iter.next(), Some(D3));
            assert_eq!(iter.next(), Some(D4));
            assert_eq!(iter.next(), Some(D5));
            assert_eq!(iter.next(), Some(D6));
            assert_eq!(iter.next(), Some(D7));
            assert_eq!(iter.next(), Some(D8));
            assert_eq!(iter.next(), None);
        }
    }
}

