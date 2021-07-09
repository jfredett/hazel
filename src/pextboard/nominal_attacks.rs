use super::*; 
use crate::constants::*;

lazy_static! {
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
        out
    };
    
    /// A lookup table to conver a bishop on an index -> it's unblocked attack squares, needed for magics
    pub static ref NOMINAL_BISHOP_ATTACKS : [Bitboard; 64] = {
        let mut out = [Bitboard::empty(); 64];
        for rank in 0..8 {
            for file in 0..8 {
                let idx = rank * 8 + file;
                let bishop = Bitboard::from(1 << idx);
                let mut attacks = bishop;
                for d in [Direction::NW, Direction::NE, Direction::SW, Direction::SE] {
                    let mut bb = bishop;
                    for _ in 0..8 {
                        bb |= bb.shift(d);
                    }
                    attacks |= bb;
                }
                out[idx] = attacks & !*EDGES & !bishop;
            }
        }
        out
    };
}

#[cfg(test)]
mod test {
    use crate::bitboard;

    use super::*;
    
    mod bishops {

        use crate::pextboard::slow_bishop_attacks;

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
        use crate::{pextboard::slow_rook_attacks, ply::Ply};
        
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