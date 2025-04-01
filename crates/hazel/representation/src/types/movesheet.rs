use hazel_core::{ben::BEN, interface::{Alter, Query}};

use crate::{board::PieceBoard, coup::rep::Move, game::{action::Action, ChessGame}, play::Play};

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
            initial_state: BEN::start_position()
        }
    }

    pub fn set_initial_state(&mut self, ben: BEN) {
        self.initial_state = ben;
    }

    pub fn current_move(&self) -> Option<Move> {
        self.line.last().copied()
    }

    pub fn last_move_string(&self) -> Option<String> {
        let mut ctx = self.line.clone();
        ctx.pop();

        let mut g : ChessGame<PieceBoard> = ChessGame::default();
        g.apply_mut(&Action::Setup(self.initial_state));

        for m in ctx {
            g.apply_mut(&Action::Make(m));
        }

        self.current_move().map(|m| m.to_pgn(&g.rep))
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
            Some(MoveSheetEntry::Move(_)) => {
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

        for m in sheet.line() {
            game.apply_mut(&Action::Make(*m));
        }
        game
    }
}

#[cfg(test)]
mod tests {
    use crate::board::PieceBoard;
    use crate::coup::rep::MoveType;
    use hazel_core::square::*;

    use super::*;

    #[test]
    fn unwinding_and_empty_sheet_is_a_noop() {
        let mut sheet = MoveSheet::new();
        assert_eq!(sheet.line().len(), 0);
        sheet.unwind();
        assert_eq!(sheet.line().len(), 0);
    }

    #[test]
    #[should_panic]
    fn unwind_past_a_double_branch_should_panic() {
        // there should never be a case in a valid variation where the branch appears twice in a
        // row.
        let mut sheet = MoveSheet::new();
        sheet.record(Move::new(D2, D3, MoveType::QUIET));
        sheet.branch();
        sheet.branch();
        sheet.record(Move::new(D3, D4, MoveType::QUIET));

        sheet.unwind();
        sheet.unwind();
        sheet.unwind(); // should panic here
    }

    #[test]
    #[should_panic]
    fn pruning_past_a_double_branch_should_panic() {
        // there should never be a case in a valid variation where the branch appears twice in a
        // row.
        let mut sheet = MoveSheet::new();
        sheet.record(Move::new(D2, D3, MoveType::QUIET));
        sheet.branch();
        sheet.branch();
        sheet.record(Move::new(D3, D4, MoveType::QUIET));

        sheet.prune();
    }

    #[test]
    fn unwinding_past_a_valid_variation_works() {
        // there should never be a case in a valid variation where the branch appears twice in a
        // row.
        let mut sheet = MoveSheet::new();
        sheet.record(Move::new(D2, D4, MoveType::QUIET));
        sheet.branch();
        sheet.record(Move::new(D2, D3, MoveType::DOUBLE_PAWN));

        sheet.unwind();
        sheet.unwind();

        let rep = ChessGame::<PieceBoard>::from(sheet);
        let ben : BEN = rep.into();

        assert_eq!(ben, BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1"));
    }

    #[test]
    fn movesheet_calculates_rep_correctly() {
        let mut sheet = MoveSheet::new();

        sheet.record(Move::new(D2, D4, MoveType::DOUBLE_PAWN));

        let game : ChessGame<PieceBoard> = ChessGame::<PieceBoard>::from(sheet);
        let actual_fen : BEN = game.into();

        let expected_fen = BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");

        assert_eq!(actual_fen, expected_fen);
    }
}
