use crate::board::{Alteration, Query};
use crate::constants::EMPTY_POSITION_FEN;
use crate::types::{Color, Piece};
use crate::notation::*;
use crate::types::Occupant;



#[derive(Debug, Clone)]
pub(super) struct Position {
    position_string: String,
    position: Vec<Alteration>
}

impl<C : Query> From<C> for Position {
    fn from(c: C) -> Self {
        let mut rep = String::new();
        let mut empty = 0;
        for s in Square::fenwise() {
            let occ = c.get(s);

            if s.file() == 0 && s != A1 {
                if empty != 0 {
                    rep.push_str(&empty.to_string());
                    empty = 0;
                }
                rep.push('/');
            }

            match occ {
                Occupant::Empty => empty += 1,
                Occupant::Occupied(p, c) => {
                    if empty != 0 {
                        rep.push_str(&empty.to_string());
                        empty = 0;
                    }
                    rep.push(p.to_fen(c));
                    rep.push_str(&c.to_string());
                }
            }
        }

        Position::new(&rep)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.position_string == other.position_string
    }
}
impl Eq for Position {}

impl Default for Position {
    fn default() -> Self {
        Self::new(EMPTY_POSITION_FEN)
    }
}

impl IntoIterator for Position {
    type Item = Alteration;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.position.into_iter()
    }
}

impl Position {
    pub fn new(fen: &str) -> Self {
        let position = Self::compile(&fen);

        Self {
            position_string: fen.to_string(),
            position,
        }
    }

    fn compile(fen: &str) -> Vec<Alteration> {
        let mut alterations = Vec::new();
        let mut cursor = Square::by_rank_and_file();
        cursor.downward();
        for c in fen.chars() {
            if cursor.is_done() { break; }

            match c {
                '1'..='8' => {
                    let skip = c.to_digit(10).unwrap() as usize;
                    for _ in 0..skip { cursor.next(); }
                }
                '/' => {
                    continue;
                }
                _ => {
                    let color = if c.is_uppercase() { Color::WHITE } else { Color::BLACK };
                    let piece = match c.to_ascii_lowercase() {
                        'p' => Piece::Pawn,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'r' => Piece::Rook,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => {
                            continue;
                        },
                    };
                    let occupant = Occupant::Occupied(piece, color);
                    alterations.push(Alteration::Place { square: cursor.current_square(), occupant } );

                    cursor.next();
                }
            }

        }



        return alterations;
    }
}
