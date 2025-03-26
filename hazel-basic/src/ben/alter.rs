use crate::ben::BEN;
use crate::interface::{Alter, Alteration};

impl Alter for BEN {
    fn alter(&self, alter: Alteration) -> Self {
        let mut ben = *self;
        ben.alter_mut(alter);
        ben
    }

    fn alter_mut(&mut self, alter: Alteration) -> &mut Self {
        match alter {
            Alteration::Place { square, occupant } => {
                let sq : usize = square.index();
                let byte_index = sq / 2;
                let nibble : u8 = occupant.into();
                if sq % 2 == 0 {
                    self.position[byte_index] = (nibble << 4) | (self.position[byte_index] & 0b00001111);
                } else {
                    self.position[byte_index] = nibble | (self.position[byte_index] & 0b11110000);
                }
            },
            Alteration::Remove { square, .. } => {
                let sq : usize = square.index();
                let byte_index = sq / 2;
                if sq % 2 == 0 {
                    self.position[byte_index] &= 0b00001111;
                } else {
                    self.position[byte_index] &= 0b11110000;
                }
            },
            Alteration::Clear => { self.position = [0; 32]; },
            Alteration::Assert(_) | Alteration::Inform(_) => { self.metadata.alter_mut(alter); },
            _ => { }
        }

        self
    }
}
