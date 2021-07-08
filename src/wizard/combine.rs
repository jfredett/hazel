use tracing::info;

use super::*;

impl Wizard {
    /// The function to be minimized
    pub fn fitness(&self) -> usize {
        // one collision should double the effective fitness (really lack of fitness), over time we
        // want this function to be minimized so collisions = 0 is critical. However, particularly
        // bad collisions might reduce the space_occupied significantly, so this may need tweaking
        // if we get unstable collision counts
        self.space_occupied() * (1 + self.collisions)
    } 

    /// Takes two wizards and combines them into a new wizard.
    pub fn combine(&self, other: &Wizard) -> Wizard {
        info!("Combining wizards");
        let mut new_wizard = Wizard::empty();

        info!("Weaving wizard spellbooks");
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
        
        info!("Mutating bishop spells");
        for b in &mut new_wizard.bishops {
            b.mutate();
        }
        
        info!("Mutating rook spells");
        for r in &mut new_wizard.rooks {
            r.mutate();
        }
        
        info!("Initializing new wizard");
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