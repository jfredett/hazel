use std::fmt::{Debug, Formatter};
use crate::{board::{Alteration, Query}, coup::rep::Move, game::Game, notation::fen::{PositionMetadata, FEN}, types::Color};

use crate::game::compiles_to::CompilesTo;

#[derive(Debug, Clone, PartialEq)]
pub enum Delim {
    Start,
    End
}

#[derive(Debug, Clone, PartialEq)]
pub enum EndGameState {
    Winner(Color),
    Stalemate,
    Aborted
}

#[derive(Clone, PartialEq)]
pub enum ChessAction {
    NewGame,
    EndGame(EndGameState),
    Variation(Delim),
    Setup(FEN),
    Make(Move),
}

impl Debug for ChessAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessAction::NewGame => write!(f, "NewGame"),
            ChessAction::EndGame(egs) => write!(f, "EndGame({:?})", egs),
            ChessAction::Variation(Delim::Start) => write!(f, "Variation(Start)"),
            ChessAction::Variation(Delim::End) => write!(f, "Variation(End)"),
            ChessAction::Setup(fen) => write!(f, "Setup({})", fen),
            ChessAction::Make(mov) => write!(f, "Make({:?})", mov),
        }
    }
}

/*
impl<Q : Query> CompilesTo<Vec<Alteration>> for ChessAction {
    type Context = Q;

    fn compile(&self, context: &Self::Context) -> Vec<Alteration> where Self::Context : Query {
        match self {
            ChessAction::NewGame => vec![Alteration::Clear],
            ChessAction::EndGame(_egs) => vec![
            ],
            ChessAction::Variation(_) => vec![
            ],
            ChessAction::Setup(fen) => fen.compile(),
            ChessAction::Make(mov) => mov.compile(context),
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

}
