use super::*;

impl Wizard {
    pub fn initialize(&mut self) {
        self.initialize_bishops();
        self.initialize_rooks();
    }
    
    // FIXME: There is probably an efficient way to do these both at once, but I can't be arsed
    // today.
    fn initialize_bishops(&mut self) {
        for i in 0..64 {
            loop {
                let spell = self.bishops[i];
                let mask = NOMINAL_BISHOP_ATTACKS[i];
                let mut restart = false;
                let mut entries = vec![];
                
                for (blockers, attacks) in bishop_block_and_attack_board_for(i, mask) {
                    let key = spell.key_for(blockers);

                    match self.table[key as usize] {
                        Some(attack_board) => { 
                            if attack_board != attacks {
                                restart = true;
                                break
                            }
                            entries.push((key, true))
                        }
                        None => {
                            self.table[key as usize] = Some(attacks);    
                            entries.push((key, false))
                        }
                    }
                }
                if !restart {
                    // Clean up everything
                    for (entry, dup) in entries {
                        if dup { continue; }
                        self.table[entry] = None
                    }
                    self.bishops[i].initialize(5..22);
                } else {
                    break;
                }
            }
        }
    }
    
    fn initialize_rooks(&mut self) {
        for i in 0..64 {
            loop {
                let spell = self.rooks[i];
                let mask = NOMINAL_ROOK_ATTACKS[i];
                let mut restart = false;
                let mut entries = vec![];
                
                
                for (blockers, attacks) in rook_block_and_attack_board_for(i, mask) {
                    let key = spell.key_for(blockers);

                    match self.table[key as usize] {
                        Some(attack_board) => { 
                            if attack_board != attacks {
                                restart = true;
                                break
                            }
                            entries.push((key, true))
                        }
                        None => {
                            self.table[key as usize] = Some(attacks);    
                            entries.push((key, false))
                        }
                    }
                }
                if !restart {
                    // Clean up everything
                    for (entry, dup) in entries {
                        if dup { continue; }
                        self.table[entry] = None
                    }
                    self.rooks[i].initialize(10..22);
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn an_initialized_wizard_has_slots_filled_in_their_table() {
        let mut w = Wizard::empty();
        w.initialize();
        for &e in w.table.iter() {
           match e {
                Some(_) => assert!(true),
                _ => { }
           } 
        }
    }
}