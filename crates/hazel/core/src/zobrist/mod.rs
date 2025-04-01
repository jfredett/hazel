use std::fmt::Debug;

use quickcheck::Arbitrary;

use crate::color::Color;
use crate::file::File;
use crate::occupant::Occupant;
use crate::piece::Piece;
use crate::square::Square;
use crate::castle_rights::CastleRights;
use crate::interface::{Alter, Alteration, Query};

pub struct ZobristTable<const SEED: u64>;


// TODO: Calculate this from the types, so when I add fairy pieces it should Just Work(tm)
// The math is:
//
//  64 squares
// * 6 piece types
// * 2 colors
// + 1 flag for when it's black's turn
// + 8 for en_passant
// + 16 for each configuration of castling rights
pub const ZOBRIST_TABLE_SIZE : usize = 1 + (2 * 6 * 64) + 8 + 16;
const ZOBRIST_SEED : u64 = 0x10062021_18092010 ^ 0x01081987_19051987;
const STM_OFFSET : usize = 1; // last entry is the STM
const EP_OFFSET : usize = STM_OFFSET + 8; // preceded by the 8 EP entries.
const CASTLING_OFFSET : usize = EP_OFFSET + 16; // preceded by the 16 Castling entries.


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

    pub const fn castling_mask_for(rights: CastleRights) -> u64 {
        let mut idx = 0;

        if rights.white_long { idx += 8; }
        if rights.white_short { idx += 4; }
        if rights.black_long { idx += 2; }
        if rights.black_short { idx += 1; }

        Self::TABLE[ZOBRIST_TABLE_SIZE - CASTLING_OFFSET + idx]
    }

    pub const fn en_passant_mask_for(maybe_file: Option<File>) -> u64 {
        if let Some(file) = maybe_file {
            let idx = file as usize;
            Self::TABLE[ZOBRIST_TABLE_SIZE - EP_OFFSET + idx]
        } else {
            0
        }

    }
}


// this can be calculated on any `query`able, I think.
#[derive(Eq, Hash, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Zobrist(u64);

impl Arbitrary for Zobrist {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut z = Zobrist::empty();
        let repeats = u8::arbitrary(g);
        // It might be alright ot just use a random u64, but this feels better to me.
        for _ in 0..repeats {
            z.alter_mut(Alteration::arbitrary(g));
        }
        z
    }
}

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
        alterations.iter().fold(Zobrist::empty(), |acc, v| { acc.alter(*v) })
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
            // Alteration::Turn => {
            //     HazelZobrist::side_to_move_mask()
            // },
            Alteration::Assert(metadata) => {
                let stm = if metadata.side_to_move.is_black() { HazelZobrist::side_to_move_mask() } else { 0 };

                stm ^
                HazelZobrist::castling_mask_for(metadata.castling) ^
                HazelZobrist::en_passant_mask_for(metadata.en_passant)
            },
            Alteration::Inform(metadata) => {
                let stm = if metadata.side_to_move.is_black() { HazelZobrist::side_to_move_mask() } else { 0 };

                stm ^
                HazelZobrist::castling_mask_for(metadata.castling) ^
                HazelZobrist::en_passant_mask_for(metadata.en_passant)
            },
            _ => { 0 }
        };
        self.0 ^= delta;
        self
    }
}


impl Zobrist {
    #[cfg(test)]
    pub fn inner(&self) -> u64 { self.0 }

    pub fn empty() -> Zobrist {
        Zobrist(0)
    }

    pub fn new(query: &impl Query) -> Zobrist {
        let alterations : Vec<Alteration> = crate::interface::query::to_alterations(query).collect();
        Zobrist::from(alterations.as_slice())
    }
}


