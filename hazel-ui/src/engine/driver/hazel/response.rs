use crate::{engine::uci::UCIMessage, game::position::Position};

use super::Hazel;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum HazelResponse {
    #[default] Silence,
    UCIResponse(UCIMessage),
    Debug(Hazel),
    Position(Option<Position>)
}
