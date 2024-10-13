use crate::types::Bitboard;
use crate::types::Direction;


lazy_static! {
    /// A lookup table to convert a knight on a given index -> it's bitboard of moves
    pub static ref KNIGHT_MOVES: [Bitboard; 64] = {
        let mut out : [Bitboard; 64] = [Bitboard::empty(); 64];
        for i in 0..64 {
                let mut bb = Bitboard::empty();
                bb.set_by_index(i);

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

    /// A lookup table to convert a pawn on a given index -> it's bitboard of moves
    pub static ref PAWN_MOVES: [[Bitboard; 64]; 2] = {
        let mut white_out = [Bitboard::empty(); 64];
        let mut black_out = [Bitboard::empty(); 64];
        // pawn moves, initial rank
        for i in 8..17 {
            let mut wbb = Bitboard::empty();
            wbb.set_by_index(i);
            let mut bbb = Bitboard::empty();
            bbb.set_by_index(64-i);

            wbb |= wbb.shift(Direction::N)
                |  wbb.shift(Direction::N).shift(Direction::N)
                |  wbb.shift(Direction::NE)
                |  wbb.shift(Direction::NW);

            bbb |= bbb.shift(Direction::S)
                |  bbb.shift(Direction::S).shift(Direction::S)
                |  bbb.shift(Direction::SE)
                |  bbb.shift(Direction::SW);


            white_out[i] = wbb;
            black_out[64-i] = bbb;
        }

        // all other pawn moves
        for i in 17..64 {
            let mut wbb = Bitboard::empty();
            wbb.set_by_index(i);
            let mut bbb = Bitboard::empty();
            bbb.set_by_index(64-i);

            wbb |= wbb.shift(Direction::N)
                |  wbb.shift(Direction::NE)
                |  wbb.shift(Direction::NW);

            bbb |= bbb.shift(Direction::S)
                |  bbb.shift(Direction::SE)
                |  bbb.shift(Direction::SW);

            white_out[i] = wbb;
            black_out[64-i] = bbb;
        }


        [ white_out, black_out ]
    };

}
