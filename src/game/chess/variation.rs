use crate::types::log::cursor::Cursor;
use crate::{interface::Alter, types::log::Log};

use crate::{board::PieceBoard, coup::rep::Move, notation::fen::{PositionMetadata, FEN}};

use super::action::{ChessAction, Delim, Reason};

#[derive(Debug, Default, Clone)]
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
        self.record(ChessAction::Setup(FEN::start_position()));
        self
    }

    pub fn halt(&mut self, state: Reason) -> &mut Self {
        self.record(ChessAction::Halt(state));
        self
    }

    pub fn setup(&mut self, fen: FEN) -> &mut Self {
        self.record(ChessAction::Setup(fen.clone()));
        self
    }

    // NOTE: I'm not in love with these methods. I would prefer to have some psuedo-atomic way to
    // do this in the PGN::parse side of things, but a pleasant way is not obvious and this better
    // matches the tokenization, so I'm going with it.
    pub(crate) fn start_variation(&mut self) -> &mut Self {
        self.record(ChessAction::Variation(Delim::Start));
        self
    }

    // NOTE: see #start_variation
    pub(crate) fn end_variation(&mut self) -> &mut Self {
        self.record(ChessAction::Variation(Delim::End));
        self
    }

    pub fn variation(&mut self, block: impl Fn(&mut Variation)) -> &mut Self {
        self.log.begin();

        let mut variation = Variation::default();

        block(&mut variation);

        variation.commit_all();

        self.record(ChessAction::Variation(Delim::Start));
        for action in variation.log.into_iter() {
            self.record(action);
        }
        self.record(ChessAction::Variation(Delim::End));

        self.log.commit();

        self
    }

    // FIXME: This is the current broken thing, I need to encode the assumptions wrt a variation
    // and to correctly calculate the current position. I know the correct search algoithm
    // abstractly, it's something like:
    //
    // "Identify a location in the log to which you want to seek, then starting from the GameStart,
    // proceed applying moves until you reach a varaition, look ahead and see if your location is
    // inside the variation space (don't worry about contents, jsut position), if it is, traverse
    // into the variation and continue, if not, ignore everything in the variation and continue to
    // the next mainline move. This process is recursive.
    //
    // This will ensure during parsing PGNs that the correct context is maintained, since we always
    // want to calculate the shortest path to the variation at the tip of the log during that
    // process.
    pub fn current_position(&self) -> FEN {
        self.log.cursor(|cursor| {
            let mut board = PieceBoard::default();
            let mut metadata = PositionMetadata::default();
            while let Some(action) = cursor.next() {
                match action {
                    ChessAction::Halt(_) => {
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
                    },
                }
            }

            // Now board and metadata are caught up, so we just ask board to write it's fen
            let mut ret = FEN::from(board);
            ret.set_metadata(metadata);
            ret
        })
    }

    pub(crate) fn get_cursor(&self) -> Cursor<ChessAction> {
        self.log.raw_cursor()
    }

    /*
    pub(crate) fn get_writehead(&mut self) -> Log<ChessAction>::WriteHead {
        self.log.writehead()
    }
    */

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
    use crate::*;

    use super::*;

    impl Variation {
        pub fn log(&self) -> Vec<ChessAction> {
            self.log.log()
        }
    }

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

    #[test]
    fn fen_correct_mainline_position_when_variation_present() {
        let mut game = Variation::default();
        game.new_game()
            .setup(FEN::start_position())
            .make(Move::new(E2, E4, MoveType::DOUBLE_PAWN))
            .commit()
            .variation(|v| {
                v.make(Move::new(D2, D4, MoveType::DOUBLE_PAWN));
            }).make(Move::new(E7, E5, MoveType::DOUBLE_PAWN))
            .make(Move::new(G1, F3, MoveType::QUIET))
            .make(Move::new(B8, C6, MoveType::QUIET))
            .make(Move::new(F1, E2, MoveType::QUIET))
            .make(Move::new(G8, F6, MoveType::QUIET))
            .make(Move::new(E1, G1, MoveType::SHORT_CASTLE))
            .commit();

        let actual_fen = game.current_position();


        assert_eq!(actual_fen, FEN::new("r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/5N2/PPPPBPPP/RNBQ1RK1 b kq - 5 5"));
    }

    #[test]
    fn a_cursor_can_follow_a_variation() {
        let mut game = Variation::default();
        game.new_game()
            .setup(FEN::start_position())
            .commit()
            .variation(|v| {
                v.make(Move::new(D2, D4, MoveType::DOUBLE_PAWN)).commit();
            })
            .make(Move::new(E7, E5, MoveType::DOUBLE_PAWN))
            .make(Move::new(E2, E4, MoveType::DOUBLE_PAWN))
            .make(Move::new(G1, F3, MoveType::QUIET))
            .make(Move::new(B8, C6, MoveType::QUIET))
            .make(Move::new(F1, E2, MoveType::QUIET))
            .make(Move::new(G8, F6, MoveType::QUIET))
            .make(Move::new(E1, G1, MoveType::SHORT_CASTLE))
            .commit();


        // HACK: This is a prototype of sorts, eventually there should be a cursor that takes a
        // list of move numbers and variation numbers, e.g:
        //
        // vec![{move: 1, variation: 0}, {move: 2, variation: 0}, {move: 3, variation: 1}]
        //
        // would specify following the mainline for the first two moves, then following the first
        // variation of the third. For simplicity, this should assume every move is the mainline
        // unless otherwise specified. When a particular variation is reached, the cursor should
        // switch to that variation and halt after reading the first 'end' delimiter.
        //
        // nested variations will have many end delimiters, so this quitting after finding the
        // first one I think is correct, but remains to be seen.
        //
        // This test, for now, should cover the variation case in for now.
        let line = game.log.cursor(|cursor| {
            let mut board = PieceBoard::default();
            let mut metadata = PositionMetadata::default();
            while let Some(action) = cursor.next() {
                match action {
                    ChessAction::Halt(_) => {
                        /* do nothing */
                    },
                    ChessAction::Variation(v) => {
                        match v {
                            Delim::Start => { },
                            Delim::End => {
                                break;
                            }
                        }
                    },
                    ChessAction::Setup(fen) => {
                        board.set_fen(fen);
                    },
                    ChessAction::Make(mov) => {
                        metadata.update(mov, &board);
                        for alter in mov.compile(&board) {
                            board.alter_mut(alter);
                        }
                    },
                }
            }

            // Now board and metadata are caught up, so we just ask board to write it's fen
            let mut ret = FEN::from(board);
            ret.set_metadata(metadata);
            ret
        });


        assert_eq!(line, FEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 2"));
    }
}
