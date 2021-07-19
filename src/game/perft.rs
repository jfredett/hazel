use tracing::{instrument};

use super::*;

impl Game {
    #[instrument(skip(self))]
    pub fn perft(&mut self, depth: usize) -> u64 {
        if depth == 0 { return 1; }
        let mut count = 0;
        for m in self.moves().as_vec() {

            self.make(m);

            count += self.perft(depth - 1);

            self.unmake();
        }
        count
    }
    
    pub fn last_played(&self) -> Option<Move> {
        self.played.last().copied()
    }
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    use crate::{constants::START_POSITION_FEN, movement::MoveType};

    use super::*;
    
    fn perft_start_position(depth: usize) -> u64 {
        let mut g = Game::from_fen(START_POSITION_FEN);
        g.perft(depth)
    }
    
    #[test]
    #[traced_test]
    fn perft_start_position_to_depth_6() {
        assert_eq!(perft_start_position(1), 20);
        assert_eq!(perft_start_position(2), 400);
        assert_eq!(perft_start_position(3), 8_902);
        assert_eq!(perft_start_position(4), 197_281);
        // assert_eq!(perft_start_position(5), 4_865_609);
        // assert_eq!(perft_start_position(6), 119_060_324);
    }
}