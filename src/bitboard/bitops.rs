use super::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not, Shl, Shr};

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

impl Mul for Bitboard {
    type Output = Bitboard;
    
    fn mul(self, rhs: Bitboard) -> Bitboard {
        Bitboard::from(self.0.overflowing_mul(rhs.0).0)
    }
}

impl Mul<u64> for Bitboard {
    type Output = Bitboard;

    fn mul(self, rhs: u64) -> Bitboard {
        Bitboard::from(self.0.overflowing_mul(rhs).0)
    }
}

impl Mul<Bitboard> for u64 {
    type Output = Bitboard;

    fn mul(self, rhs: Bitboard) -> Bitboard {
        Bitboard::from(self.overflowing_mul(rhs.0).0)
    }
}

impl Shl<u64> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: u64) -> Self::Output {
        Bitboard::from(self.0 << rhs)
    }
}

impl Shr<u64> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: u64) -> Self::Output {
        Bitboard::from(self.0 >> rhs)
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

        mod mul {
            use super::*;

            #[quickcheck]
            fn multiply_bitboard_by_bitboard(b1: u64, b2: u64) -> bool {
                Bitboard::from(b1) * Bitboard::from(b2) == Bitboard::from(b1.overflowing_mul(b2).0)
            }
            
            #[quickcheck]
            fn multiply_bitboard_by_u64(b1: u64, b2: u64) -> bool {
                Bitboard::from(b1) * b2 == Bitboard::from(b1.overflowing_mul(b2).0) 
            }
            
            #[quickcheck]
            fn multiply_u64_by_bitboard(b1: u64, b2: u64) -> bool {
                b1 * Bitboard::from(b2) == Bitboard::from(b1.overflowing_mul(b2).0)
            }
        }

        mod not {
            use super::*;

            #[quickcheck]
            fn ands_two_bitboards(b: Bitboard) -> bool {
                let expected = Bitboard::from(!b.0);

                !b == expected
            }

            #[quickcheck]
            fn self_inverse(b: Bitboard) -> bool {
                !!b == b
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

            #[quickcheck]
            fn idempotence(b: Bitboard) -> bool {
                b & b == b
            }

            #[quickcheck]
            fn commutativity(b1: Bitboard, b2: Bitboard) -> bool {
                b1 & b2 == b2 & b1
            }

            #[quickcheck]
            fn associativity(a: Bitboard, b: Bitboard, c: Bitboard) -> bool {
                (a & b) & c == a & (b & c)
            }
        }

        mod bitor {
            use super::*;

            #[quickcheck]
            fn ors_two_bitboards(b1: Bitboard, b2: Bitboard) -> bool {
                let expected = Bitboard::from(b1.0 | b2.0);

                b1 | b2 == expected
            }

            #[quickcheck]
            fn idempotence(b: Bitboard) -> bool {
                b | b == b
            }

            #[quickcheck]
            fn commutativity(b1: Bitboard, b2: Bitboard) -> bool {
                b1 | b2 == b2 | b1
            }

            #[quickcheck]
            fn associativity(a: Bitboard, b: Bitboard, c: Bitboard) -> bool {
                (a | b) | c == a | (b | c)
            }
        }


        mod bitxor {
            use super::*;

            #[quickcheck]
            fn xors_two_bitboards(b1: Bitboard, b2: Bitboard) -> bool {
                let expected = Bitboard::from(b1.0 ^ b2.0);

                b1 ^ b2 == expected
            }

            #[quickcheck]
            fn commutativity(b1: Bitboard, b2: Bitboard) -> bool {
                b1 ^ b2 == b2 ^ b1
            }

            #[quickcheck]
            fn self_inverse(b: Bitboard) -> bool {
                b ^ b ^ b == b
            }

            #[quickcheck]
            fn associativity(a: Bitboard, b: Bitboard, c: Bitboard) -> bool {
                (a ^ b) ^ c == a ^ (b ^ c)
            }
        }

        mod laws {
            use super::*;

            #[quickcheck]
            fn ditributativity_of_or_over_and(a: Bitboard, b: Bitboard, c: Bitboard) -> bool {
                a | (b & c) == (a | b) & (a | c)
            }

            #[quickcheck]
            fn ditributativity_of_and_over_or(a: Bitboard, b: Bitboard, c: Bitboard) -> bool {
                a & (b | c) == (a & b) | (a & c)
            }

            #[quickcheck]
            fn ditributativity_of_and_over_xor(a: Bitboard, b: Bitboard, c: Bitboard) -> bool {
                a & (b ^ c) == (a & b) ^ (a & c)
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

