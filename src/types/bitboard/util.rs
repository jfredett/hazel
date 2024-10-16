use super::*;

use crate::notation::*;

impl Bitboard {
    #[inline(always)]
    pub fn coords_to_index(rank: usize, file: usize) -> usize {
        Square::from((rank, file)).index()
    }

    #[inline(always)]
    pub fn coords_to_offset(rank: usize, file: usize) -> usize {
        1 << Square::from((rank, file)).index()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod coords_indexes_and_notation {
        use super::*;

        mod coords_to_offset {
            use super::*;

            #[quickcheck]
            fn coords_to_offset_correctly(rank_i: usize, file_i: usize) -> bool {
                let (rank, file) = (rank_i % 8, file_i % 8);

                Bitboard::coords_to_offset(rank, file) == 1 << Bitboard::coords_to_index(rank, file)
            }

            #[quickcheck]
            fn coords_to_index_correctly(rank_i: usize, file_i: usize) -> bool {
                let (rank, file) = (rank_i % 8, file_i % 8);

                Bitboard::coords_to_index(rank, file) == 8 * rank + file
            }
        }
    }
}
