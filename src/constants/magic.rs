use crate::bitboard::Bitboard;
use crate::constants::{
    Direction,   
    move_tables::NOMINAL_ROOK_ATTACKS,
};



#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Magic {
    mask : Bitboard,
    attacks: Box<[Option<Bitboard>; 4096]>,
    magic: u64,
    shift: u64
}

impl Magic {
    pub fn empty() -> Magic {
        Magic {
            mask: Bitboard::empty(),
            attacks: box [None; 4096],
            magic: 0,
            shift: 0
        }
    }

    pub fn new_rook(idx: usize) -> Magic {
        loop {
            let mask = NOMINAL_ROOK_ATTACKS[idx];
            let shift = 64 - 12;
            let magic = Magic::low_bit_random_u64();
            let mut bbs = box [None; 4096];
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
    
    fn low_bit_random_u64() -> u64 {
        rand::random::<u64>() & rand::random::<u64>() & rand::random::<u64>()
    }
    
    pub fn count_entries(&self) -> usize {
        let mut count = 0;
        for e in self.attacks.iter() {
            if let Some(_) = e { count += 1; }
        }
        return count;
    }
    
    #[inline(always)]
    fn key_for(&self, board: Bitboard) -> usize {
       (((board & self.mask) * self.magic) >> self.shift).into()
    }
    
    pub fn attacks_for(&self, blockers: Bitboard) -> Bitboard {
        let key = self.key_for(blockers);
        if let Some(a) = self.attacks[key] {
            return a;
        } else {
            panic!("Failed lookup for Magic #: #{:?}", self.magic);
        }
    } 
    
    
}

fn rook_block_and_attack_board_for(sq: usize, mask: Bitboard) -> Vec<(Bitboard, Bitboard)> {
    block_and_attack_board_for(sq, mask, slow_rook_attacks)
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
fn select_subset<T>(selection: u64, vector: &Vec<T>) -> Vec<T> 
   where T : Copy {
   let mut out = vec![];
   for i in 0..64 {
       if selection & (1 << i) > 0 {
           out.push(vector[i]) 
       }
   }
   out
}

pub fn slow_rook_attacks(rook_pos: Bitboard, occupancy: Bitboard) -> Bitboard {
    let rook_idx = rook_pos.all_set_indices()[0];
    let rook_rank = rook_idx % 8;
    let rook_file = rook_idx / 8;
    
    let mut squares = vec![];
    
    for i in 1..=(8-rook_rank) {
        let try_move = rook_pos.shift_by(Direction::N, i);
        if try_move.is_empty() { break; }
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=rook_rank {
        let try_move = rook_pos.shift_by(Direction::S, i);
        if try_move.is_empty() { break; }
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=(8-rook_file) {
        let try_move = rook_pos.shift_by(Direction::W, i);
        if try_move.is_empty() { break; }
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0])
        }
    }

    for i in 1..=rook_file {
        let try_move = rook_pos.shift_by(Direction::E, i);
        if try_move.is_empty() { break; }
        if !(try_move & occupancy).is_empty() {
            squares.push(try_move.all_set_indices()[0]);
            break;
        } else {
            squares.push(try_move.all_set_indices()[0]);
        }
    }
    
    let mut out = Bitboard::empty();
    for s in squares {
        out.set_by_index(s);
    }

    return out;
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
    fn magics_are_equivalent_to_slow_attack(rook_in: u64, occupancy: Bitboard) -> bool {
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

}
