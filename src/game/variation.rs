use tracing::debug;

use crate::{board::Alter, types::log::Log};

use crate::{board::PieceBoard, coup::rep::Move, notation::fen::{PositionMetadata, FEN}};

use super::action::chess::{ChessAction, Delim, Reason};

#[derive(Default, Clone)]
pub struct Variation {
    // Active Data
    /// A record of every action in the game
    log: Log<ChessAction>,
    halted: bool

    // Caches / Derived Data
}

impl Variation {
    pub fn new() -> Self {
        Self {
            log: Log::start(),
            halted: false
        }
    }

    pub fn commit(&mut self) -> &mut Self {
        if self.halted { return self; }

        // update cached state?
        self.log.commit();
        self
    }

    pub fn commit_all(&mut self) -> &mut Self {
        if self.halted { return self; }

        self.log.commit_all();
        self
    }

    pub fn make(&mut self, mov: Move) -> &mut Self {
        self.record(ChessAction::Make(mov));
        self
    }


    pub fn new_game(&mut self) -> &mut Self {
        self.record(ChessAction::NewGame);
        self
    }

    pub fn halt(&mut self, state: Reason) -> &mut Self {
        self.record(ChessAction::Halted(state));
        self
    }

    pub fn setup(&mut self, fen: FEN) -> &mut Self {
        self.record(ChessAction::Setup(fen.clone()));
        self
    }

    pub fn variation(&mut self, block: impl Fn(&mut Variation)) -> &mut Self {
        self.log.begin();

        let mut variation = Variation::new();

        block(&mut variation);

        variation.commit_all();

        self.record(ChessAction::Variation(Delim::Start));
        for action in variation.log.into_iter() {
            self.record(action);
        }
        self.record(ChessAction::Variation(Delim::End));

        self
    }

    pub fn current_position(&self) -> FEN {
        self.log.cursor(|cursor| {
            let mut board = PieceBoard::default();
            let mut metadata = PositionMetadata::default();
            while let Some(action) = cursor.next() {
                debug!("Processing action: {:?}", action);
                match action {
                    ChessAction::NewGame => {
                        board = PieceBoard::default();
                        metadata = PositionMetadata::default();
                    },
                    ChessAction::Halted(_) => {
                        todo!();
                    },
                    ChessAction::Variation(_) => {
                        // This is a variation, so we don't need to do anything. Only reading the
                        // mainline
                    },
                    ChessAction::Setup(fen) => {
                        board.set_fen(fen);
                    },
                    ChessAction::Make(mov) => {
                        metadata.update(mov, &board);
                        for alter in mov.compile(&board) {
                            board.alter_mut(alter);
                        }
                        debug!("After move metadata:\n{:?}", metadata);
                    },
                }
            }

            // Now board and metadata are caught up, so we just ask board to write it's fen
            let mut ret = FEN::from(board);
            ret.set_metadata(metadata);
            debug!("Final FEN: {:?}", ret);
            ret
        })
    }

    fn record(&mut self, action: ChessAction) -> &mut Self {
        if self.halted { return self; }

        self.log.record(action);
        self
    }
}



#[cfg(test)]
mod tests {
    use crate::notation::*;
    use crate::{coup::rep::MoveType, types::Occupant};
    use crate::board::interface::*;

    use super::*;

    #[test]
    fn fen_correct_after_one_move_from_start_pos() {
        let mut game = Variation::default();
        game.new_game()
            .setup(FEN::start_position())
            .make(Move::new(D2, D4, MoveType::DOUBLE_PAWN))
            .commit();

        let actual_fen = game.current_position();

        assert_eq!(actual_fen, FEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 2"));
    }

    #[test]
    #[tracing_test::traced_test]
    fn fen_correct_after_castling() {
        let mut game = Variation::default();
        game.new_game()
            .setup(FEN::start_position())
            .make(Move::new(E2, E4, MoveType::DOUBLE_PAWN))
            .make(Move::new(E7, E5, MoveType::DOUBLE_PAWN))
            .make(Move::new(G1, F3, MoveType::QUIET))
            .make(Move::new(B8, C6, MoveType::QUIET))
            .make(Move::new(F1, E2, MoveType::QUIET))
            .make(Move::new(G8, F6, MoveType::QUIET))
            .make(Move::new(E1, G1, MoveType::SHORT_CASTLE))
            .commit();

        let actual_fen = game.current_position();

        assert_eq!(actual_fen, FEN::new("r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/5N2/PPPPBPPP/RNBQ1RK1 b kq - 5 5"));
    }
}