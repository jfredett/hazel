use crate::bitboard::*;
use std::ops::{Not, BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign};

/// Implements the various bit-ops for Bitboards
macro_rules! binop_trait {
    ($trait:ident, $method:ident) => {
        impl $trait for Bitboard {
            type Output = Bitboard;

            #[inline]
            fn $method(self, rhs: Bitboard) -> Bitboard {
                let res = $trait::$method(self.0, rhs.0);
                return Bitboard::from(res)
            }
        }
    };
}

/// Implements assigning-bit-ops for Bitboards
macro_rules! binop_assign_trait {
    ($trait:ident, $method:ident) => {
        impl $trait for Bitboard {
            #[inline]
            fn $method(&mut self, rhs: Bitboard) { $trait::$method(&mut self.0, rhs.0); }
        }
    };
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        Bitboard::from(!self.0)
    }
}

binop_trait!(BitAnd, bitand);
binop_trait!(BitXor, bitxor);
binop_trait!(BitOr, bitor);

binop_assign_trait!(BitOrAssign, bitor_assign);
binop_assign_trait!(BitXorAssign, bitxor_assign);
binop_assign_trait!(BitAndAssign, bitand_assign);


#[cfg(test)]
mod test {
    use super::*;

    mod bitops {
        use super::*;

        mod not {
            use super::*;

            #[quickcheck]
            fn ands_two_bitboards(b_i: u64) -> bool {
                let b1 = Bitboard::from(b_i);
                let expected = Bitboard::from(!b_i);

                !b1 == expected
            }
        }

        mod bitand {
            use super::*;

            #[quickcheck]
            fn ands_two_bitboards(b1_i: u64, b2_i: u64) -> bool {
                let b1 = Bitboard::from(b1_i);
                let b2 = Bitboard::from(b2_i);
                let expected = Bitboard::from(b1_i & b2_i);

                b1 & b2 == expected
            }
        }

        mod bitor {
            use super::*;

            #[quickcheck]
            fn ors_two_bitboards(b1_i: u64, b2_i: u64) -> bool {
                let b1 = Bitboard::from(b1_i);
                let b2 = Bitboard::from(b2_i);
                let expected = Bitboard::from(b1_i | b2_i);

                b1 | b2 == expected
            }
        }

        mod bitxor {
            use super::*;

            #[quickcheck]
            fn xors_two_bitboards(b1_i: u64, b2_i: u64) -> bool {
                let b1 = Bitboard::from(b1_i);
                let b2 = Bitboard::from(b2_i);
                let expected = Bitboard::from(b1_i ^ b2_i);

                b1 ^ b2 == expected
            }
        }

    }

    mod bitassignops {
        use super::*;

        mod xor_assign {
            use super::*;

            #[quickcheck]
            fn xors_two_bitboards(b1_i: u64, b2_i: u64) -> bool {
                let mut b1 = Bitboard::from(b1_i);
                let b2 = Bitboard::from(b2_i);
                let expected = Bitboard::from(b1_i ^ b2_i);

                b1 ^= b2;

                b1 == expected
            }
        }

        mod and_assign {
            use super::*;

            #[quickcheck]
            fn ands_two_bitboards(b1_i: u64, b2_i: u64) -> bool {
                let mut b1 = Bitboard::from(b1_i);
                let b2 = Bitboard::from(b2_i);
                let expected = Bitboard::from(b1_i & b2_i);

                b1 &= b2;

                b1 == expected
            }
        }

        mod or_assign {
            use super::*;

            #[quickcheck]
            fn ors_two_bitboards(b1_i: u64, b2_i: u64) -> bool {
                let mut b1 = Bitboard::from(b1_i);
                let b2 = Bitboard::from(b2_i);
                let expected = Bitboard::from(b1_i | b2_i);

                b1 |= b2;

                b1 == expected
            }
        }


    }
}

