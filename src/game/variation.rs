// this is a stub still, don't worry that it doesn't work.
use crate::coup::rep::HalfPly;
use crate::game::line::Line;
use crate::constants::START_POSITION_FEN;

pub struct Variation {
    parent: Box<Line>,
    initial_position: String,
    previous_move_index: usize,
    continuation: Line,
}

/*
impl From<Line> for Variation {
    fn from(line: Line) -> Self {
        let continuation = Line::default();
        // FIXME: This is not really correct
        // continuation.initial_position = line.current_position().to_fen();

        let length = line.halfplies();
        Variation {
            parent: Box::new(line),
            initial_position: START_POSITION_FEN.to_string(),
            previous_move_index: length,
            continuation,
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn line_make_variation() {
        let mut line = Line::default();
        let halfply = HalfPly::from("e2e4");
        line.push(halfply.clone());
        let variation = line.make_variation();
        assert_eq!(variation.current_move(), Some(halfply.clone()));
    }
    */
}

