use super::chess::ChessAction;

pub enum GameAction {
    // Seeks forward/back by specified number of actions
    Seek(isize),
    // Resets to state after the beginning of the previous game.
    Restart,
    Play(ChessAction),
    Comment(String)
}
