use crate::game::chess::position::Position;
use crate::coup::rep::Move;
use tracing::*;


mod pawn;
mod check;
mod slider;
mod knight;
mod king;

#[derive(Debug)]
struct MoveGenerator {
    // This should actually just be passed into the generate_moves, and MoveGen is just for holding
    // caches.
    // TODO: Cache anything worth caching?
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self { }
    }

    pub fn generate_moves(&self, position: &Position) -> impl Iterator<Item = Move> {
        // TODO: Check cache for this position

        // TODO: Determine if we are in check

        // TODO: Generate moves (maybe in parallel?
        pawn::generate_moves(position).chain(
        knight::generate_moves(position)).chain(
        slider::bishop::generate_moves(position)).chain(
        slider::rook::generate_moves(position)).chain(
        slider::queen::generate_moves(position)).chain(
        king::generate_moves(position))
    }

    pub fn perft(&self, depth: usize, position: &mut Position) -> usize {
        if depth == 0 { return 1; }

        let movs = self.generate_moves(position);
        let mut count = 0;

        for mov in movs {

            position.make(mov);

            count += self.perft(depth - 1, position);

            position.unmake();
        }

        count
    }
}


#[cfg(test)]
mod tests {
    use crate::notation::ben::BEN;

    use super::*;

    macro_rules! assert_no_difference {
        ($a:expr, $b:expr) => {
            if $a != $b {
                if $a > $b {
                    println!("Actual undercounts Expected by: {} - {} = {}", $a, $b, $a - $b);
                } else {
                    println!("Actual overcounts Expected by: {} - {} = {}", $b, $a, $b - $a);
                }
                assert!(false);
            } else {
                assert_eq!($a, $b);
            }
        }
    }

    fn perft_position(depth: usize, position: &mut Position) -> usize {
        let gen = MoveGenerator::new();
        gen.perft(depth, position)
    }

    fn perft_start_position(depth: usize) -> usize {
        perft_position(depth, &mut Position::new(BEN::start_position(), vec![]))
    }

    #[test]
    #[tracing_test::traced_test]
    fn perft_1() {
        assert_no_difference!(perft_start_position(1), 20);
    }

    #[test]
    fn perft_2() {
        assert_no_difference!(perft_start_position(2), 400);
    }

    #[test]
    fn perft_3() {
        assert_no_difference!(perft_start_position(3), 8_902);
    }

    // #[test]
    fn perft_4() {
        assert_no_difference!(perft_start_position(4), 197_281);
    }

    // #[test]
    fn check_mate_position_has_zero_perft_at_any_depth() {
        let count = perft_position(1, &mut Position::new(BEN::new("7k/6Q1/6K1/8/8/8/8/8 b - - 0 1"), vec![]));
        assert_eq!(count, 0);

    }
}
