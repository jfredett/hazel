use std::{collections::HashMap, sync::RwLock, fmt::Debug};

use crate::{game::position::Position, notation::Square, types::{Bitboard, Color, Occupant, Piece}, Alteration};
use crate::notation::*;

pub struct ZobristTable<const SEED: u64>;


// TODO: Calculate this from the types, so when I add fairy pieces it should Just Work(tm)
// The math is:
//
//  64 squares
// * 6 piece types
// * 2 colors
// + 1 flag for when it's black's turn
const ZOBRIST_SEED : u64 = 0x10062021_18092010 ^ 0x01081987_19051987;
const ZOBRIST_TABLE_SIZE : usize = 1 + (2 * 6 * 64);

type HazelZobrist = ZobristTable<ZOBRIST_SEED>;

impl<const SEED: u64> ZobristTable<SEED> {

    const TABLE : [u64; ZOBRIST_TABLE_SIZE] = {
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
        while times < (depth + 1) {
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
    pub const fn black_to_move_mask(&self) -> u64 {
        Self::TABLE[ZOBRIST_TABLE_SIZE - 1]
    }

    pub const fn table(&self) -> &'static [u64] {
        &Self::TABLE
    }

    pub fn initialize_at_rt() -> Self {
        // seed the table
        let mut idx = 0;
        while idx < ZOBRIST_TABLE_SIZE {
            dbg!("Here");
            // It doesn't matter what these are, they're random, we just need both functions to map
            // down to the idx.
            Self::TABLE[idx] = Self::random(idx as u64);
            idx += 1;
        }

        // Technically this could be anything, because these items exist at compile time.
        ZobristTable
    }
    pub const fn initialize() -> Self {
        ZobristTable
    }
}


// this can be calculated on any `query`able, I think.
#[derive(Eq, Hash, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Zobrist(u64);

pub(crate) const ZOBRIST_TABLE: ZobristTable<ZOBRIST_SEED> = ZobristTable::initialize();

impl Zobrist {
    pub fn new(position: &Position) -> Zobrist {
        let (_, _, alterations) = Position::calculate_boardstate(position);
        *Zobrist(0).update(&alterations)
    }

    pub fn empty() -> Zobrist {
        Zobrist(0)
    }

    pub fn update(&mut self, alterations: &[Alteration]) -> &mut Self {
        tracing::debug!("updating zobrist");

        for alter in alterations {


            let delta = match alter {
                Alteration::Place { square: sq, occupant: Occupant::Occupied(piece, color) } => { HazelZobrist::zobrist_mask_for(*sq, *color, *piece) } // xor in the occupant
                Alteration::Remove { square: sq, occupant: Occupant::Occupied(piece, color) } => { HazelZobrist::zobrist_mask_for(*sq, *color, *piece) } // xor in the occupant
                Alteration::Assert(metadata) => {
                    if metadata.side_to_move.is_black() {
                        ZOBRIST_TABLE.black_to_move_mask()
                    } else {
                        // do nothing
                        0
                    }
                }
                Alteration::Clear => { (*self) = Zobrist(0); continue; } // set to the zobrist of the empty board.
                _ => { continue } // nothing else matters, we don't care about metadata
            };

            tracing::debug!("\n\nalter is: <{:?}>\ndelta is: {:#04x},\nzob is {:#04x}", alter, delta, self.0);
            // doesn't account for side-to-move. Or any other metadata for that matter
            self.0 ^= delta;
            tracing::debug!("\nzob updates to: {:#04x}\n", self.0);
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
        fn zobrist_table_is_not_all_distinct() {
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
        fn random_returns_random_values_at_rt(depth1: u64, depth2: u64) -> bool {
            if depth1 == depth2 { return true; }
            if depth1 > 1024 || depth2 > 1024 { return true; }
            HazelZobrist::random(depth1) != HazelZobrist::random(depth2)
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
        use crate::types::{color::COLORS, piece::PIECES};

        use super::*;

        #[test]
        fn zobrist_is_nonzero() {
            let p = Position::new(BEN::start_position(), vec![]);
            assert_ne!(p.zobrist(), Zobrist::empty());
        }

        #[test]
        fn depth_for_covers_expected_range() {
            let mut depths = vec![];
            let mut idx = 0;
            for sq in Square::by_rank_and_file() {
                for color in COLORS {
                    for piece in PIECES {
                        let depth : usize = HazelZobrist::depth_for(sq, color, piece) as usize;
                        assert_eq!(idx, depth);
                        idx += 1;
                        tracing::debug!("{} = {}, {} = {}, {:?} = {}", sq, sq.index(), color, color as usize, piece, piece as usize);
                        depths.push(depth);
                    }
                }
            }


            use itertools::Itertools;
            for (key, group) in &depths.into_iter().chunk_by(|e| *e) {
                tracing::debug!("{key} occurs {} times", group.count());
            }
        }

        #[test]
        fn zobrist_is_different_after_a_move_is_made() {
            let p1 = Position::new(BEN::start_position(), vec![]);
            let p2 = Position::new(BEN::start_position(), vec![Move::new(D2, D4, MoveType::QUIET)]);
            assert_ne!(p1.zobrist(), p2.zobrist());
        }

        #[test]
        fn zobrist_is_same_for_same_position() {
            let p1 = Position::new(BEN::start_position(), vec![]);
            let p2 = Position::new(BEN::start_position(), vec![]);

            assert_eq!(p1.zobrist(), p2.zobrist());
        }

        #[test]
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
            let p1 = Position::new(BEN::start_position(), variation_1);
            let p2 = Position::new(BEN::start_position(), variation_2);

            assert_eq!(p1.zobrist(), p2.zobrist());
        }
    }
}
