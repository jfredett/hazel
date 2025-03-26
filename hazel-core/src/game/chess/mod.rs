pub mod action;
pub mod delim;
pub mod familiar;
pub mod position;
pub mod reason;
pub mod variation;

use action::Action;
use hazel_basic::ben::BEN;
use hazel_basic::position_metadata::PositionMetadata;

use crate::coup::rep::Move;
use crate::interface::{Alter, Query, Play};

#[derive(Clone, Default)]
pub struct ChessGame<T> where T: Alter + Query + Default + Clone {
    // FIXME: This is bad, I don't like it.
    pub rep: T,
    pub metadata: PositionMetadata,
}


/*
* In this design, ChessGame can only roll _forward_, the unplay trait would require a bunch more
* context I don't have and don't want to put in ChessGame, so I suppose it'll have to be
* implemented further up in Familiar or something.
*/

impl<T> From<ChessGame<T>> for BEN where T : Alter + Query + Default + Clone {
    fn from(value: ChessGame<T>) -> Self {
        // TODO: Replace this with a generic 'FastRep' type alias that is optimized for this case
        let mut fen : BEN = hazel_basic::interface::query::convert_representation(&value.rep);
        fen.set_metadata(value.metadata);
        fen
    }
}

// TODO: Maybe wrap the constraint in it's own typeclass?
impl<T> Play for ChessGame<T> where T: Alter + Query + Default + Clone {
    type Metadata = PositionMetadata;

    fn apply(&self, action: &Action<Move, BEN>) -> Self {
        let mut new_game = self.clone();
        new_game.apply_mut(action);
        new_game
    }

    fn apply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self {
        match action {
            Action::Setup(fen) => {
                let alts = fen.to_alterations();
                for a in alts {
                    self.rep.alter_mut(a);
                }
                self.metadata = fen.metadata();
            }
            Action::Make(mov) => {
                /* FIXME: this should be using a Position, which obviates the need for the metadata
                * update call */
                let alts = mov.compile(&self.rep);
                // Order matters, the metadata must be updated before the board
                // self.metadata.update(mov, &self.rep);
                for a in alts {
                    self.rep.alter_mut(a);
                }
            }
            _ => {}
        }
        self
    }

    fn metadata(&self) -> PositionMetadata {
        self.metadata
    }
}


#[cfg(test)]
mod tests {
    use crate::board::PieceBoard;
    use hazel_basic::square::*;
    use hazel_basic::ben::BEN;
    use crate::{coup::rep::{Move, MoveType}, game::ChessGame};

    use super::*;


    #[test]
    fn correctly_calculates_position_after_several_moves() {
        let mut game : ChessGame<PieceBoard> = ChessGame::default();
        game.apply_mut(&Action::Setup(BEN::start_position()))
            .apply_mut(&Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN)));

        let expected_fen = BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");
        let actual_fen : BEN = game.into();

        similar_asserts::assert_eq!(actual_fen, expected_fen);
    }

    mod from_into {
        use super::*;

        // FIXME: The way I'm using from/into is pretty incoherent, I should look at better ways,
        // maybe restrict to more basic types only?
        //
        // #[test]
        // fn from_ben() {
        //     let ben = BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq d3 0 2");
        //     let game : ChessGame<PieceBoard> = ben.into();
        //     let expected_fen = BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq d3 0 2");
        //     let actual_fen = BEN::from(game);

        //     similar_asserts::assert_eq!(actual_fen, expected_fen);
        // }

        #[test]
        fn into_ben() {
            let mut game : ChessGame<PieceBoard> = ChessGame::default();
            game.apply_mut(&Action::Setup(BEN::start_position()));

            let ben : BEN = game.clone().into();
            let expected_fen = BEN::start_position();

            similar_asserts::assert_eq!(ben, expected_fen);
        }
    }

    /* FIXME: This moves to -engine I think.
    mod play_impl {

        use super::*;

        #[test]
        fn play_applies_correctly() {
            let game = ChessGame::<PieceBoard>::from(BEN::start_position());
            let action = Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN));
            let new_game = game.apply(&action);
            let actual_ben : BEN = new_game.into();
            assert_eq!(actual_ben, BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1"));
        }

        #[test]
        fn play_applies_mutably_correctly() {
            let mut game = ChessGame::<PieceBoard>::from(BEN::start_position());
            let action = Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN));
            game.apply_mut(&action);
            let actual_ben : BEN = game.into();
            assert_eq!(actual_ben, BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1"));
        }
    }
    */
}
