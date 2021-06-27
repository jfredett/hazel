use std::mem::{self, MaybeUninit};

use super::*;
use crate::{bitboard::Bitboard, constants::magic::Magic};




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
    

    /// A lookup table to convert a rook on an index -> it's unblocked attack squares, needed for magics
    pub static ref NOMINAL_ROOK_ATTACKS : [Bitboard; 64] = {
        let mut out = [Bitboard::empty(); 64];
        for rank in 0..8 {
            for file in 0..8 {
                let idx = rank * 8 + file;
                let mut mask = !*EDGES;
                
                if rank == 0 { mask |= !*RANK_8; }
                if rank == 7 { mask |= !*RANK_1; }
                if file == 0 { mask |= !*H_FILE; }
                if file == 7 { mask |= !*A_FILE; }

                out[idx] = RANK_MASKS[rank] | FILE_MASKS[file];
                out[idx] &= mask;
                out[idx] &= !*CORNERS;
                out[idx] &= !Bitboard::from(1 << idx);
            }
        }
        return out
    };
    
    /// A lookup table to conver a bishop on an index -> it's unblocked attack squares, needed for magics
    pub static ref NOMINAL_BISHOP_ATTACKS : [Bitboard; 64] = {
        let mut out = [Bitboard::empty(); 64];
        for rank in 0..8 {
            for file in 0..8 {
                let idx = rank * 8 + file;
                let bishop = Bitboard::from(1 << idx);
                let mut attacks = bishop.clone();
                for d in [Direction::NW, Direction::NE, Direction::SW, Direction::SE] {
                    let mut bb = bishop.clone();
                    for _ in 0..8 {
                        bb |= bb.shift(d);
                    }
                    attacks |= bb;
                }
                out[idx] = attacks & !*EDGES & !bishop;
            }
        }
        return out
    };
    
    pub static ref ROOK_ATTACKS : [Magic; 64] = {
        // NOTE: This is unsafe because rust is _very_ weird about array initialization. There should be 
        // some way to work with an uninitialized array safely, and have the final block 'check' to make 
        // sure it's fully initialized at the end. This is a non-safe way of just doing that, it's unfortunate
        // you can't reference the index in the array initialization syntax.
        unsafe {
            let mut out: [MaybeUninit<Magic>; 64] = MaybeUninit::uninit().assume_init();
            let mut i = 0;
            for e in &mut out {
                *e = MaybeUninit::new(Magic::new_rook(i));
                i += 1;
            }
            
            mem::transmute::<_, [Magic; 64]>(out)
        }
    };
}

#[cfg(test)]
mod test {
    use crate::bitboard;

    use super::*;
    
    mod bishops {

        use super::*; 

        #[test]
        fn nominal_bishop_attacks_calculate_correctly() {
            // stick a bishop on d4, it should be on the a1-h8 diag and the a8-h1 diag
            let expected : Bitboard = (*A1_H8_DIAG | *A7_G1_DIAG) & !*EDGES & !bitboard!("d4");
            // d4 is 0o33
            assert_eq!(NOMINAL_BISHOP_ATTACKS[0o33], expected);
        }

        #[test]
        fn nominal_bishop_attacks_calculate_correctly_when_on_edge() {
            // stick a bishop on d4, it should be on the a1-h8 diag and the a8-h1 diag
            let expected : Bitboard = (*A1_H8_DIAG) & !*EDGES & !bitboard!("a1");
            assert_eq!(NOMINAL_BISHOP_ATTACKS[0o00], expected);
            assert!(!NOMINAL_BISHOP_ATTACKS[0o00].is_index_set(0o00));
            assert!(!NOMINAL_BISHOP_ATTACKS[0o00].is_index_set(0o07));
        }
    }
    
    mod rooks {
        use super::*; 
        use crate::constants::magic::slow_rook_attacks;

        
        #[quickcheck]
        fn rook_magic_attacks_calculate_attacks_correctly(rook_in: u64, occupancy:Bitboard) -> bool {
            let rook_idx = rook_in % 64;
            let rook_pos = Bitboard::from(1 << rook_idx);
            
            ROOK_ATTACKS[rook_idx as usize].attacks_for(occupancy) == slow_rook_attacks(rook_pos, occupancy)
        }

        #[test]
        fn nominal_rook_attacks_calculate_correctly_in_middle_of_board() {
            // stick a rook on d4, it should see...
            let expected : Bitboard = (*D_FILE | *RANK_4) & !*EDGES & !bitboard!("d4");
            // d4 is 0o33
            assert_eq!(NOMINAL_ROOK_ATTACKS[0o33], expected);
            assert!(!NOMINAL_ROOK_ATTACKS[0o33].is_index_set(0o37));
            assert!(!NOMINAL_ROOK_ATTACKS[0o33].is_index_set(0o73));
            assert!(!NOMINAL_ROOK_ATTACKS[0o33].is_index_set(0o30));
            assert!(!NOMINAL_ROOK_ATTACKS[0o33].is_index_set(0o03));
        }
        
        #[test]
        fn nominal_rook_attacks_calculate_correctly_on_corner_of_board() {
            // stick a rook on d4, it should see...
            let expected : Bitboard = (*A_FILE | *RANK_1) & !*CORNERS & !bitboard!("a1");
            assert_eq!(NOMINAL_ROOK_ATTACKS[0o00], expected);
            assert!(!NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o00));
            assert!(!NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o07));
            assert!(!NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o70));

            assert!(NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o01));
            assert!(NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o02));
            assert!(NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o03));
            assert!(NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o04));
            assert!(NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o05));
            assert!(NOMINAL_ROOK_ATTACKS[0o00].is_index_set(0o06));

        }

        #[test]
        fn nominal_rook_attacks_calculate_correctly_on_first_rank_noncorner() {
            // stick a rook on d4, it should see...
            let expected : Bitboard = (*D_FILE | *RANK_1) & !*CORNERS & !*RANK_8 & !bitboard!("d1");
            assert_eq!(NOMINAL_ROOK_ATTACKS[0o03], expected);

            assert!(!NOMINAL_ROOK_ATTACKS[0o03].is_index_set(0o00));
            assert!(NOMINAL_ROOK_ATTACKS[0o03].is_index_set(0o01));
            assert!(NOMINAL_ROOK_ATTACKS[0o03].is_index_set(0o02));
            assert!(NOMINAL_ROOK_ATTACKS[0o03].is_index_set(0o04));
            assert!(NOMINAL_ROOK_ATTACKS[0o03].is_index_set(0o05));
            assert!(NOMINAL_ROOK_ATTACKS[0o03].is_index_set(0o06));
            assert!(!NOMINAL_ROOK_ATTACKS[0o03].is_index_set(0o07));
        }
    }
}