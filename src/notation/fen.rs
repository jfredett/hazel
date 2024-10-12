#![allow(dead_code)]
use crate::{board::{occupant::Occupant, Chess}, constants::{Color, Piece}, movegen::Alteration};


// NOTE: There exists a metadata type in `Ply` which might be useful here. I intended it to be
// packed into 4 bytes, but I think I implemented it as a plain struct, either way, it can come in
// here.
// I need to start reorganizing things more aggressively, and pruning out the stuff I won't need
// anymore. It's messy in here.

#[derive(Debug)]
pub struct FEN {
    original_fen: String,
    position: Vec<Alteration>,
    side_to_move: Color,
    castling: CastleRights,
    // The index of the square containing the en passant target square, or None if there is none
    en_passant: Option<usize>,
    halfmove_clock: usize,
    to_play: Color,
    fullmove_number: usize,
}

#[derive(Debug)]
struct CastleRights {
    white_short: bool,
    white_long: bool,
    black_short: bool,
    black_long: bool,
}

impl FEN {
    pub fn new(fen: &str) -> Self {
        let mut parts = fen.split_whitespace();
        let position_str = parts.next().unwrap();
        let position = Self::compile(position_str);
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
            square => Some(square.parse().unwrap()),
        };

        Self {
            original_fen: fen.to_string(),
            position,
            side_to_move,
            castling,
            en_passant,
            halfmove_clock: halfmove_clock.parse().unwrap(),
            to_play: side_to_move,
            fullmove_number: fullmove_number.parse().unwrap(),
        }
    }

    pub fn setup<C>(&self) -> C where C : Chess {
        let mut board = C::default();
        for alteration in &self.position {
            board.alter_mut(*alteration);
        }
        board
    }

    fn compile(fen: &str) -> Vec<Alteration> {
        let mut alterations = Vec::new();
        let mut rank = 7;
        let mut file = 0;
        for c in fen.chars() {
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    file += c.to_digit(10).unwrap() as usize;
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
                            // FIXME: This is ugly.
                            alterations.push(Alteration::Place{ index: 8 * rank + file, occupant: Occupant::Empty } );
                            file += 1;
                            continue;
                        },
                    };
                    let occupant = Occupant::Occupied(piece, color);
                    alterations.push(Alteration::Place { index: 8 * rank + file, occupant } );
                    file += 1;
                }
            }
        }
        alterations
    }
}




#[cfg(test)]
mod tests {
    use crate::{board::pieceboard::PieceBoard, constants::{POS2_KIWIPETE_FEN, START_POSITION_FEN}};

    use super::*;

    #[test]
    fn fen_startpos() {
        let fen = FEN::new(START_POSITION_FEN);
        assert_eq!(fen.original_fen, START_POSITION_FEN);
        // We test the position part below in the #setup test
        assert_eq!(fen.side_to_move, Color::WHITE);
        assert_eq!(fen.castling.white_short, true);
        assert_eq!(fen.castling.white_long, true);
        assert_eq!(fen.castling.black_short, true);
        assert_eq!(fen.castling.black_long, true);
        assert_eq!(fen.en_passant, None);
        assert_eq!(fen.halfmove_clock, 0);
        assert_eq!(fen.to_play, Color::WHITE);
        assert_eq!(fen.fullmove_number, 1);
    }

    #[test]
    fn fen_kiwipete_position() {
        let fen = FEN::new(POS2_KIWIPETE_FEN);
        dbg!(&fen);
        assert_eq!(fen.original_fen, POS2_KIWIPETE_FEN);
        // We test the position part below in the #setup test
        assert_eq!(fen.side_to_move, Color::WHITE);
        assert_eq!(fen.castling.white_short, true);
        assert_eq!(fen.castling.white_long, true);
        assert_eq!(fen.castling.black_short, true);
        assert_eq!(fen.castling.black_long, true);
        assert_eq!(fen.en_passant, None);
        assert_eq!(fen.halfmove_clock, 0);
        assert_eq!(fen.to_play, Color::WHITE);
        assert_eq!(fen.fullmove_number, 1);
    }

    #[test]
    fn fen_startpos_setup() {
        let fen = FEN::new(START_POSITION_FEN);
        // This is the new implementation
        let board = fen.setup::<PieceBoard>();

        // this is the old. It can be deprecated once this is done, then this test will need to
        // change, probably.
        let expected = PieceBoard::from_fen(START_POSITION_FEN);
        assert_eq!(board, expected);
    }

    #[test]
    fn fen_kiwipete_setup() {
        let fen = FEN::new(POS2_KIWIPETE_FEN);
        // This is the new implementation
        let board = fen.setup::<PieceBoard>();

        // this is the old. It can be deprecated once this is done, then this test will need to
        // change, probably.
        let expected = PieceBoard::from_fen(POS2_KIWIPETE_FEN);
        assert_eq!(board, expected);
    }

    #[test]
    fn fen_empty_board_setup() {
        let fen = FEN::new("8/8/8/8/8/8/8/8 w KQkq - 0 1");
        let board = fen.setup::<PieceBoard>();
        let expected = PieceBoard::default();
        assert_eq!(board, expected);
    }

}

