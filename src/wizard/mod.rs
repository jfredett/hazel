///! Wizards are a wrapper for Magic Bitboard for Bishop, Rook, and Queen attacks.
///! They also support genetic-ish optimization... eventually.

use crate::bitboard::Bitboard;

mod tables;
mod utils;
mod consts;
mod spell;
mod initialization;
mod io;
mod debug;
pub mod arena;

use tables::*;
use utils::*;
use spell::*;
use consts::*;

use serde::{Serialize, Deserialize};


#[derive(PartialEq, Eq, Hash, Clone)]
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
}