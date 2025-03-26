use crate::{ben::BEN, color::Color, occupant::Occupant, piece::Piece, position_metadata::PositionMetadata, square::Square, START_POSITION_FEN};
use crate::interface::{Alter, Alteration};

impl BEN {
    pub fn to_alterations(&self) -> impl Iterator<Item = Alteration> {
        crate::interface::query::to_alterations(self)
    }

    // TODO: Nom would be nice here, but because I compile directly to alterations, it's hard to
    // find the right place to run this, since `hazel-parser` is included in `core`.
    pub fn compile(fen: &str) -> impl Iterator<Item = Alteration> {
        let mut alterations = vec![];
        let mut cursor = Square::by_rank_and_file();
        cursor.downward();
        let mut chunks = fen.split_whitespace();

        let configuration = chunks.next().expect("Invalid position configuration");

        for c in configuration.chars() {
            if cursor.is_done() { break; }

            match c {
                '1'..='8' => {
                    let skip = c.to_digit(10).unwrap() as usize;
                    for _ in 0..skip { cursor.next(); }
                }
                '/' => {
                    continue;
                }
                c => {
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

        let mut metadata = PositionMetadata::default();
        metadata.parse(&mut chunks);
        alterations.push(Alteration::inform(&metadata));

        alterations.into_iter()
    }

    // This should be an extension called 'parse' and live in parser
    pub fn new(pos: &str) -> Self {
        let alterations = Self::compile(pos);
        let mut ret = Self::empty();
        for alter in alterations {
            ret.alter_mut(alter);
        }
        ret
    }

    pub fn start_position() -> Self {
        Self::new(START_POSITION_FEN)
    }
}

