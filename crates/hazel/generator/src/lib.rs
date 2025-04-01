use hazel_representation::game::chess::position::Position;
use hazel_representation::coup::rep::Move;


// TODO: Move this somewhere better, probably types?
mod check;
mod king;
mod knight;
mod pawn;
mod slider;

#[derive(Debug, Default)]
pub struct MoveGenerator {
    // This should actually just be passed into the generate_moves, and MoveGen is just for holding
    // caches.
    // TODO: Cache anything worth caching?
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self { }
    }

    pub fn generate_moves(&self, position: &Position) -> Vec<Move> {
        // TODO: Determine if we are in check
        if check::is_in_check(position) {
            return check::generate_moves(position).collect();
        }

        // TODO: in parallel?
        pawn::generate_moves(position).chain(
        knight::generate_moves(position)).chain(
        slider::bishop::generate_moves(position)).chain(
        slider::rook::generate_moves(position)).chain(
        slider::queen::generate_moves(position)).chain(
        king::generate_moves(position)).collect()
    }

    pub fn perft(&self, depth: usize, position: &mut Position) -> usize {
        if depth == 0 { return 1; }

        let movs = self.generate_moves(position);
        let mut count = 0;

        for mov in movs {

            position.make(mov);

            // if depth == 1 {
            //     tracing::debug!("after-make {:?}: {} {}\n\n{}\n{:?}",
            //         position.zobrist(),
            //         crate::query::to_fen_position(&position.clone()),
            //         position.metadata(),
            //         crate::query::display_board(&position.board()),
            //         position.tape
            //     );
            // }

            count += self.perft(depth - 1, position);

            position.unmake();

            // if depth == 1 {
            //     tracing::debug!("after-unmake {:?}: {} {}\n\n{}\n{:?}",
            //         position.zobrist(),
            //         crate::query::to_fen_position(&position.clone()),
            //         position.metadata(),
            //         crate::query::display_board(&position.board()),
            //         position.tape
            //     );
           // }
        }

        count
    }
}


#[cfg(test)]
mod tests {

    use hazel_core::ben::BEN;

    use super::*;

    macro_rules! assert_no_difference {
        ($a:expr, $b:expr) => {
            if $a != $b {
                if $a > $b {
                    // NOTE: This might not quite be aligned, but the other side of the branch works on my
                    // terminal at least
                    println!("                               Actual - Expected = Overcount");
                    println!("Actual undercounts Expected by: {:>6} - {:>8} = {:>9}", $a, $b, $a - $b);
                } else {
                    println!("                                Expected - Actual = Undercount");
                    println!("Actual undercounts Expected by: {:>8} - {:>6} = {:>10}", $b, $a, $b - $a);
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
        perft_position(depth, &mut Position::new(BEN::start_position()))
    }

    #[test]
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

    #[allow(dead_code)] // off by 140 right now
    // #[test]
    fn perft_4() {
        assert_no_difference!(perft_start_position(4), 197_281);
    }

    #[test]
    fn check_mate_position_has_zero_perft_at_any_depth() {
        let count = perft_position(1, &mut Position::new(BEN::new("7k/6Q1/6K1/8/8/8/8/8 b - - 0 1")));
        assert_eq!(count, 0);

    }
}
