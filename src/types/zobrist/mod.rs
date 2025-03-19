use std::fmt::Debug;

use crate::types::Color;
use crate::{notation::Square, query, types::{Occupant, Piece}, Alteration, Query};
use crate::interface::Alter;

pub struct ZobristTable<const SEED: u64>;


// TODO: Calculate this from the types, so when I add fairy pieces it should Just Work(tm)
// The math is:
//
//  64 squares
// * 6 piece types
// * 2 colors
// + 1 flag for when it's black's turn
const ZOBRIST_TABLE_SIZE : usize = 1 + (2 * 6 * 64);
const ZOBRIST_SEED : u64 = 0x10062021_18092010 ^ 0x01081987_19051987;

pub type HazelZobrist = ZobristTable<ZOBRIST_SEED>;

impl std::ops::BitXor<Self> for Zobrist {
    type Output = Zobrist;

    fn bitxor(self, other: Zobrist) -> Self::Output {
        Zobrist(self.0 ^ other.0)
    }
}

impl<const SEED: u64> ZobristTable<SEED> {

    pub const TABLE : [u64; ZOBRIST_TABLE_SIZE] = {
        // seed the table
        let mut idx = 0;
        let mut table = [0; ZOBRIST_TABLE_SIZE];

        while idx < (ZOBRIST_TABLE_SIZE - 1) {
            // It doesn't matter what these are, they're random, we just need both functions to map
            // down to the idx.
            table[idx] = Self::random(idx as u64);
            idx += 1;
        }
        table[ZOBRIST_TABLE_SIZE - 1] = Self::random(ZOBRIST_TABLE_SIZE as u64);
        table

    };

    pub const fn random(depth: u64) -> u64 {
        // adapted from https://en.wikipedia.org/wiki/Xorshift
        let mut x = SEED;
        let mut times = 0; // this ensures we always do at least some number of iterations and do not return the seed at depth 0.
        while times < (depth + 16) {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            times += 1;
        }
        x
    }

    // TODO: This comes up enough (or the equivalent (sq, occ)) that it probably should be it's own
    // type ("OccupiedSquare"?)
    pub const fn depth_for(sq: Square, color: Color, piece: Piece) -> u64 {
        12 * (sq.index() as u64) + (6 * color as u64 + piece as u64)
    }


    /// A slow implementation of OccupiedSquare -> 0..ZOBRIST_TABLE_SIZE
    pub const fn slow_zobrist_mask_for(sq: Square, color: Color, piece: Piece) -> u64 {
        Self::random(Self::depth_for(sq, color, piece))
    }

    /// A fast implementation based on the cache
    pub const fn zobrist_mask_for(sq: Square, color: Color, piece: Piece) -> u64 {
        let idx = Self::depth_for(sq, color, piece) as usize;
        Self::TABLE[idx]
    }

    /// A convenience function.
    pub const fn side_to_move_mask() -> u64 {
        Self::TABLE[ZOBRIST_TABLE_SIZE - 1]
    }
}


// this can be calculated on any `query`able, I think.
#[derive(Eq, Hash, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Zobrist(u64);

impl Default for Zobrist {
    fn default() -> Self {
        Zobrist::empty()
    }
}

impl Debug for Zobrist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Z|{:>#018X}|", self.0)
    }
}


// this should probably be killed
impl From<&[Alteration]> for Zobrist {
    fn from(alterations: &[Alteration]) -> Zobrist {
        *Zobrist::empty().update(alterations)
    }
}

impl From<u64> for Zobrist {
    fn from(v: u64) -> Zobrist {
        Zobrist(v)
    }
}


impl Alter for Zobrist {
    fn alter(&self, alteration: Alteration) -> Self {
        let mut ret = *self;
        *ret.alter_mut(alteration)
    }

    fn alter_mut(&mut self, alteration: Alteration) -> &mut Self {
        let delta = match alteration {
            Alteration::Place { square: sq, occupant: Occupant::Occupied(piece, color) } => {
                HazelZobrist::zobrist_mask_for(sq, color, piece)
            },
            Alteration::Remove { square: sq, occupant: Occupant::Occupied(piece, color) } => {
                HazelZobrist::zobrist_mask_for(sq, color, piece)
            },
            Alteration::Turn => {
                HazelZobrist::side_to_move_mask()
            },
            // Alteration::Assert(metadata) => {
            //     if metadata.side_to_move.is_black() {
            //         // do nothing
            //         0
            //     } else {
            //         // undo the startturn's black-to-move masking
            //         HazelZobrist::black_to_move_mask()
            //     }
            // },
            _ => { 0 }
        };
        self.0 ^= delta;
        self
    }
}

// pub fn zobrist_for(q: &impl Query) -> Zobrist {

// }


impl Zobrist {
    #[cfg(test)]
    pub fn inner(&self) -> u64 { self.0 }

    pub fn empty() -> Zobrist {
        Zobrist(0)
    }

    pub fn new(query: &impl Query) -> Zobrist {
        let alterations : Vec<Alteration> = query::to_alterations(query).collect();
        Zobrist::from(alterations.as_slice())
    }

