use tracing::instrument;

use super::*;

impl Game {
    #[instrument(skip(self))]
    pub fn perft(&mut self, depth: usize) -> u64 {
        let movs = self.moves();
        if movs.is_empty() {
            0
        } else {
            if depth == 0 {
                return 1;
            }
            let mut count = 0;
            for m in self.moves().as_vec() {
                self.make(m);

                count += self.perft(depth - 1);

                self.unmake();
            }
            count
        }
    }

    pub fn last_played(&self) -> Option<Move> {
        self.played.last().copied()
    }
}

#[cfg(test)]
mod tests {

    use crate::{constants::START_POSITION_FEN, movement::MoveType};

    use super::*;

    fn perft_start_position(depth: usize) -> u64 {
        let mut g = Game::from_fen(START_POSITION_FEN);
        g.perft(depth)
    }

    #[test]
    fn check_mate_position_has_zero_perft_at_any_depth() {
        let mut g = Game::from_fen("7k/6Q1/6K1/8/8/8/8/8 b - - 0 1");
        assert_eq!(g.perft(1), 0);

    }

    use tracing_test::traced_test;

    macro_rules! assert_no_difference {
        ($a:expr, $b:expr) => {
            if $a - $b != 0 {
                if $a > $b {
                    println!("Left overcounts right by: {} - {} = {}", $a, $b, $a - $b);
                } else {
                    println!("Right overcounts left by: {} - {} = {}", $b, $a, $b - $a);
                }
                assert!(false);
            } else {
                assert_eq!($a, $b);
            }
        }
    }

    #[test]
    #[ignore] // for speed
    fn perft_start_position_to_depth_4() {
        assert_no_difference!(perft_start_position(1), 20);
        assert_no_difference!(perft_start_position(2), 400);
        assert_no_difference!(perft_start_position(3), 8_902);
        assert_no_difference!(perft_start_position(4), 197_281);
    }

    
    #[test]
    #[ignore]
    fn slow_perft() {
        assert_no_difference!(perft_start_position(5), 4_865_609);
        assert_eq!(perft_start_position(6), 119_060_324);
    }
}
