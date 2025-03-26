use crate::{ben::BEN, occupant::Occupant, position_metadata::PositionMetadata, square::Square};

use crate::interface::Query;


impl Query for BEN {
    fn get(&self, square: impl Into<Square>) -> Occupant {
        let sq : usize = square.into().index();
        let byte_index = sq / 2;
        let occupant_nibble = if sq % 2 == 0 {
            self.position[byte_index] >> 4
        } else {
            self.position[byte_index] & 0b00001111
        };
        Occupant::from(occupant_nibble)
    }

    fn try_metadata(&self) -> Option<PositionMetadata> {
        Some(self.metadata)
    }
}
