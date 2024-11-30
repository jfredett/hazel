use std::fmt::{Debug, Formatter};

use crate::{coup::rep::Move, notation::fen::FEN, types::Color};

#[derive(Debug, Clone, PartialEq)]
pub enum Delim {
    Start,
    End
}

#[derive(Debug, Clone, PartialEq)]
pub enum Reason {
    /// Checkmate by the given color
    Winner(Color),
    /// Draw for any reason
    Stalemate,
    /// Aborted for unspecified reason
    Aborted,
    /// Returned from an unfinished variation
    Returned,
}

#[derive(Clone, PartialEq)]
pub enum ChessAction {
    NewGame,
    Halted(Reason),
    Variation(Delim),
    Setup(FEN),
    Make(Move),
}

impl Debug for ChessAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessAction::NewGame => write!(f, "NewGame"),
            ChessAction::Halted(egs) => write!(f, "EndGame({:?})", egs),
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
