use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use tracing::{debug, instrument};
use crate::constants::Piece;

use super::*;

impl Wizard {
    #[instrument(skip(self))]
    pub fn initialize(&mut self) {
        debug!("Initializing wizard");
        self.collisions = 0;
        for e in self.table.iter_mut() {
            *e = None;
        }

        self.initialize_piece(Piece::Bishop);
        self.initialize_piece(Piece::Rook);

        debug!("Wizard initialized with {} collisions", self.collisions)
    }
    
    #[instrument(skip(self))]
    pub fn initialize_piece(&mut self, piece: Piece) {
        debug!("Initializing table for {:?}", piece);
        let (iters, shift_min) = match piece {
            Piece::Bishop => (self.bishops.iter_mut(), BISHOP_INDEX_MINS),
            Piece::Rook   => (self.rooks.iter_mut(), ROOK_INDEX_MINS),
            _ => panic!("Do not call this with anything other than bishop or rook, this should be an error type")
        };
        
        let images : Vec<Vec<(usize, Bitboard)>> = iters.enumerate().par_bridge().map(|(i, spell)| {
            spell.initialize(shift_min[i]);
            // NOTE: have to match twice since we can't put two closures with the same signature into the same variable for some reason.
            let attacks = match piece {
                Piece::Bishop => bishop_block_and_attack_board_for(i, NOMINAL_BISHOP_ATTACKS[i]),
                Piece::Rook   => rook_block_and_attack_board_for(i, NOMINAL_ROOK_ATTACKS[i]),
                _ => panic!("Do not call this with anything other than bishop or rook, this should be an error type")
            };
            
            let mut image = vec![];
            attacks.par_iter().map(|(blockers, attacks)| (spell.key_for(*blockers), *attacks)).collect_into_vec(&mut image);
            image
        }).collect();
        
        for image in images {
            for (idx, attacks) in image {
                match self.table[idx] {
                    Some(attack_board) => if attack_board != attacks { self.collisions += 1; }
                    None => { self.table[idx] = Some(attacks); }
                }
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
}