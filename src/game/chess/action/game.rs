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
        if let Some(value) = value {
            GameAction::Set(key_s, Some(value.into()))
        } else {
            GameAction::Set(key_s, None)
        }
    }

    pub fn comment(comment: impl Into<String>) -> Self {
        GameAction::Comment(comment.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_action_seek() {
        let action = GameAction::seek(1);
        match action {
            GameAction::Seek(amount) => assert_eq!(amount, 1),
            _ => panic!("Expected GameAction::Seek")
        }
    }

    #[test]
    fn game_action_jump() {
        let action = GameAction::jump(-1);
        match action {
            GameAction::Jump(amount) => assert_eq!(amount, -1),
            _ => panic!("Expected GameAction::Jump")
        }
    }

    #[test]
    fn game_action_ponder() {
        let action = GameAction::ponder();
        match action {
            GameAction::Ponder => (),
            _ => panic!("Expected GameAction::Ponder")
        }
    }

    #[test]
    fn game_action_idle() {
        let action = GameAction::idle();
        match action {
            GameAction::Idle => (),
            _ => panic!("Expected GameAction::Idle")
        }
    }

    #[test]
    fn game_action_comment() {
        let action = GameAction::comment("comment");
        match action {
            GameAction::Comment(comment) => assert_eq!(comment, "comment"),
            _ => panic!("Expected GameAction::Comment")
        }
    }

    #[test]
    fn game_action_set() {
        let action = GameAction::set("key", Some("value"));
        match action {
            GameAction::Set(key, value) => {
                assert_eq!(key, "key");
                assert_eq!(value.unwrap(), "value");
            },
            _ => panic!("Expected GameAction::Set")
        }
    }
}
