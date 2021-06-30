use std::fmt::Debug;

use super::*;

impl Debug for Wizard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wizard(rooks: {:?}, bishops: {:?})", self.rooks, self.bishops)
    }
}