use super::*;

impl Bitboard {
    /// Mostly for test convenience, this function may be used to convert typical
    /// algebraic notation ("a2", "b2", etc) to 1-indexed coordinates.
    pub fn notation_to_coords(notation: &str) -> (usize, usize) {
        Bitboard::index_to_coords(Bitboard::notation_to_index(notation))
    }

    #[inline(always)]
    pub fn coords_to_notation(rank: usize, file: usize) -> &'static str {
        let index = Bitboard::coords_to_index(rank, file);
        INDEX_TO_NOTATION[index]
    }

    #[inline(always)]
    pub fn notation_to_index(notation: &str) -> usize {
        NOTATION_TO_INDEX(notation)
    }

    #[inline(always)]
    pub fn coords_to_index(rank: usize, file: usize) -> usize {
        COORDS_TO_INDEX[rank][file]
    }

    #[inline(always)]
    pub fn index_to_coords(idx: usize) -> (usize, usize) {
        INDEX_TO_COORDS[idx]
    }

    #[inline(always)]
    pub fn coords_to_offset(rank: usize, file: usize) -> usize {
        1 << COORDS_TO_INDEX[rank][file]
    }

    /// _For Test Convenience Only_, not performant at all.
    #[inline(always)]
    pub fn index_to_notation(idx: usize) -> &'static str {
        INDEX_TO_NOTATION[idx]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod coords_indexes_and_notation {
        use super::*;
        mod correctly_parses_to_index {
            use super::*;
            #[test]
            fn a1() {
                assert_eq!(Bitboard::notation_to_index("a1"), 0o00);
            }
            #[test]
            fn a2() {
                assert_eq!(Bitboard::notation_to_index("a2"), 0o10);
            }
            #[test]
            fn a3() {
                assert_eq!(Bitboard::notation_to_index("a3"), 0o20);
            }
            #[test]
            fn a4() {
                assert_eq!(Bitboard::notation_to_index("a4"), 0o30);
            }
            #[test]
            fn a5() {
                assert_eq!(Bitboard::notation_to_index("a5"), 0o40);
            }
            #[test]
            fn a6() {
                assert_eq!(Bitboard::notation_to_index("a6"), 0o50);
            }
            #[test]
            fn a7() {
                assert_eq!(Bitboard::notation_to_index("a7"), 0o60);
            }
            #[test]
            fn a8() {
                assert_eq!(Bitboard::notation_to_index("a8"), 0o70);
            }

            #[test]
            fn b1() {
                assert_eq!(Bitboard::notation_to_index("b1"), 0o01);
            }
            #[test]
            fn b2() {
                assert_eq!(Bitboard::notation_to_index("b2"), 0o11);
            }
            #[test]
            fn b3() {
                assert_eq!(Bitboard::notation_to_index("b3"), 0o21);
            }
            #[test]
            fn b4() {
                assert_eq!(Bitboard::notation_to_index("b4"), 0o31);
            }
            #[test]
            fn b5() {
                assert_eq!(Bitboard::notation_to_index("b5"), 0o41);
            }
            #[test]
            fn b6() {
                assert_eq!(Bitboard::notation_to_index("b6"), 0o51);
            }
            #[test]
            fn b7() {
                assert_eq!(Bitboard::notation_to_index("b7"), 0o61);
            }
            #[test]
            fn b8() {
                assert_eq!(Bitboard::notation_to_index("b8"), 0o71);
            }

            #[test]
            fn c1() {
                assert_eq!(Bitboard::notation_to_index("c1"), 0o02);
            }
            #[test]
            fn c2() {
                assert_eq!(Bitboard::notation_to_index("c2"), 0o12);
            }
            #[test]
            fn c3() {
                assert_eq!(Bitboard::notation_to_index("c3"), 0o22);
            }
            #[test]
            fn c4() {
                assert_eq!(Bitboard::notation_to_index("c4"), 0o32);
            }
            #[test]
            fn c5() {
                assert_eq!(Bitboard::notation_to_index("c5"), 0o42);
            }
            #[test]
            fn c6() {
                assert_eq!(Bitboard::notation_to_index("c6"), 0o52);
            }
            #[test]
            fn c7() {
                assert_eq!(Bitboard::notation_to_index("c7"), 0o62);
            }
            #[test]
            fn c8() {
                assert_eq!(Bitboard::notation_to_index("c8"), 0o72);
            }

            #[test]
            fn d1() {
                assert_eq!(Bitboard::notation_to_index("d1"), 0o03);
            }
            #[test]
            fn d2() {
                assert_eq!(Bitboard::notation_to_index("d2"), 0o13);
            }
            #[test]
            fn d3() {
                assert_eq!(Bitboard::notation_to_index("d3"), 0o23);
            }
            #[test]
            fn d4() {
                assert_eq!(Bitboard::notation_to_index("d4"), 0o33);
            }
            #[test]
            fn d5() {
                assert_eq!(Bitboard::notation_to_index("d5"), 0o43);
            }
            #[test]
            fn d6() {
                assert_eq!(Bitboard::notation_to_index("d6"), 0o53);
            }
            #[test]
            fn d7() {
                assert_eq!(Bitboard::notation_to_index("d7"), 0o63);
            }
            #[test]
            fn d8() {
                assert_eq!(Bitboard::notation_to_index("d8"), 0o73);
            }

            #[test]
            fn e1() {
                assert_eq!(Bitboard::notation_to_index("e1"), 0o04);
            }
            #[test]
            fn e2() {
                assert_eq!(Bitboard::notation_to_index("e2"), 0o14);
            }
            #[test]
            fn e3() {
                assert_eq!(Bitboard::notation_to_index("e3"), 0o24);
            }
            #[test]
            fn e4() {
                assert_eq!(Bitboard::notation_to_index("e4"), 0o34);
            }
            #[test]
            fn e5() {
                assert_eq!(Bitboard::notation_to_index("e5"), 0o44);
            }
            #[test]
            fn e6() {
                assert_eq!(Bitboard::notation_to_index("e6"), 0o54);
            }
            #[test]
            fn e7() {
                assert_eq!(Bitboard::notation_to_index("e7"), 0o64);
            }
            #[test]
            fn e8() {
                assert_eq!(Bitboard::notation_to_index("e8"), 0o74);
            }

            #[test]
            fn f1() {
                assert_eq!(Bitboard::notation_to_index("f1"), 0o05);
            }
            #[test]
            fn f2() {
                assert_eq!(Bitboard::notation_to_index("f2"), 0o15);
            }
            #[test]
            fn f3() {
                assert_eq!(Bitboard::notation_to_index("f3"), 0o25);
            }
            #[test]
            fn f4() {
                assert_eq!(Bitboard::notation_to_index("f4"), 0o35);
            }
            #[test]
            fn f5() {
                assert_eq!(Bitboard::notation_to_index("f5"), 0o45);
            }
            #[test]
            fn f6() {
                assert_eq!(Bitboard::notation_to_index("f6"), 0o55);
            }
            #[test]
            fn f7() {
                assert_eq!(Bitboard::notation_to_index("f7"), 0o65);
            }
            #[test]
            fn f8() {
                assert_eq!(Bitboard::notation_to_index("f8"), 0o75);
            }

            #[test]
            fn g1() {
                assert_eq!(Bitboard::notation_to_index("g1"), 0o06);
            }
            #[test]
            fn g2() {
                assert_eq!(Bitboard::notation_to_index("g2"), 0o16);
            }
            #[test]
            fn g3() {
                assert_eq!(Bitboard::notation_to_index("g3"), 0o26);
            }
            #[test]
            fn g4() {
                assert_eq!(Bitboard::notation_to_index("g4"), 0o36);
            }
            #[test]
            fn g5() {
                assert_eq!(Bitboard::notation_to_index("g5"), 0o46);
            }
            #[test]
            fn g6() {
                assert_eq!(Bitboard::notation_to_index("g6"), 0o56);
            }
            #[test]
            fn g7() {
                assert_eq!(Bitboard::notation_to_index("g7"), 0o66);
            }
            #[test]
            fn g8() {
                assert_eq!(Bitboard::notation_to_index("g8"), 0o76);
            }

            #[test]
            fn h1() {
                assert_eq!(Bitboard::notation_to_index("h1"), 0o07);
            }
            #[test]
            fn h2() {
                assert_eq!(Bitboard::notation_to_index("h2"), 0o17);
            }
            #[test]
            fn h3() {
                assert_eq!(Bitboard::notation_to_index("h3"), 0o27);
            }
            #[test]
            fn h4() {
                assert_eq!(Bitboard::notation_to_index("h4"), 0o37);
            }
            #[test]
            fn h5() {
                assert_eq!(Bitboard::notation_to_index("h5"), 0o47);
            }
            #[test]
            fn h6() {
                assert_eq!(Bitboard::notation_to_index("h6"), 0o57);
            }
            #[test]
            fn h7() {
                assert_eq!(Bitboard::notation_to_index("h7"), 0o67);
            }
            #[test]
            fn h8() {
                assert_eq!(Bitboard::notation_to_index("h8"), 0o77);
            }
        }

        mod correctly_parses_to_coords {
            use super::*;

            #[test]
            fn a1() {
                assert_eq!(Bitboard::notation_to_coords("a1"), (0, 0));
            }
            #[test]
            fn a2() {
                assert_eq!(Bitboard::notation_to_coords("a2"), (1, 0));
            }
            #[test]
            fn a3() {
                assert_eq!(Bitboard::notation_to_coords("a3"), (2, 0));
            }
            #[test]
            fn a4() {
                assert_eq!(Bitboard::notation_to_coords("a4"), (3, 0));
            }
            #[test]
            fn a5() {
                assert_eq!(Bitboard::notation_to_coords("a5"), (4, 0));
            }
            #[test]
            fn a6() {
                assert_eq!(Bitboard::notation_to_coords("a6"), (5, 0));
            }
            #[test]
            fn a7() {
                assert_eq!(Bitboard::notation_to_coords("a7"), (6, 0));
            }
            #[test]
            fn a8() {
                assert_eq!(Bitboard::notation_to_coords("a8"), (7, 0));
            }

            #[test]
            fn b1() {
                assert_eq!(Bitboard::notation_to_coords("b1"), (0, 1));
            }
            #[test]
            fn b2() {
                assert_eq!(Bitboard::notation_to_coords("b2"), (1, 1));
            }
            #[test]
            fn b3() {
                assert_eq!(Bitboard::notation_to_coords("b3"), (2, 1));
            }
            #[test]
            fn b4() {
                assert_eq!(Bitboard::notation_to_coords("b4"), (3, 1));
            }
            #[test]
            fn b5() {
                assert_eq!(Bitboard::notation_to_coords("b5"), (4, 1));
            }
            #[test]
            fn b6() {
                assert_eq!(Bitboard::notation_to_coords("b6"), (5, 1));
            }
            #[test]
            fn b7() {
                assert_eq!(Bitboard::notation_to_coords("b7"), (6, 1));
            }
            #[test]
            fn b8() {
                assert_eq!(Bitboard::notation_to_coords("b8"), (7, 1));
            }

            #[test]
            fn c1() {
                assert_eq!(Bitboard::notation_to_coords("c1"), (0, 2));
            }
            #[test]
            fn c2() {
                assert_eq!(Bitboard::notation_to_coords("c2"), (1, 2));
            }
            #[test]
            fn c3() {
                assert_eq!(Bitboard::notation_to_coords("c3"), (2, 2));
            }
            #[test]
            fn c4() {
                assert_eq!(Bitboard::notation_to_coords("c4"), (3, 2));
            }
            #[test]
            fn c5() {
                assert_eq!(Bitboard::notation_to_coords("c5"), (4, 2));
            }
            #[test]
            fn c6() {
                assert_eq!(Bitboard::notation_to_coords("c6"), (5, 2));
            }
            #[test]
            fn c7() {
                assert_eq!(Bitboard::notation_to_coords("c7"), (6, 2));
            }
            #[test]
            fn c8() {
                assert_eq!(Bitboard::notation_to_coords("c8"), (7, 2));
            }

            #[test]
            fn d1() {
                assert_eq!(Bitboard::notation_to_coords("d1"), (0, 3));
            }
            #[test]
            fn d2() {
                assert_eq!(Bitboard::notation_to_coords("d2"), (1, 3));
            }
            #[test]
            fn d3() {
                assert_eq!(Bitboard::notation_to_coords("d3"), (2, 3));
            }
            #[test]
            fn d4() {
                assert_eq!(Bitboard::notation_to_coords("d4"), (3, 3));
            }
            #[test]
            fn d5() {
                assert_eq!(Bitboard::notation_to_coords("d5"), (4, 3));
            }
            #[test]
            fn d6() {
                assert_eq!(Bitboard::notation_to_coords("d6"), (5, 3));
            }
            #[test]
            fn d7() {
                assert_eq!(Bitboard::notation_to_coords("d7"), (6, 3));
            }
            #[test]
            fn d8() {
                assert_eq!(Bitboard::notation_to_coords("d8"), (7, 3));
            }

            #[test]
            fn e1() {
                assert_eq!(Bitboard::notation_to_coords("e1"), (0, 4));
            }
            #[test]
            fn e2() {
                assert_eq!(Bitboard::notation_to_coords("e2"), (1, 4));
            }
            #[test]
            fn e3() {
                assert_eq!(Bitboard::notation_to_coords("e3"), (2, 4));
            }
            #[test]
            fn e4() {
                assert_eq!(Bitboard::notation_to_coords("e4"), (3, 4));
            }
            #[test]
            fn e5() {
                assert_eq!(Bitboard::notation_to_coords("e5"), (4, 4));
            }
            #[test]
            fn e6() {
                assert_eq!(Bitboard::notation_to_coords("e6"), (5, 4));
            }
            #[test]
            fn e7() {
                assert_eq!(Bitboard::notation_to_coords("e7"), (6, 4));
            }
            #[test]
            fn e8() {
                assert_eq!(Bitboard::notation_to_coords("e8"), (7, 4));
            }

            #[test]
            fn f1() {
                assert_eq!(Bitboard::notation_to_coords("f1"), (0, 5));
            }
            #[test]
            fn f2() {
                assert_eq!(Bitboard::notation_to_coords("f2"), (1, 5));
            }
            #[test]
            fn f3() {
                assert_eq!(Bitboard::notation_to_coords("f3"), (2, 5));
            }
            #[test]
            fn f4() {
                assert_eq!(Bitboard::notation_to_coords("f4"), (3, 5));
            }
            #[test]
            fn f5() {
                assert_eq!(Bitboard::notation_to_coords("f5"), (4, 5));
            }
            #[test]
            fn f6() {
                assert_eq!(Bitboard::notation_to_coords("f6"), (5, 5));
            }
            #[test]
            fn f7() {
                assert_eq!(Bitboard::notation_to_coords("f7"), (6, 5));
            }
            #[test]
            fn f8() {
                assert_eq!(Bitboard::notation_to_coords("f8"), (7, 5));
            }

            #[test]
            fn g1() {
                assert_eq!(Bitboard::notation_to_coords("g1"), (0, 6));
            }
            #[test]
            fn g2() {
                assert_eq!(Bitboard::notation_to_coords("g2"), (1, 6));
            }
            #[test]
            fn g3() {
                assert_eq!(Bitboard::notation_to_coords("g3"), (2, 6));
            }
            #[test]
            fn g4() {
                assert_eq!(Bitboard::notation_to_coords("g4"), (3, 6));
            }
            #[test]
            fn g5() {
                assert_eq!(Bitboard::notation_to_coords("g5"), (4, 6));
            }
            #[test]
            fn g6() {
                assert_eq!(Bitboard::notation_to_coords("g6"), (5, 6));
            }
            #[test]
            fn g7() {
                assert_eq!(Bitboard::notation_to_coords("g7"), (6, 6));
            }
            #[test]
            fn g8() {
                assert_eq!(Bitboard::notation_to_coords("g8"), (7, 6));
            }

            #[test]
            fn h1() {
                assert_eq!(Bitboard::notation_to_coords("h1"), (0, 7));
            }
            #[test]
            fn h2() {
                assert_eq!(Bitboard::notation_to_coords("h2"), (1, 7));
            }
            #[test]
            fn h3() {
                assert_eq!(Bitboard::notation_to_coords("h3"), (2, 7));
            }
            #[test]
            fn h4() {
                assert_eq!(Bitboard::notation_to_coords("h4"), (3, 7));
            }
            #[test]
            fn h5() {
                assert_eq!(Bitboard::notation_to_coords("h5"), (4, 7));
            }
            #[test]
            fn h6() {
                assert_eq!(Bitboard::notation_to_coords("h6"), (5, 7));
            }
            #[test]
            fn h7() {
                assert_eq!(Bitboard::notation_to_coords("h7"), (6, 7));
            }
            #[test]
            fn h8() {
                assert_eq!(Bitboard::notation_to_coords("h8"), (7, 7));
            }
        }
    }

    mod inverses {
        use super::*;

        #[quickcheck]
        fn coords_to_index_to_coords(rank_i: usize, file_i: usize) -> bool {
            let (rank, file) = (rank_i % 8, file_i % 8);

            Bitboard::index_to_coords(Bitboard::coords_to_index(rank, file)) == (rank, file)
        }

        #[quickcheck]
        fn coords_to_notation_to_coords(rank_i: usize, file_i: usize) -> bool {
            let (rank, file) = (rank_i % 8, file_i % 8);

            Bitboard::notation_to_coords(Bitboard::coords_to_notation(rank, file)) == (rank, file)
        }
    }
}
