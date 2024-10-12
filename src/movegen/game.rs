#![allow(dead_code, unused_imports)]

use crate::{board::{pieceboard::PieceBoard, Chess, Query}, constants::{Color, START_POSITION_FEN}, movegen::halfply::HalfPly};
use std::fmt::{self, Display, Formatter};


/// A Line is a single sequence of moves starting from the provided initial position (via FEN, by
/// default, the standard start position).
#[derive(Clone, Debug)]
struct Line {
    initial_position: String, // TODO: I should probably have a FEN type.
    halfplies: Vec<HalfPly>,
}

impl Default for Line {
    fn default() -> Self {
        Line {
            initial_position: START_POSITION_FEN.to_string(),
            halfplies: Vec::new(),
        }
    }
}

struct Variation {
    parent: Box<Line>,
    initial_position: String,
    previous_move_index: usize,
    continuation: Line,
}

struct Game {
    mainline: Line,
    variations: Vec<Variation>,
}

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



impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // PieceBoard is used to do the conversion to PGN since it's very simple, and `Chess` will
        // do.
        write!(f, "{}", self.to_pgn())
    }
}

// FIXME: This is in desparate want of that Notation type.
impl From<Vec<&str>> for Line {
    fn from(moves: Vec<&str>) -> Self {
        let mut line = Line::default();
        for move_str in moves {
            let halfply = HalfPly::from(move_str);
            line.push(halfply);
        }
        line
    }
}

impl Line {
    fn push(&mut self, halfply: HalfPly) {
        self.halfplies.push(halfply);
    }

    fn pop(&mut self) -> Option<HalfPly> {
        self.halfplies.pop()
    }

    fn current_move(&self) -> Option<HalfPly> {
        match self.halfplies.last() {
            Some(halfply) => Some(halfply.clone()),
            None => None,
        }
    }

    fn current_position(&self) -> impl Chess {
        let mut board = PieceBoard::from_fen(&self.initial_position);
        for halfply in &self.halfplies {
            board = board.make(halfply.into());
        }
        board
    }

    fn current_color(&self) -> Color {
        if self.halfplies() % 2 == 0 {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }

    fn halfplies(&self) -> usize {
        self.halfplies.len()
    }

    fn to_pgn(&self) -> String {
        // PieceBoard is used to do the conversion to PGN since it's very simple, any `Chess` will
        // do.
        let board = PieceBoard::from_fen(&self.initial_position);
        self.to_pgn_with_context(&board)
    }

    fn to_pgn_with_context<C>(&self, context: &C) -> String where C: Query {
        //FIXME: The context is _completely_ wrong here, since I'm not actually making these moves.
        //UCI moves and PGN moves are both ambiguous, you have to calculate to unpack the
        //boardstate, which is understandable but pretty annoying.
        let mut pgn = String::new();
        let line = self.halfplies.clone();
        for (move_number, halfply) in line.into_iter().enumerate() {
            if move_number % 2 == 0 {
                pgn.push_str(&format!("{}. {}", move_number / 2 + 1, &halfply.to_pgn(context)));
            } else {
                pgn.push_str(&format!(" {}\n", &halfply.to_pgn(context)));
            }
        }
        pgn
    }

    /// Clones the line into a new line, suitable for making a variation from the current move.
    /// DEPRECATED: This is going to be a whole struct thing...
    fn make_variation(&self) -> Line {
        let mut variation = Line::default();
        for halfply in &self.halfplies {
            variation.push(halfply.clone());
        }
        variation
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::pieceboard::PieceBoard;
    use crate::board::Chess;
    use crate::constants::START_POSITION_FEN;


    #[test]
    fn renders_game_pgn_correctly() {
        let line = Line::from(vec![
            "d2d4",
            "d7d5",
            "c1f4",
            "g8f6",
            "g1f3",
            "e7e6",
        ]);
        assert_eq!(line.to_pgn(), "1. d4 d5\n2. Bf4 Nf6\n3. Nf3 e6\n");
    }

    #[test]
    fn renders_game_with_check_correctly() {
        /*
        let line = Line::from(vec![
        ]);
        */
    }

    #[test]
    fn renders_game_with_checkmate_correctly() {
    }

    #[test]
    fn renders_game_with_longcastling_correctly() {
    }

    #[test]
    fn renders_game_with_promotion_correctly() {
    }

    #[test]
    fn renders_game_with_en_passant_correctly() {
    }

    #[test]
    fn renders_game_with_disambiguation_correctly() {
    }

    #[test]
    fn renders_game_with_capture_correctly() {
    }

    #[test]
    fn renders_game_with_shortcastling_correctly() {
    }

    #[test]
    fn renders_game_with_promotion_and_checkmate_correctly() {
    }


    #[test]
    fn line_push_pop() {
        let mut line = Line::default();
        let halfply = HalfPly::from("e2e4");
        line.push(halfply.clone());
        assert_eq!(line.current_move(), Some(halfply.clone()));
        assert_eq!(line.pop(), Some(halfply.clone()));
        assert_eq!(line.pop(), None);
    }

    #[test]
    fn line_current_color() {
        let mut line = Line::default();
        let halfply = HalfPly::from("e2e4");
        assert_eq!(line.current_color(), Color::WHITE);
        line.push(halfply.clone());
        assert_eq!(line.current_color(), Color::BLACK);
        line.push(halfply.clone());
        assert_eq!(line.current_color(), Color::WHITE);
        line.push(halfply.clone());
        assert_eq!(line.current_color(), Color::BLACK);
    }

    #[test]
    fn line_to_pgn() {
        let mut line = Line::default();
        let halfply = HalfPly::from("e2e4");
        line.push(halfply.clone());
        assert_eq!(line.to_pgn(), "1. e4");
        let halfply = HalfPly::from("e7e5");
        line.push(halfply.clone());
        assert_eq!(line.to_pgn(), "1. e4 e5\n");
    }

    #[test]
    fn line_make_variation() {
        let mut line = Line::default();
        let halfply = HalfPly::from("e2e4");
        line.push(halfply.clone());
        let variation = line.make_variation();
        assert_eq!(variation.current_move(), Some(halfply.clone()));
    }
}

