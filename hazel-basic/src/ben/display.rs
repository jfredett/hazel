use crate::ben::BEN;

// this has to live in basic, which means to_fen_position has to live there too, but it uses query,
// so maybe query and them need to be all the way down in at the bottom?
impl std::fmt::Display for BEN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::interface::query::to_fen_position(self))
    }
}

