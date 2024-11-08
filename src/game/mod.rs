use action::{chess::{ChessAction, EndGameState}, game::GameAction};
use tracing::{debug, instrument};

use crate::{board::Alter, types::log::Log};

use crate::{board::PieceBoard, coup::rep::Move, notation::fen::{PositionMetadata, FEN}};
pub mod action;
pub mod variation_builder;
pub mod compiles_to;

#[derive(Default, Clone)]
pub struct Game {
    // Active Data
    /// A record of every action in the game
    chess_log: Log<ChessAction>,

    // Caches / Derived Data
}

impl Game {
    pub fn commit(&mut self) -> &mut Self {
        // update cached state?
        self.chess_log.commit();
        self
    }

    fn record(&mut self, action: ChessAction) -> &mut Self {
        self.chess_log.record(action);
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

    pub fn end_game(&mut self, state: EndGameState) -> &mut Self {
        self.record(ChessAction::EndGame(state));
        self
    }

    pub fn setup(&mut self, fen: FEN) -> &mut Self {
        self.record(ChessAction::Setup(fen.clone()));
        self
    }

    pub fn variation(&mut self) -> &mut Self {
        todo!();
    }

    pub fn current_position(&self) -> FEN {
        self.chess_log.cursor(|cursor| {
            let mut board = PieceBoard::default();
            let mut metadata = PositionMetadata::default();
            while let Some(action) = cursor.next() {
                debug!("Processing action: {:?}", action);
                match action {
                    ChessAction::NewGame => {
                        board = PieceBoard::default();
                        metadata = PositionMetadata::default();
                    },
                    ChessAction::EndGame(_) => {
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

    // 2-NOV-2024
    //
    // Above are functions for creating a game, below should be functions for querying the action
    // log to generate a specific gamestate from alterations. #variation above should also probably
    // return that variationbuilder struct. I may want to consider using a library for this builder
    // stuff, but DIY doesn't seem so bad right now.
    //
    // I also need functions for dumping to/reading from PGN.
    //
    // I think adding a `Comment` Action is probably worthwhile, so I can stick debug info in the
    // log. #rewind is a very low-level primitive, it's also different then the others, since it
    // alters the state of Game, and doesn't record a new action (it's something that can happen
    // outside the turn structure.
    //
    // 3-NOV-2024
    //
    // I also should have a similar abstraction for Rewind/Seek/etc, the GameLog is then composed
    // of GameActions, which can contain ChessActions or meta actions which just affect the
    // Gamestate external to making a move (e.g., modesetting and the like).
    //
    // I think that may exist external to this particular log, I'm not sure if it's one struct or
    // two. Right now Alterations are a dependent thing, ChessActions become Alterations, maybe I
    // could do a GameAction -> Alteration, where they're written as a meta tag in the alteration
    // stream.
    //
    // That would also let ActionCache naturally cover the GameActions and the ChessAction cases.
    //
    // I think I can push the 'compile' methods into ActionCache (maybe it's ActionCompiler at that
    // point?)
    //
    // So the new design is something like:
    //
    //
    // GameAction:
    //    Step
    //    Rewind
    //    Seek(isize)
    //    Play(ChessAction)
    // ChessAction:
    //  NewGame + EndGame(State) xor Game(Delim) // maybe reuse the delim idea?
    //  Setup
    //  Make
    //  NewVariation + EndVariation(State) xor Variation(Delim) // maybe don't use the delim idea?
    //  Literal(Alteration)
    //  Comment(String)
    // Alteration:
    //  Place
    //  Remove
    //  Clear
    //  Meta(Vec<u8>)
    //
    // This would give me a very natural tree, and I can add additional sublanguages to the
    // `GameAction` to record state changes for, e.g., time control, movegen, etc.
    //
    // This whole tree could then be 'compiled' to bytecode, each variant is assigned a byte, and
    // we assume we are reading GameAction bytes to start, then if we see the play byte, we switch
    // contexts, etc.
    //
    // All of these bytes are then compiled to alterations, either predefined ones like
    // place/remove/clear, or just an arbitrary list of bytes, this gives us a final bytestream
    // of alterations that can then be used to respresent the boardstate.
    //
    // I'm going to start with the gameactions as methods, then wrap them in the GameAction enum
    // later.
    //
}



#[cfg(test)]
mod tests {
    use crate::notation::*;
    use crate::{coup::rep::MoveType, types::Occupant};
    use crate::board::interface::*;

    use super::*;

    /// These are WIPs.

    #[test]
    fn fen_correct_after_one_move_from_start_pos() {
        let mut game = Game::default();
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
        let mut game = Game::default();
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
