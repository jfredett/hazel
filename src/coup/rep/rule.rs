
use std::fmt::{Debug, Formatter};
use crate::notation::*;
use crate::board::interface::{Alteration, Alter};
use crate::coup::rep::Move;
use crate::types::Color;
use crate::board::simple::PieceBoard;
use crate::notation::fen::FEN;


#[derive(Debug, Clone, PartialEq)]
pub enum ChessEvent {
    // Chess Metadata
    WhiteShortCastle(bool),
    WhiteLongCastle(bool),
    BlackShortCastle(bool),
    BlackLongCastle(bool),
    EP(Square),
    Check(bool),
}

impl Into<u32> for ChessEvent {
    fn into(self) -> u32 {
        todo!()
    }
}

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
    Tag(ChessEvent)
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
            ChessAction::Tag(ce) => write!(f, "Tag({:?})", ce),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Game {
    action_log: Vec<ChessAction>
}

impl IntoIterator for &Game {
    type Item = ChessAction;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.action_log.clone().into_iter()
    }
}

impl Game {
    pub fn push(&mut self, action: ChessAction) {
        self.action_log.push(action);
    }

    pub fn pop(&mut self) -> Option<ChessAction> {
        self.action_log.pop()
    }
}

impl From<&Game> for Vec<Alteration> {
    fn from(game: &Game) -> Vec<Alteration> {
        let mut ret = vec![];
        for action in game.into_iter() {
            // we need a board to track context, so we choose a simple one.
            // We'll recompute it's state each loop for simplcity.
            let mut board = PieceBoard::default();
            for a in ret.clone() { board.alter_mut(a); }

            let alters = match action {
                ChessAction::NewGame => vec![Alteration::Clear],
                ChessAction::EndGame(_eg_state) => vec![],
                ChessAction::Variation(_) => vec![],
                // TODO: These should become `From` implementations so I just `into` here
                ChessAction::Setup(fen) => {
                    let mut inner_ret = vec![];
                    inner_ret.extend(fen.compile());
                    // TODO: Something to convert the metadata -> tags
                    inner_ret
                },
                // TODO: For this, I need `into_alter`, not just From, since i need a context
                ChessAction::Make(mov) => mov.compile(&board),
                ChessAction::Tag(ChessEvent::WhiteShortCastle(b)) => vec![Alteration::Tag(b as u32)],
                ChessAction::Tag(ChessEvent::WhiteLongCastle(b)) => vec![Alteration::Tag(b as u32)],
                ChessAction::Tag(ChessEvent::BlackShortCastle(b)) => vec![Alteration::Tag(b as u32)],
                ChessAction::Tag(ChessEvent::BlackLongCastle(b)) => vec![Alteration::Tag(b as u32)],
                ChessAction::Tag(ChessEvent::EP(sq)) => vec![Alteration::Tag(sq.index() as u32)],
                ChessAction::Tag(ChessEvent::Check(b)) => vec![Alteration::Tag(b as u32)],
            };

            ret.extend(alters);
        }
        ret
    }
}

impl From<Game> for Vec<Alteration> {
    fn from(game: Game) -> Vec<Alteration> {
        Vec::from(&game)
    }
}

#[cfg(test)]
mod tests {
    use crate::{coup::rep::MoveType, types::Occupant};
    use crate::board::interface::*;

    use super::*;

    #[test]
    fn simple_game() {
        let mut game = Game::default();
        game.push(ChessAction::NewGame);
        game.push(ChessAction::Setup(FEN::start_position()));
        game.push(ChessAction::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN)));
        game.push(ChessAction::Tag(ChessEvent::EP(D3)));

        let alters: Vec<Alteration> = game.clone().into();

        dbg!(&alters);

        assert_eq!(alters.len(), 37);

        let mut b = PieceBoard::default();
        for alter in alters {
            b.alter_mut(alter);
        }

        assert_eq!(b.get(D2), Occupant::empty());
        assert_eq!(b.get(D4), Occupant::white_pawn());
    }
}
