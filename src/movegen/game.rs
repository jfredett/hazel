#![allow(dead_code, unused_imports)]

use crate::{board::Query, constants::Color, movegen::halfply::HalfPly};


// A single line.
struct Line {
    halfplies: Vec<HalfPly>,
}

// Analysis
struct Analysis {
    lines: Vec<Line>,
}

impl Default for Line {
    fn default() -> Self {
        Line {
            halfplies: Vec::new(),
        }
    }
}

impl Default for Analysis {
    fn default() -> Self {
        Analysis {
            lines: Vec::new(),
        }
    }
}

impl Line {
    fn push(&mut self, halfply: HalfPly) {
        self.halfplies.push(halfply);
    }

    fn pop(&mut self) -> Option<HalfPly> {
        self.halfplies.pop()
    }

    fn current_move(&self) -> Option<&HalfPly> {
        self.halfplies.last()
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

    fn to_pgn<C>(&self, context: &C) -> String where C: Query {
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
    fn make_variation(&self) -> Line {
        let mut variation = Line::default();
        for halfply in &self.halfplies {
            variation.push(halfply.clone());
        }
        variation
    }
}
