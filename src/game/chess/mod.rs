pub mod action;
pub mod castle_rights;
pub mod delim;
pub mod familiar;
pub mod position_metadata;
pub mod reason;
pub mod variation;

use std::fmt::{Debug, Formatter};

use action::Action;
use tracing::debug;

use crate::coup::rep::Move;
use crate::game::position_metadata::PositionMetadata;
use crate::interface::{Alter, Query, Play};
use crate::notation::ben::BEN;
use crate::notation::fen::FEN;

#[derive(Clone, Default)]
pub struct ChessGame<T> where T: Alter + Query + Default + Clone {
    // FIXME: This is bad, I don't like it.
    pub rep: T,
    pub metadata: PositionMetadata,
}


impl<T> Debug for ChessGame<T> where T: Debug + Alter + Query + Default + Clone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChessGame {{ rep: {:?}, metadata: {:?} }}", self.rep, self.metadata)
    }
}

impl<T> ChessGame<T> where T: Alter + Query + Default + Clone {
    pub fn unmake(&mut self, _action: Move) {
        todo!();
    }
}

impl<T> From<BEN> for ChessGame<T> where T : From<BEN> + Alter + Query + Default + Clone {
    fn from(ben: BEN) -> Self {
        let rep = T::from(ben);
        ChessGame {
            rep,
            metadata: PositionMetadata::default(),
        }
    }
}

impl<T> From<FEN> for ChessGame<T> where T : From<FEN> + Alter + Query + Default + Clone {
    fn from(fen: FEN) -> Self {
        let rep = T::from(fen.clone());
        ChessGame {
            rep,
            metadata: fen.metadata(),
        }
    }
}

impl<T> From<ChessGame<T>> for FEN where T : Into<FEN> + Alter + Query + Default + Clone {
    fn from(game: ChessGame<T>) -> Self {
        let mut ret = game.rep.into();
        ret.set_metadata(game.metadata);
        ret
    }
}

impl<T> From<ChessGame<T>> for BEN where T : Into<BEN> + Alter + Query + Default + Clone {
    fn from(game: ChessGame<T>) -> Self {
        let mut ret = game.rep.into();
        ret.set_metadata(game.metadata);
        ret
    }
}

/*
* In this design, ChessGame can only roll _forward_, the unplay trait would require a bunch more
* context I don't have and don't want to put in ChessGame, so I suppose it'll have to be
* implemented further up in Familiar or something.
*/

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
                let alts = fen.compile();
                for a in alts {
                    self.rep.alter_mut(a);
                }
                self.metadata = fen.metadata();
            }
            Action::Make(mov) => {
                let alts = mov.compile(&self.rep);
                // Order matters, the metadata must be updated before the board
                debug!("Applying move: {:?}", mov);
                debug!("Metdata before: {:?}", self.metadata);
                self.metadata.update(mov, &self.rep);
                debug!("Metdata after: {:?}", self.metadata);
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
    use crate::coup::rep::MoveType;
    use crate::types::{Color, Occupant};
    use crate::{constants::START_POSITION_FEN, coup::rep::Move, notation::fen::FEN};
    use crate::notation::*;

    use super::*;


    #[test]
    #[tracing_test::traced_test]
    fn correctly_calculates_position_after_several_moves() {
        let mut game : ChessGame<PieceBoard> = ChessGame::default();
        game.apply_mut(&Action::Setup(FEN::new(START_POSITION_FEN).into()))
            .apply_mut(&Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN)));

        let expected_fen = FEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 2");
        let actual_fen = FEN::with_metadata(game.rep, game.metadata);

        similar_asserts::assert_eq!(actual_fen, expected_fen);
    }
}

