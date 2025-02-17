use crate::{board::PieceBoard, coup::rep::Move, notation::ben::BEN, Alter, Alteration};


#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    // necessaries
    pub initial: BEN,
    pub moves: Vec<Move>,
    // caches

}

impl From<Position> for Vec<Alteration> {
    fn from(pos: Position) -> Self {
        let mut ret = pos.initial.compile();
        let mut board = PieceBoard::from(pos.initial);
        for m in pos.moves.iter() {
            let alterations = m.compile(&board);
            for a in alterations.iter() {
                board.alter_mut(*a);
            }
            ret.extend(alterations);
        }
        ret
    }
}

impl Position {
    pub fn new(fen: impl Into<BEN>, moves: Vec<Move>) -> Self {
        Self { initial: fen.into(), moves }
    }
}
