use tracing::{debug, info, instrument, trace};
use crate::constants::INDEX_TO_NOTATION;

use super::*;

impl Wizard {
    pub fn initialize(&mut self) {
        self.initialize_rooks();
        self.initialize_bishops();
    }
    
    #[instrument(skip(self))]
    fn initialize_rooks(&mut self) {
        info!("Initializing Rooks");
        let mut loop_count = 0;
        for (i, spell) in self.rooks.iter_mut().enumerate() {
            let mut restart_count = 0;
            loop {
                let mut restart = false;
                let mut entries: Vec<(usize, bool)> = vec![];
                
                spell.initialize(ROOK_INDEX_MINS[i]..MAX_SHIFT);

                loop_count += 1;
                if loop_count % 1000 == 0 {
                    trace!("current rook square: {}", INDEX_TO_NOTATION[i]);
                }
                
                for (blockers, attacks) in rook_block_and_attack_board_for(i, NOMINAL_ROOK_ATTACKS[i]) {
                    let key = spell.key_for(blockers);
                    
                    let dup = match self.table[key] {
                        Some(attack_board) => { 
                            if attack_board != attacks {
                                restart = true;
                                break;
                            }
                            true
                        },
                        None => {
                            self.table[key] = Some(attacks);    
                            false
                        }
                    };
                    entries.push((key, dup));
                }
                    
                if restart {
                    restart_count += 1;
                    if restart_count % 100 == 0 { 
                        debug!("rook restarted {}", restart_count); 
                        trace!("rook init magic: {}", spell.magic);
                        trace!("rook init shift: {}", spell.shift);
                        trace!("rook init offset: {}", spell.offset);
                    }
                    // Clean up everything
                    for (entry, dup) in entries {
                        if dup { continue; }
                        self.table[entry] = None
                    }
                    spell.mutate();
                } else {
                    break;
                }
                
            }
        }
    }
    #[instrument(skip(self))]
    fn initialize_bishops(&mut self) {
        info!("Initializing Bishops");
        let mut loop_count = 0;
        for (i, spell) in self.bishops.iter_mut().enumerate() {
            let mut restart_count = 0;
            loop {
                let mut restart = false;
                let mut entries: Vec<(usize, bool)> = vec![];
                
                spell.initialize(BISHOP_INDEX_MINS[i]..MAX_SHIFT);

                loop_count += 1;
                if loop_count % 1000 == 0 {
                    trace!("current bishop square: {}", INDEX_TO_NOTATION[i]);
                }
                
                for (blockers, attacks) in bishop_block_and_attack_board_for(i, NOMINAL_BISHOP_ATTACKS[i]) {
                    let key = spell.key_for(blockers);
                    
                    let dup = match self.table[key] {
                        Some(attack_board) => { 
                            if attack_board != attacks {
                                restart = true;
                                break;
                            }
                            true
                        },
                        None => {
                            self.table[key] = Some(attacks);    
                            false
                        }
                    };
                    entries.push((key, dup));
                }
                    
                if restart {
                    restart_count += 1;
                    if restart_count % 1000 == 0 { 
                        debug!("bishop restarted {}", restart_count); 
                        trace!("bishop init magic: {}", spell.magic);
                        trace!("bishop init shift: {}", spell.shift);
                        trace!("bishop init offset: {}", spell.offset);
                    }
                    // Clean up everything
                    for (entry, dup) in entries {
                        if dup { continue; }
                        self.table[entry] = None
                    }
                    spell.mutate();
                } else {
                    break;
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