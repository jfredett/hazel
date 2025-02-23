use super::*;

pub struct IndexIterator {
    source: Bitboard,
}

pub struct SquareIterator {
    source: Bitboard,
}

impl IntoIterator for Bitboard {
    type Item = Square;

    type IntoIter = SquareIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareIterator { source: self }
    }
}

impl Iterator for SquareIterator {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.is_empty() {
            None
        } else {
            let sq = Square::new(self.source.first_index());
            self.source.unset(sq);
            Some(sq)
        }
    }
}

impl Iterator for IndexIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.is_empty() {
            None
        } else {
            let idx = self.source.first_index();
            self.source.unset(Square::new(idx));
            Some(idx)
        }
    }
}

impl Bitboard {
    pub fn index_iterator(&self) -> IndexIterator {
        IndexIterator {
            source: *self
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_iterator() {
        let b = Bitboard::from(0x0000000000000001u64);
        let mut iter = b.into_iter();
        assert_eq!(iter.next(), Some(A1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn bitboard_index_iterator() {
        let b = Bitboard::from(0x0000000000000001u64);
        let mut iter = b.index_iterator();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }
}
