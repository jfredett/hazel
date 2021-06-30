use crate::{bitboard::Bitboard, constants::Direction, util::select_subset};

pub(super) fn rook_block_and_attack_board_for(sq: usize, mask: Bitboard) -> Vec<(Bitboard, Bitboard)> {
    block_and_attack_board_for(sq, mask, slow_rook_attacks)
}

pub(super) fn bishop_block_and_attack_board_for(sq: usize, mask: Bitboard) -> Vec<(Bitboard, Bitboard)> {
    block_and_attack_board_for(sq, mask, slow_bishop_attacks)
}

pub(super) fn block_and_attack_board_for<F>(sq: usize, mask: Bitboard, attack_fn: F) -> Vec<(Bitboard, Bitboard)> 
    where F : Fn(Bitboard, Bitboard) -> Bitboard {
    let pos = Bitboard::from(1 << sq);
    let blocker_indexes = mask.all_set_indices();
    let mask_count = blocker_indexes.len();
    let mut out = vec![];
    
    for i in 0..2u64.pow(mask_count as u32) {
        let mut occupancy_board = Bitboard::empty();
        for idx in select_subset(i, &blocker_indexes) {
            occupancy_board.set_by_index(idx);            
        }
        let attack_board = attack_fn(pos, occupancy_board);
        out.push((occupancy_board, attack_board));

    }
    out
}

pub fn slow_attacks(pos: Bitboard, occupancy: Bitboard, dirs: [Direction; 4]) -> Bitboard {
    let mut out = Bitboard::empty();

    for dir in dirs {
        'next_dir: for i in 1..8 {
            let try_move = pos.shift_by(dir, i);
            if try_move.is_empty() { break 'next_dir; }
            out.set_by_index(try_move.all_set_indices()[0]);
            if !(try_move & occupancy).is_empty() { break 'next_dir; }
        }
    }
    
    return out;
}

pub fn slow_bishop_attacks(bishop_pos: Bitboard, occupancy: Bitboard) -> Bitboard {
    slow_attacks(bishop_pos, occupancy, [Direction::NW, Direction::SW, Direction::NE, Direction::SE])
}

pub fn slow_rook_attacks(rook_pos: Bitboard, occupancy: Bitboard) -> Bitboard {
    slow_attacks(rook_pos, occupancy, [Direction::N, Direction::E, Direction::S, Direction::W] )
}

#[cfg(test)]
mod test {
    use crate::{bitboard, constants::Color, ply::Ply};

    use super::*;
    
    mod rooks {
        use crate::constants::POS2_KIWIPETE_FEN;

        use super::*;
        
        #[test]
        fn slow_rook_attacks_kiwipete_a1_position() {
            let ply = Ply::from_fen(&String::from(POS2_KIWIPETE_FEN));
            assert_eq!(
                slow_rook_attacks(bitboard!("a1"), ply.occupancy()),
                bitboard!("a2", "b1", "c1", "d1", "e1")
            );
        }

        #[test]
        fn correctly_calculates_rook_attacks_via_slow_method() {

            /* A board what looks like:
             * 
             * 8 k . . . . . . .
             * 7 . . . . . . . .
             * 6 . . . . p . . .
             * 5 . . . . . . . .
             * 4 . p . . R . . .
             * 3 . . . . . . . .
             * 2 . . . . P . . .
             * 1 K . . . . . . .
             *   a b c d e f g h
             * 
             */
            let board = Ply::from_fen(&String::from("k7/8/4p3/8/1p2R3/8/4P3/K7 w - - 0 1"));
            
            // this is fine here since we know there is only 1 rook on the board, this'd bust if there were two.
            let rook_pos = board.rooks[Color::WHITE as usize];
            let expected = bitboard!("e2", "e3", "e5", "e6", "b4", "c4", "d4", "f4", "g4", "h4");
            
            let rook_attacks = slow_rook_attacks(rook_pos, board.occupancy());
            
            assert_eq!(rook_attacks, expected);
        }
        
        //#[quickcheck]
        //fn magic_rooks_are_equivalent_to_slow_attack(rook_in: u64, occupancy: Bitboard) -> bool {
        //    let rook_idx = rook_in % 64;
        //    let rook_pos = Bitboard::from(1 << rook_idx);
        //    let m = Magic::new_rook(rook_idx as usize);
        //    
        //    m.attacks_for(occupancy) == slow_rook_attacks(rook_pos, occupancy)
        //}

        /*
        #[test]
        fn rook_magic_correctly_calculates_rook_attacks_like_slow_method() {
            let board = Ply::from_fen(&String::from("k7/8/4p3/8/1p2R3/8/4P3/K7 w - - 0 1"));
            
            // this is fine here since we know there is only 1 rook on the board, this'd bust if there were two.
            let rook_pos = board.rooks[Color::WHITE as usize];
            let expected = bitboard!("e2", "e3", "e5", "e6", "b4", "c4", "d4", "f4", "g4", "h4");
            
            let m = Magic::new_rook(rook_pos.all_set_indices()[0]);
            
            let rook_attacks = m.attacks_for(board.occupancy() & !rook_pos);
            
            assert_eq!(rook_attacks, expected);
        }
        
        #[test]
        fn generating_all_rooks_magics_works() {
            dbg!("test");
            for i in 0..64 {
                dbg!(i);
                let m = Magic::new_rook(i);
                dbg!(m.magic);
            }
        }
        */
    }
    
    mod bishops {
        use super::*;

        #[test]
        fn correctly_calculates_bishop_attacks_via_slow_method() {

            /* A board what looks like:
             * 
             * 8 k . . . . . . .
             * 7 . . . . . . . .
             * 6 . . . . . . p .
             * 5 . . . . . . . .
             * 4 . p . . B . . .
             * 3 . . . . . P . .
             * 2 . . . . . . . .
             * 1 K . . . . . . .
             *   a b c d e f g h
             * 
             */
            let board = Ply::from_fen(&String::from("k7/8/6p1/8/1p2B3/5P2/8/K7 w - - 0 1"));
            
            // this is fine here since we know there is only 1 bishop on the board, this'd bust if there were two.
            let bishop_pos = board.bishops[Color::WHITE as usize];
            let expected = bitboard!("f3","f5","g6","d5","c6","b7","a8","d3","c2","b1");
            
            let bishop_attacks = slow_bishop_attacks(bishop_pos, board.occupancy());
            
            assert_eq!(bishop_attacks, expected);
        }

        //#[quickcheck]
        //fn magic_bishops_are_equivalent_to_slow_attack(bishop_in: u64, occupancy: Bitboard) -> bool {
        //    let bishop_idx = bishop_in % 64;
        //    let bishop_pos = Bitboard::from(1 << bishop_idx);
        //    let m = Magic::new_bishop(bishop_idx as usize);
        //    
        //    m.attacks_for(occupancy) == slow_bishop_attacks(bishop_pos, occupancy)
        //}
    }

}