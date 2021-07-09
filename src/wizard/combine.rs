use rayon::iter::{IntoParallelRefMutIterator, ParallelBridge, ParallelIterator};
use tracing::trace;

use super::*;

impl Wizard {
    /// The function to be minimized
    pub fn fitness(&self) -> usize {
        (self.collisions * 1_000_000) + self.space_occupied() 
    } 
    
    pub fn mutate(&mut self) {
        // ~3% chance of mutation, this should be controllable
        for spell in self.rooks.iter_mut() {
            if rand::random::<u8>() % 32 == 0 {
                spell.mutate();        
            }
        }
        
        for spell in self.bishops.iter_mut() {
            if rand::random::<u8>() % 32 == 0 {
                spell.mutate();        
            }
        }
    }

    /// Takes two wizards and combines them into a new wizard.
    pub fn combine(&self, other: &Wizard) -> Wizard {
        trace!("Combining wizards");
        let mut new_wizard = Wizard::empty();

        trace!("Weaving wizard spellbooks");
        // create 2 64b numbers, if high, choose spell from left, if low, choose from right
        let bishop_selection = rand::random::<u64>();
        let rook_selection = rand::random::<u64>();
        for i in 0..64 {
            if bishop_selection & (1 << i) > 0 {
                new_wizard.bishops[i] = self.bishops[i];
            } else {
                new_wizard.bishops[i] = other.bishops[i];
            }
            
            if rook_selection & (1 << i) > 0 {
                new_wizard.rooks[i] = self.rooks[i];
            } else {
                new_wizard.rooks[i] = other.rooks[i];
            }
        }
        
        trace!("Mutating bishop spells");
        new_wizard.bishops.iter_mut().par_bridge().for_each(|b: &mut Spell| {
            b.mutate();
        });
        
        trace!("Mutating rook spells");
        new_wizard.rooks.iter_mut().par_bridge().for_each(|r: &mut Spell| {
            r.mutate();
        });
        
        trace!("Initializing new wizard");
        new_wizard.initialize();
        
        // run a 'mutate' step over all the spells, randomly tweaking a few bits of
        new_wizard
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;
    use tracing::debug;

    use super::*;
    
    #[test]
    fn combining_wizards_creates_a_new_wizard() {
        let wiz1 = Wizard::new();
        let wiz2 = Wizard::new();
        
        let wiz_combined = wiz1.combine(&wiz2);
        assert_ne!(wiz1, wiz_combined);
        assert_ne!(wiz2, wiz_combined);
    }
    
    #[test]
    fn two_wizards_have_different_fitness_in_general() {
        let wiz1 = Wizard::new();
        let wiz2 = Wizard::new();
        
        assert_ne!(wiz1.fitness(), wiz2.fitness());
    }
}