    // Deprecate.
    pub fn update(&mut self, alterations: &[Alteration]) -> &mut Self {
        for alter in alterations {
            self.alter_mut(*alter);
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use crate::coup::rep::{Move, MoveType};
    use crate::notation::ben::BEN;
    use crate::game::chess::position::Position;

    use super::*;

    mod zobrist_table {
        use super::*;

        #[test]
        fn zobrist_table_is_not_all_zeros() {
            let table = HazelZobrist::TABLE;
            for e in table {
                assert_ne!(e, 0);
            }
        }

        #[test]
        fn zobrist_table_is_all_distinct() {
            let table = HazelZobrist::TABLE;
            for i in 0..ZOBRIST_TABLE_SIZE {
                for j in 0..i {
                    if table[i] == table[j] {
                        panic!("TABLE[{}] == TABLE[{}]` == {}", i, j, table[i]);
                    }
                }
            }
        }

        #[quickcheck]
        fn depth_for_works(sq: Square, color: Color, piece: Piece) -> bool {
            let depth = HazelZobrist::depth_for(sq, color, piece) as usize;
            // NOTE: Depth only maps from [0, (ZTS-1)], the `ZTS`th spot is for the side-to-move
            // mask
            depth < (ZOBRIST_TABLE_SIZE - 1)
        }

        #[quickcheck]
        fn slow_mask_is_the_same_as_fast_mask(sq: Square, color: Color, piece: Piece) -> bool {
            let slow = HazelZobrist::slow_zobrist_mask_for(sq, color, piece);
            let fast = HazelZobrist::zobrist_mask_for(sq, color, piece);
            if slow != fast {
                dbg!(slow, fast);
                false
            } else { true }
        }
    }

    mod zobrist {
        use quickcheck::{Arbitrary, Gen};

        use crate::{alteration::MetadataAssertion, game::position_metadata::PositionMetadata, types::{color::COLORS, piece::PIECES}};
        use crate::notation::*;

        use super::*;

        #[test]
        #[tracing_test::traced_test]
        fn zobrist_is_nonzero() {
            let p = Position::new(BEN::start_position());
            assert_ne!(p.zobrist(), Zobrist::empty());
        }

        // #[test]
        // fn depth_for_covers_expected_range() {
        //     let mut depths = vec![];
        //     let mut idx = 0;
        //     for sq in Square::by_rank_and_file() {
        //         for color in COLORS {
        //             for piece in PIECES {
        //                 let depth : usize = HazelZobrist::depth_for(sq, color, piece) as usize;
        //                 assert_eq!(idx, depth);
        //                 idx += 1;
        //                 depths.push(depth);
        //             }
        //         }
        //     }


        //     use itertools::Itertools;
        //     for (key, group) in &depths.into_iter().chunk_by(|e| *e) {
        //         assert!(false); // I left this half-complete and can't remember what I intended to
        //         // test here.
        //     }
        // }

        #[test]
        fn zobrist_is_different_after_a_move_is_made() {
            let p1 = Position::new(BEN::start_position());
            let p2 = Position::with_moves(BEN::start_position(), vec![Move::new(D2, D4, MoveType::QUIET)]);
            assert_ne!(p1.zobrist(), p2.zobrist());
        }

        #[test]
        fn zobrist_is_same_for_same_position() {
            let p1 = Position::new(BEN::start_position());
            let p2 = Position::new(BEN::start_position());

            assert_eq!(p1.zobrist(), p2.zobrist());
        }

        #[test]
        #[tracing_test::traced_test]
        fn zobrist_is_same_for_transposition() {
            let variation_1 = vec![
                Move::new(D2, D4, MoveType::QUIET),
                Move::new(D7, D5, MoveType::QUIET),
                Move::new(C1, F4, MoveType::QUIET),
                Move::new(G8, F6, MoveType::QUIET),
            ];
            let variation_2 = vec![
                Move::new(D2, D4, MoveType::QUIET),
                Move::new(G8, F6, MoveType::QUIET),
                Move::new(C1, F4, MoveType::QUIET),
                Move::new(D7, D5, MoveType::QUIET),
            ];
            let p1 = Position::with_moves(BEN::start_position(), variation_1);
            let p2 = Position::with_moves(BEN::start_position(), variation_2);

            assert_eq!(p1.zobrist(), p2.zobrist());
        }


        impl Arbitrary for Alteration {
            fn arbitrary(g: &mut Gen) -> Alteration {
                let variant : usize = usize::arbitrary(g) % 5;

                let occupant = Occupant::Occupied(Piece::arbitrary(g), Color::arbitrary(g));

                match variant {
                    0 => Self::Place { square: Square::arbitrary(g), occupant },
                    1 => Self::Remove { square: Square::arbitrary(g), occupant },
                    2 => Self::Assert(PositionMetadata::arbitrary(g)),
                    3 => Self::Clear,
                    4 => Self::Lit(u8::arbitrary(g)),
                    _ => { unreachable!(); }
                }
            }
        }

        #[quickcheck]
        fn zobrist_update_is_idempotent(alteration: Alteration, alteration2: Alteration) -> bool {
            // this is mostly to allow the non-zero checking later, which makes sure we are
            // primarily testing interesting cases/disallow a potential null solution state
            if alteration == Alteration::Clear || alteration2 == Alteration::Clear { return true; }
            let mut z1 = Zobrist::empty();
            let mut z2 = Zobrist::empty();
            z1.update(&[alteration2]);
            z2.update(&[alteration2]);

            if z1 == Zobrist::empty() { return true; }

            z1.update(&[alteration, alteration]);

            if z1 != Zobrist::empty() && z2 != Zobrist::empty() && z1 == z2 {
                true
            } else {
                dbg!(z1, z2);
                false
            }

        }
    }
}
