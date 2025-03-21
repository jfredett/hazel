use crate::{types::zobrist::Zobrist, Alter, Alteration};

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct PositionZobrist {
    pub current: Zobrist,
    pub position: Zobrist
}

impl Alter for PositionZobrist {
    fn alter_mut(&mut self, alter: Alteration) -> &mut Self {
        tracing::debug!("self.current-before {:?}", self.current);
        self.current.alter_mut(alter);
        tracing::debug!("self.current-after {:?}", self.current);

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
