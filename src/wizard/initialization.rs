use tracing::{debug, info, instrument, trace};
use crate::constants::Piece;

use super::*;

impl Wizard {
    pub fn initialize(&mut self) {
        self.collisions = 0;
        for e in self.table.iter_mut() {
            *e = None;
        }

        self.initialize_piece(Piece::Bishop);
        self.initialize_piece(Piece::Rook);
    }
    
    #[instrument(skip(self))]
    fn initialize_piece(&mut self, piece: Piece) {
        info!("Initializing table for {:?}", piece);
        let (iters, shift_min) = match piece {
            Piece::Bishop => (self.bishops.iter_mut(), BISHOP_INDEX_MINS),
            Piece::Rook   => (self.rooks.iter_mut(), ROOK_INDEX_MINS),
            _ => panic!("Do not call this with anything other than bishop or rook, this should be an error type")
        };
        
        for (i, spell) in iters.enumerate() {
            spell.initialize(shift_min[i]..MAX_SHIFT);
            
            // NOTE: have to match twice since we can't put two closures with the same signature into the same variable for some reason.
            let attacks = match piece {
                Piece::Bishop => bishop_block_and_attack_board_for(i, NOMINAL_BISHOP_ATTACKS[i]),
                Piece::Rook   => rook_block_and_attack_board_for(i, NOMINAL_ROOK_ATTACKS[i]),
                _ => panic!("Do not call this with anything other than bishop or rook, this should be an error type")
            };

            for (blockers, attacks) in attacks {
                let key = spell.key_for(blockers);
                
                match self.table[key] {
                    Some(attack_board) => { 
                        if attack_board != attacks {
                            self.collisions += 1;
                        }
                    },
                    None => { self.table[key] = Some(attacks); }
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use super::*;
    
    #[test]
    #[traced_test]
    fn an_initialized_wizard_has_slots_filled_in_their_table() {
        let mut w = Wizard::empty();
        w.initialize();
        for &e in w.table.iter() {
           if e.is_some() { return } 
        }
        panic!("No items set");
    }
    
    // NOTE: This may, technically speaking, fail. If we get _extremely_ lucky we might randomly
    // choose 128 random spells with exactly the right values to not collide. This is astronomically
    // unlikely, so it's no worries to have a potential flaky test here. Since when it doesn't flake
    // it proves that the collision counter is working.
    #[test]
    fn an_initialized_wizard_has_non_zero_collisions() {
        let mut w = Wizard::empty();
        w.initialize();
        assert!(w.collisions > 0);
    }

    #[test]
    fn quick_bish() {
        let mut count = 0;
        for sq in 0..64 {
            let mask = NOMINAL_BISHOP_ATTACKS[sq];
            for _ in bishop_block_and_attack_board_for(sq, mask) {
                count += 1;
            }
        }
        dbg!(count);
        assert!(false);
    }

    #[test]
    fn quick_rook() {
        let mut count = 0;
        for sq in 0..64 {
            let mask = NOMINAL_ROOK_ATTACKS[sq];
            for _ in rook_block_and_attack_board_for(sq, mask) {
                count += 1;
            }
        }
        dbg!(count);
        assert!(false);
    }
}