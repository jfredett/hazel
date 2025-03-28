use crate::{board::PieceBoard, game::position_metadata::PositionMetadata, Alter};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct GameState {
    pub board: PieceBoard,
    pub metadata: PositionMetadata
}

impl GameState {
    pub fn new(board: PieceBoard, metadata: PositionMetadata) -> GameState {
        GameState {
            board,
            metadata
        }
    }
}


impl Alter for GameState {
    fn alter(&self, mov: crate::Alteration) -> Self {
        let mut copy = self.clone();
        copy.alter_mut(mov);
        copy
    }

    fn alter_mut(&mut self, mov: crate::Alteration) -> &mut Self {
        self.board.alter_mut(mov);
        self.metadata.alter_mut(mov);
        self
    }
}


#[cfg(test)]
mod tests {

}
