#![cfg(test)]

use quickcheck::Arbitrary;

use super::Game;

impl Arbitrary for Game {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // Play a random number of random moves in a game. This is exceedingly unlikely to result in checkmate so I don't bother to check, 
        let mut num_moves = u8::arbitrary(g);
        let mut game = Game::start_position();
        while num_moves != 0 {
            let available_moves = game.position.moves();
            let mov_idx = usize::arbitrary(g) % available_moves.len();
            let mov = available_moves.moves.concat()[mov_idx];
            game.make(mov);
            num_moves -= 1;
        }
        game
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn game_can_be_generated_arbitrarily(g: Game) -> bool {
        // just verifies the generator doesn't panic
        let other = g.clone();
        g == other
    } 
}