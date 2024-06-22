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
            self.source.unset_by_index(idx);
            Some(idx)
        }
    }
}
