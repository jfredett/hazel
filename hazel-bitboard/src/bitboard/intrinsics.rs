use super::*;
use std::convert::TryInto;

impl Bitboard {
    /// Executes a Parallel Extract using the given bitboard as a mask. Requires the BMI2 instruction set to be supported.
    #[cfg(target_feature = "bmi2")]
    pub fn pext(&self, mask: Bitboard) -> u64 {
        unsafe { core::arch::x86_64::_pext_u64(self.0, mask.0) }
    }

    #[cfg(not(target_feature = "bmi2"))]
    pub fn pext(&self, _mask: Bitboard) -> u64 {
        panic!("Hazel currently requires CPUs which support the BMI2 instruction set, see CPU-REQUIREMENTS for details")
    }

    /// The index of the first set bit, e.g.:
    /// ```
    /// # use hazel::types::Bitboard;
    /// let mut b = Bitboard::empty();
    /// b.set_by_index(10);
    /// b.set_by_index(20);
    /// b.set_by_index(30);
    /// assert_eq!(b.all_set_indices()[0], 10);
    /// assert_eq!(b.first_index(), 10)
    /// ```
    ///
    /// Uses the tzcnt instruction to avoid slow loops. This requires the BMI1 instruction set to be available.
    #[cfg(target_feature = "bmi1")]
    pub fn first_index(&self) -> usize {
        unsafe {
            (core::arch::x86_64::_mm_tzcnt_64(self.0))
                .try_into()
                .unwrap()
        }
    }

    #[cfg(not(target_feature = "bmi1"))]
    pub fn first_index(&self) -> usize {
        panic!("Hazel currently requires CPUs which support the BMI1 instruction set, see CPU-REQUIREMENTS for details")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod pext {
        use super::*;

        #[quickcheck]
        fn pext_pexts_correctly(bb1_in: u64, mask_in: u64) -> bool {
            let bb = Bitboard::new(bb1_in);
            let mask = Bitboard::new(mask_in);

            unsafe { core::arch::x86_64::_pext_u64(bb1_in, mask_in) == bb.pext(mask) }
        }
    }

    mod first_index {
        use super::*;

        #[quickcheck]
        fn first_index_calculated_index_correctly(bb_in: u64) -> bool {
            let bb = Bitboard::from_index((bb_in % 64) as usize);
            (bb_in % 64) as usize == bb.first_index()
        }

        #[quickcheck]
        fn first_index_is_equilvalent_to_all_set_indices_at_position_zero_for_nonempty_bitboards(
            bb: Bitboard,
        ) -> bool {
            if bb.is_empty() {
                return true;
            }
            bb.all_set_squares()[0].index() == bb.first_index()
        }
    }
}
