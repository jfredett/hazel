use std::fmt::Display;
use std::str::SplitWhitespace;

use crate::notation::fen::castle_rights::CastleRights;
use crate::notation::*;
use crate::types::Color;

#[derive(Debug, Clone)]
pub struct PositionMetadata {
    pub side_to_move: Color,
    pub castling: CastleRights,
    // The index of the square containing the en passant target square, or None if there is none
    pub en_passant: Option<Square>,
    pub halfmove_clock: usize,
    pub fullmove_number: usize,
}

impl Default for PositionMetadata {
    fn default() -> Self {
        Self {
            side_to_move: Color::WHITE,
            castling: CastleRights {
                white_short: true,
                white_long: true,
                black_short: true,
                black_long: true,
            },
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}

impl Display for PositionMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ep_sq = match self.en_passant {
            Some(sq) => sq.to_string(),
            None => "-".to_string(),
        };

        write!(f, "{} {} {} {} {}",
            self.side_to_move,
            self.castling,
            ep_sq,
            self.halfmove_clock,
            self.fullmove_number,
        )
    }
}

impl PositionMetadata {
    pub fn parse(&mut self, parts: &mut SplitWhitespace<'_>) {
        let side_to_move = parts.next().unwrap();
        let castling = parts.next().unwrap();
        let en_passant = parts.next().unwrap();
        let halfmove_clock = parts.next().unwrap();
        let fullmove_number = parts.next().unwrap();

        let side_to_move = match side_to_move {
            "w" => Color::WHITE,
            "b" => Color::BLACK,
            _ => panic!("Invalid side to move"),
        };

        let castling = CastleRights {
            white_short: castling.contains('K'),
            white_long: castling.contains('Q'),
            black_short: castling.contains('k'),
            black_long: castling.contains('q'),
        };

        let en_passant = match en_passant {
            "-" => None,
            square => Some(Square::new(square.parse().unwrap())),
        };

        self.side_to_move = side_to_move;
        self.castling = castling;
        self.en_passant = en_passant;
        self.halfmove_clock = halfmove_clock.parse().unwrap();
        self.fullmove_number = fullmove_number.parse().unwrap();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let mut metadata = PositionMetadata::default();
        let mut parts = "w KQkq - 0 1".split_whitespace();
        metadata.parse(&mut parts);

        assert_eq!(metadata.side_to_move, Color::WHITE);
        assert_eq!(metadata.castling.white_short, true);
        assert_eq!(metadata.castling.white_long, true);
        assert_eq!(metadata.castling.black_short, true);
        assert_eq!(metadata.castling.black_long, true);
        assert_eq!(metadata.en_passant, None);
        assert_eq!(metadata.halfmove_clock, 0);
        assert_eq!(metadata.fullmove_number, 1);
    }

    #[test]
    fn print() {
        let metadata = PositionMetadata {
            side_to_move: Color::WHITE,
            castling: CastleRights {
                white_short: true,
                white_long: true,
                black_short: true,
                black_long: true,
            },
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        };

        assert_eq!(metadata.to_string(), "w KQkq - 0 1");
    }
}
