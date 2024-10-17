use super::*;

pub struct IndexIterator {
    source: Bitboard,
}

impl IntoIterator for Bitboard {
    type Item = usize;

    type IntoIter = IndexIterator;

    fn into_iter(self) -> Self::IntoIter {
        IndexIterator { source: self }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_iterator() {
        let b = Bitboard::from(0x0000000000000001u64);
        let mut iter = b.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }
}
