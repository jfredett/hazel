use crate::{
    notation::{*, fen::FEN},
    types::Occupant,
    util::charray::{Origin, Charray},
};

/// implementing Query states that the implementor can provide the occupant of a square on the
/// board using standard 'index' notation with the 0th square being a1 and the 63rd square being
/// h8.
pub trait Query {
    fn get<S>(&self, square: S) -> Occupant where S : SquareNotation;
}

lazy_static! {
    /// A Charray texture to display the empty board
    static ref TEXTURE: Vec<&'static str> = vec![
         "8 . . . . . . . .",
         "7 . . . . . . . .",
         "6 . . . . . . . .",
         "5 . . . . . . . .",
         "4 . . . . . . . .",
         "3 . . . . . . . .",
         "2 . . . . . . . .",
         "1 . . . . . . . .",
         "  a b c d e f g h"
    ];
    static ref EMPTY_BOARD : Charray<9,17> = Charray::new().with_texture(TEXTURE.to_vec());
}

/// For a variety of, I'm sure, very good reasons, I can't provide a generic `impl Debug for T where T: Query`.
/// Something about orphans, I'm sure there is some kind of hack.
/// For now, this does what's needed.
pub fn display_board(board: &impl Query) -> String {
    let mut charray = EMPTY_BOARD.clone();
    charray.set_origin(Origin::BottomLeft);

    for s in Square::by_rank_and_file() {
        let occ = board.get(s);
        charray.set(1 + s.rank(), 2 * s.file() + 2, occ.to_string().as_bytes()[0]);
    }

    format!("{}", charray.to_string())
}

pub fn to_fen(board: &impl Query) -> FEN {
    let mut f = String::new();
    let mut empty = 0;

    /*
    for s in Square::by_rank_and_file() {
        if s.index() % 8 == 0 && s != A1 {
            if empty != 0 {
                f.push_str(&empty.to_string());
                empty = 0;
            }
            f.push_str("/");
        }
        let occ = board.get(s);
        match occ {
            Occupant::Empty => empty += 1,
            _ => {
                if empty > 0 {
                    f.push_str(&empty.to_string());
                    empty = 0;
                }
                f.push_str(&occ.to_string());
            }
        }

        if empty == 8 {
            f.push_str("8");
            empty = 0;
        }
    }
    */

    for s in Square::by_rank_and_file().downward() {
        let occ = board.get(s);
        match occ {
            Occupant::Empty => empty += 1,
            _ => {
                if empty != 0 {
                    f.push_str(&empty.to_string());
                    empty = 0;
                }
                f.push_str(&occ.to_string());
            }
        }

        if s.file() == 7 && s != A8 {
            if empty != 0 {
                f.push_str(&empty.to_string());
                empty = 0;
            }
            f.push_str("/");
        }
    }

    f.pop(); // remove the last slash

    FEN::with_default_metadata(&f)
}

#[cfg(test)]
mod tests {
    use crate::board::simple::PieceBoard;
    use crate::constants::POS2_KIWIPETE_FEN;


    use super::*;

    #[test]
    fn display_test() {
        let mut p = PieceBoard::default();
        p.set_startpos();

        let actual = display_board(&p);
        let expected = "8 r n b q k b n r
7 p p p p p p p p
6 . . . . . . . .
5 . . . . . . . .
4 . . . . . . . .
3 . . . . . . . .
2 P P P P P P P P
1 R N B Q K B N R
  a b c d e f g h
";

        println!("{}", actual);
        println!("{}", expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn to_fen_test() {
        let mut p = PieceBoard::default();
        p.set_startpos();

        let actual = to_fen(&p);
        let expected = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn to_fen_test_kiwipete() {
        let mut p = PieceBoard::default();
        p.set_fen(&FEN::new(POS2_KIWIPETE_FEN));

        let actual = to_fen(&p);
        let expected = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

        assert_eq!(format!("{}", actual), expected);
    }
}
