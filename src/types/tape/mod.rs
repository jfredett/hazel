use std::fmt::Debug;

use crate::{Alter, Alteration};

use crate::types::zobrist::Zobrist;

pub mod cursor;
pub mod cursorlike;
pub mod familiar;
pub mod tape;
pub mod tape_direction;
pub mod tapelike;

// TODO: Figure out where this should live

#[derive(Default, Clone, Copy)]
struct PositionZobrist {
    pub current: Zobrist,
    pub position: Zobrist
}

impl Alter for PositionZobrist {
    fn alter_mut(&mut self, alter: Alteration) -> &mut Self {
        self.current.alter_mut(alter);

        if matches!(alter, Alteration::End) {
            self.position = self.current;
        }
        self
    }

    fn alter(&self, alter: Alteration) -> Self {
        let mut ret = *self;
        ret.alter_mut(alter);
        ret
    }
}

