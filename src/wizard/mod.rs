///! Wizards are a wrapper for Magic Bitboard for Bishop, Rook, and Queen attacks.
///! They also support genetic-ish optimization... eventually.

use crate::bitboard::Bitboard;

mod tables;
mod utils;
mod consts;
mod spell;

use tables::*;
use utils::*;
use spell::*;
use consts::*;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Wizard {
    // Metadata -- 13b * 128 ~ 1.625KiB
    rooks: [Spell; BOARD_SIZE],
    bishops: [Spell; BOARD_SIZE],
    // Attack Table -- 2.215MiB
    table: Box<[Option<Bitboard>; TABLE_SIZE]>
}

impl Wizard {
    pub fn empty() -> Wizard {
        Wizard {
            rooks: [Spell::empty(); BOARD_SIZE],
            bishops: [Spell::empty(); BOARD_SIZE],
            table: box [None; TABLE_SIZE]
        }
    }
    
    pub fn new() -> Wizard { 
       let mut acolyte = Wizard::empty();
       for i in 0..64 {
           acolyte.rooks[i].initialize(10..22);
           acolyte.bishops[i].initialize(5..22);
       }
       
       acolyte
    }
    
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
                }
            }
        }
    }
}