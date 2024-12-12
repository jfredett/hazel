use crate::{coup::rep::Move, game::action::Action, notation::ben::BEN};

pub trait Play where Self: Clone {
    type Metadata: Clone;

    fn apply(&self, action: &Action<Move, BEN>) -> Self;

    fn metadata(&self) -> Self::Metadata;

    fn apply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self {
        *self = self.apply(action);
        self
    }
}

/* TODO: This is pending proper implementation.
pub trait Unplay where Self: Clone + Play {
    fn unapply(&self, action: &Action<Move, BEN>) -> Self;

    fn unapply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self {
        *self = self.unapply(action);
        self
    }
}
*/

#[cfg(test)]
mod tests {
    use ben::BEN;
    use fen::FEN;

    use super::*;
    use crate::{board::PieceBoard, constants::START_POSITION_FEN, coup::rep::{Move, MoveType}, game::ChessGame, notation::*};

    #[test]
    fn play_applies_correctly() {
        let game = ChessGame::<PieceBoard>::from(FEN::start_position());
        let action = Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN));
        let new_game = game.apply(&action);
        let actual_ben : BEN = new_game.into();
        assert_eq!(actual_ben, BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 2"));
    }

    #[test]
    fn play_applies_mutably_correctly() {
        let mut game = ChessGame::<PieceBoard>::from(FEN::start_position());
        let action = Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN));
        game.apply_mut(&action);
        let actual_ben : BEN = game.into();
        assert_eq!(actual_ben, BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 2"));
    }

    /*
    #[test]
    fn unplay_unapplies_correctly() {
        let game = ChessGame::<BEN>::from(FEN::start_position());
        let action = Action::Make(Move::new(A1, A2, MoveType::QUIET));
        let new_game = game.apply(&action);
        let old_game = new_game.unapply(&action);
        assert_eq!(old_game.rep, BEN::new(START_POSITION_FEN));
    }

    #[test]
    fn unplay_unapplies_mutably_correctly() {
        let mut game = ChessGame::<BEN>::from(FEN::start_position());
        let action = Action::Make(Move::new(A1, A2, MoveType::QUIET));
        game.apply_mut(&action);
        game.unapply_mut(&action);
        assert_eq!(game.rep, BEN::new(START_POSITION_FEN));
    }
    */
}
