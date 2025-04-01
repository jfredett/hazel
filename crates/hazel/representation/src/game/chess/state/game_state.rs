/* TODO: Move this back to core.
*
use crate::board::PieceBoard;
use hazel_core::{interface::{Alter, Alteration}, position_metadata::PositionMetadata};

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
    fn alter(&self, mov: Alteration) -> Self {
        let mut copy = self.clone();
        copy.alter_mut(mov);
        copy
    }

    fn alter_mut(&mut self, mov: Alteration) -> &mut Self {
        self.board.alter_mut(mov);
        self.metadata.alter_mut(mov);
        self
    }
}


#[cfg(test)]
mod tests {

}
*/
