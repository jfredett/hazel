use crate::game::{compiles_to::CompilesTo, ChessAction, Game};

pub enum GameAction {
    // Seeks forward/back by specified number of actions
    Seek(usize),
    Jump(isize),
    Ponder,
    Idle,
    Set(String, Option<String>),
    Comment(String)
}

impl GameAction {
    pub fn seek(amount: usize) -> Self {
        GameAction::Seek(amount)
    }

    pub fn jump(amount: isize) -> Self {
        GameAction::Jump(amount)
    }

    pub fn ponder() -> Self {
        GameAction::Ponder
    }

    pub fn idle() -> Self {
        GameAction::Idle
    }

    pub fn set(key: impl Into<String>, value: Option<impl Into<String>>) -> Self {
        let key_s : String = key.into();
        match value {
            Some(value) => GameAction::Set(key_s, Some(value.into())),
            None => GameAction::Set(key_s, None)
        }
    }

    pub fn comment(comment: impl Into<String>) -> Self {
        GameAction::Comment(comment.into())
    }
}
