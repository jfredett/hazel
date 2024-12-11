use crate::types::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Reason {
    /// Checkmate by the given color
    Winner(Color),
    /// Draw for any reason
    Stalemate,
    /// Aborted for unspecified reason
    Aborted,
}

