use hazel::game::position_metadata::PositionMetadata;
use hazel::Alteration;
use hazel::notation::ben::BEN;
use hazel_basic::color::Color;
use hazel_basic::occupant::Occupant;
use hazel_basic::piece::Piece;
use hazel_basic::square::Square;

pub trait BENCompilesToAlteration {
    fn compile(fen: &str) -> impl Iterator<Item = Alteration>;
}

impl BENCompilesToAlteration for BEN {
    // FIXME: This is the only thing that needs to be in `-parser` for BEN.
    // TODO: Move this to it's own function, it should produce a ~~Log~~, ~~Tape~~, _Spell_ of alteratons
    // TODO: Nom.
    fn compile(fen: &str) -> impl Iterator<Item = Alteration> {
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
        let metadata_alterations : Vec<Alteration> = metadata.into_information();
        alterations.extend(metadata_alterations);

        alterations.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;
    use super::*;

}


