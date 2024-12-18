use crate::{constants::START_POSITION_FEN, coup::rep::Move, game::{action::Action, ChessGame}, notation::ben::BEN, Alter, Play, Query};

#[derive(Clone, Debug)]
pub enum MoveSheetEntry {
    Move(Move),
    Branch 
}

#[derive(Clone, Default, Debug)]
pub struct MoveSheet {
    sheet: Vec<MoveSheetEntry>,
    line: Vec<Move>,
    initial_state: BEN
}

impl MoveSheet {
    pub fn new() -> Self {
        Self {
            line: Vec::new(),
            sheet: Vec::new(),
            initial_state: BEN::new(START_POSITION_FEN)
        }
    }

    pub fn set_initial_state(&mut self, ben: BEN) {
        self.initial_state = ben;
    }

    pub fn line(&self) -> &[Move] {
        &self.line
    }

    pub fn record(&mut self, m: Move) {
        self.sheet.push(MoveSheetEntry::Move(m));
        self.line.push(m);
    }

    pub fn branch(&mut self) {
        self.sheet.push(MoveSheetEntry::Branch);
        self.line.pop();
    }

    pub fn prune(&mut self) {
        while let Some(entry) = self.sheet.pop() {
            if matches!(entry, MoveSheetEntry::Branch) { break; }
            self.line.pop();
        };

        if let MoveSheetEntry::Move(m) = self.sheet.last().unwrap() {
            self.line.push(*m);
        } else {
            panic!("Invalid MoveSheet, double-branch");
        }
    }

    pub fn unwind(&mut self) {
        match self.sheet.pop() {
            Some(MoveSheetEntry::Move(m)) => {
                self.line.pop();
            },
            Some(MoveSheetEntry::Branch) => {
                if let MoveSheetEntry::Move(m) = self.sheet.last().unwrap() {
                    self.line.push(*m);
                } else {
                    panic!("Invalid MoveSheet, double-branch");
                }
            },
            None => {}
        }
    }
}

impl<T> From<MoveSheet> for ChessGame<T> where T : Alter + Query + Default + Clone {
    fn from(sheet: MoveSheet) -> Self {
        ChessGame::<T>::from(&sheet)
    }
}

impl<T> From<&MoveSheet> for ChessGame<T> where T : Alter + Query + Default + Clone {
    fn from(sheet: &MoveSheet) -> Self {
        let mut game = ChessGame::default();
        game.apply_mut(&Action::Setup(sheet.initial_state));

        for m in &sheet.line {
            game.apply_mut(&Action::Make(*m));
        }
        game
    }
}

#[cfg(test)]
mod tests {
    use crate::board::PieceBoard;
    use crate::coup::rep::MoveType;
    use crate::notation::*;

    use super::*;


    #[test]
    fn movesheet_calculates_rep_correctly() {
        let mut sheet = MoveSheet::new();

        sheet.record(Move::new(D2, D4, MoveType::DOUBLE_PAWN));

        let game : ChessGame<PieceBoard> = ChessGame::<PieceBoard>::from(&sheet);
        let actual_fen : BEN = game.into();

        let expected_fen = BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 2");

        assert_eq!(actual_fen, expected_fen);
    }
}
