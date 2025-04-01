use crate::bitboard::Bitboard;
use hazel_core::direction::Direction;
use hazel_core::square::*;

use lazy_static::lazy_static;

lazy_static! {
    /// A lookup table to convert a knight on a given index -> it's bitboard of moves
    // NOTE: 32k is all it takes to store a bitboard->bitboard map of all pairs and singles of
    // knights and their respective moves, probably worth doing to avoid the loop in
    // #*_knight_attacks in position
    pub static ref KNIGHT_MOVES: [Bitboard; 64] = {
        let mut out : [Bitboard; 64] = [Bitboard::empty(); 64];
        #[allow(clippy::needless_range_loop)]
        for i in 0..64 {
                let mut bb = Bitboard::empty();
                bb.set(Square::new(i));

                let position_board = bb.shift(Direction::N).shift(Direction::N).shift(Direction::E) // NNE
                                   | bb.shift(Direction::N).shift(Direction::N).shift(Direction::W) // NNW
                                   | bb.shift(Direction::W).shift(Direction::W).shift(Direction::N) // WWN
                                   | bb.shift(Direction::W).shift(Direction::W).shift(Direction::S) // WWS
                                   | bb.shift(Direction::S).shift(Direction::S).shift(Direction::W) // SSW
                                   | bb.shift(Direction::S).shift(Direction::S).shift(Direction::E) // SSE
                                   | bb.shift(Direction::E).shift(Direction::E).shift(Direction::S) // EES
                                   | bb.shift(Direction::E).shift(Direction::E).shift(Direction::N) // EEN
                                   ;
                out[i] = position_board;
        }
        out
    };

    // NOTE: BitOps aren't const yet, so this is as close as I could get
    pub static ref KING_ATTACKS : [Bitboard; 64] = {
        let mut arr = [Bitboard::empty(); 64];

        #[allow(clippy::needless_range_loop)]
        for idx in 0..64 {

            let mut bb = Bitboard::empty();
            let sq = Square::new(idx);
            bb.set(sq);

            bb = bb.shift(Direction::N) | bb.shift(Direction::NE) | bb.shift(Direction::E) | bb.shift(Direction::SE) |
                 bb.shift(Direction::S) | bb.shift(Direction::SW) | bb.shift(Direction::W) | bb.shift(Direction::NW);

            bb.unset(sq);

            arr[idx] = bb;
        }
        arr
    };
}
