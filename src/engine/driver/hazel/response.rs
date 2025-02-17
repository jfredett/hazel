use crate::engine::uci::UCIMessage;

use super::Hazel;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum HazelResponse {
    #[default] Silence,
    UCIResponse(UCIMessage),
    Debug(Hazel)
}
