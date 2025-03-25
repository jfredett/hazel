use crate::{alter, game::ChessGame, query, Alter, Query};

use super::*;

// this I think is probably closer to 'Position', but I'm not there yet.
impl<Q> From<ChessGame<Q>> for BEN where Q : Query + Alter + Default + Clone {
    fn from(game: ChessGame<Q>) -> Self {
        let mut ben : BEN = alter::setup(query::to_alterations(&game.rep));
        ben.set_metadata(game.metadata);
        ben
    }
}

impl<T> From<BEN> for ChessGame<T> where T :  Alter + Query + Default + Clone {
    fn from(ben: BEN) -> Self {
        let rep = alter::setup(ben.to_alterations());

        ChessGame {
            rep,
            metadata: ben.metadata(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
