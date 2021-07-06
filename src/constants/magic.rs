use crate::{
    bitboard::Bitboard, constants::{
        Direction,   
        move_tables::{
            NOMINAL_ROOK_ATTACKS,
            NOMINAL_BISHOP_ATTACKS
        }
    }
};



#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Magic {
    mask : Bitboard,
    attacks: Box<[Option<Bitboard>; 8192]>,
    
    magic: u64,
    shift: u64
}

impl Magic {
    pub fn empty() -> Magic {
        Magic {
            mask: Bitboard::empty(),
            attacks: box [None; 8192],
            magic: 0,
            shift: 0
        }
    }

    pub fn new_rook(idx: usize) -> Magic {
        loop {
            let mask = NOMINAL_ROOK_ATTACKS[idx];
            let shift = 64 - 13;
            let magic = Magic::low_bit_random_u64();
            let mut bbs = box [None; 8192];
            let mut restart = false;
            
            for (blockers, attacks) in rook_block_and_attack_board_for(idx, mask) {
                let key: u64 = ((blockers * magic) >> shift).into();
                match bbs[key as usize] {
                    Some(attack_board) => { 
                        if attack_board != attacks {
                            restart = true;
                            break
                        }
                    }
                    None => {
                        bbs[key as usize] = Some(attacks);    
                    }
                }
            }
            if !restart {
                return Magic {
                    mask,
                    shift,
                    magic,
                    attacks: bbs
                }
            }
        }
    }
    
    pub fn new_bishop(idx: usize) -> Magic {
        loop {
            let mask = NOMINAL_BISHOP_ATTACKS[idx];
            let shift = 64 - 12;
            let magic = Magic::low_bit_random_u64();
            let mut bbs = box [None; 8192];
            let mut restart = false;
            
            for (blockers, attacks) in bishop_block_and_attack_board_for(idx, mask) {
                let key: u64 = ((blockers * magic) >> shift).into();
                match bbs[key as usize] {
                    Some(attack_board) => { 
                        if attack_board != attacks {
                            restart = true;
                            break
                        }
                    }
                    None => {
                        bbs[key as usize] = Some(attacks);    
                    }
                }
            }
            if !restart {
                return Magic {
                    mask,
                    shift,
                    magic,
                    attacks: bbs
                }
            }
        }
    }
    
    fn low_bit_random_u64() -> u64 {
        rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
    }
    
    pub fn count_entries(&self) -> usize {
        let mut count = 0;
        for e in self.attacks.iter() {
            if e.is_some() { count += 1; }
        }
        count
    }
    
    #[inline(always)]
    fn key_for(&self, board: Bitboard) -> usize {
       (((board & self.mask) * self.magic) >> self.shift).into()
    }
    
    pub fn attacks_for(&self, blockers: Bitboard) -> Bitboard {
        let key = self.key_for(blockers);
        if let Some(a) = self.attacks[key] {
            a
        } else {
            panic!("Failed lookup for Magic #: #{:?}", self.magic);
        }
    } 
}

fn rook_block_and_attack_board_for(sq: usize, mask: Bitboard) -> Vec<(Bitboard, Bitboard)> {
    block_and_attack_board_for(sq, mask, slow_rook_attacks)
}

fn bishop_block_and_attack_board_for(sq: usize, mask: Bitboard) -> Vec<(Bitboard, Bitboard)> {
    block_and_attack_board_for(sq, mask, slow_bishop_attacks)
}

fn block_and_attack_board_for<F>(sq: usize, mask: Bitboard, attack_fn: F) -> Vec<(Bitboard, Bitboard)> 
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

/// Selects a subset from a vector using the given `selection` bitset as a
/// selection mask -- if the `nth` bit is high, then the `nth` element will be
/// chosen
/// TODO: Send this to a util module
fn select_subset<T>(selection: u64, vector: &[T]) -> Vec<T> 
   where T : Copy {
   let mut out = vec![];
   for i in 0..64 {
       if selection & (1 << i) > 0 {
           out.push(vector[i]) 
       }
   }
   out
}

pub fn slow_bishop_attacks(bishop_pos: Bitboard, occupancy: Bitboard) -> Bitboard {
    let bishop_idx = bishop_pos.all_set_indices()[0];
    let bishop_rank = bishop_idx % 8;
    let bishop_file = bishop_idx / 8;

    let mut squares = vec![];

    for i in 1..=(8-bishop_rank) {
        let try_move = bishop_pos.shift_by(Direction::NW, i);
        if try_move.is_empty() { break; }
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=bishop_rank {
        let try_move = bishop_pos.shift_by(Direction::SW, i);
        if try_move.is_empty() { break; }
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=(8-bishop_file) {
        let try_move = bishop_pos.shift_by(Direction::NE, i);
        if try_move.is_empty() { break; }
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=bishop_file {
        let try_move = bishop_pos.shift_by(Direction::SE, i);
        if try_move.is_empty() { break; }
        squares.push(try_move.all_set_indices()[0]);
        if !(try_move & occupancy).is_empty() {
            break;
        } else {
            squares.push(try_move.all_set_indices()[0]);
        }
    }

    let mut out = Bitboard::empty();
    for s in squares {
        out.set_by_index(s);
    }

    out
}

pub fn slow_rook_attacks(rook_pos: Bitboard, occupancy: Bitboard) -> Bitboard {
    let mut squares = vec![];
    
    for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
        'next_dir: for i in 1..8 {
            let try_move = rook_pos.shift_by(dir, i);
            if try_move.is_empty() { break 'next_dir; }
            squares.push(try_move.all_set_indices()[0]);
            if !(try_move & occupancy).is_empty() { break 'next_dir; }
        }
    }
    
    let mut out = Bitboard::empty();
    for s in squares {
        out.set_by_index(s);
    }

    out
}


#[cfg(test)]
mod test {
    use crate::{bitboard, constants::Color, ply::Ply};

    use super::*;
    
    #[test]
    fn select_subset_test() {
        let v = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
        let sub = select_subset(0b0000111100001111, &v);
        assert_eq!(sub, vec![0,1,2,3,8,9,10,11]);
    }
    
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
        
        #[quickcheck]
        fn magic_rooks_are_equivalent_to_slow_attack(rook_in: u64, occupancy: Bitboard) -> bool {
            let rook_idx = rook_in % 64;
            let rook_pos = Bitboard::from(1 << rook_idx);
            let m = Magic::new_rook(rook_idx as usize);
            
            m.attacks_for(occupancy) == slow_rook_attacks(rook_pos, occupancy)
        }
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

        #[quickcheck]
        fn magic_bishops_are_equivalent_to_slow_attack(bishop_in: u64, occupancy: Bitboard) -> bool {
            let bishop_idx = bishop_in % 64;
            let bishop_pos = Bitboard::from(1 << bishop_idx);
            let m = Magic::new_bishop(bishop_idx as usize);
            
            m.attacks_for(occupancy) == slow_bishop_attacks(bishop_pos, occupancy)
        }
    }

}
