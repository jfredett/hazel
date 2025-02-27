use crate::{
    game::position_metadata::PositionMetadata, notation::*, types::{zobrist::Zobrist, Occupant}, util::charray::{Charray, Origin}
};

use super::Alteration;

/// implementing Query states that the implementor can provide the occupant of a square on the
/// board using standard 'index' notation with the 0th square being a1 and the 63rd square being
/// h8.
pub trait Query {
    fn get(&self, square: impl Into<Square>) -> Occupant;
    /// not every Query implementer will have metadata, that's okay, but if we have it we want to
    /// be able to use it.
    fn try_metadata(&self) -> Option<PositionMetadata> {
        None
    }

    fn is_empty(&self, square: impl Into<Square>) -> bool {
        self.get(square).is_empty()
    }

    fn is_occupied(&self, square: impl Into<Square>) -> bool {
        self.get(square).is_occupied()
    }
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

// For a variety of, I'm sure, very good reasons, I can't provide a generic `impl Debug for T
// where T: Query`. Something about orphans, I'm sure there is some kind of hack. For now, this
// does what's needed.
pub fn display_board(board: &impl Query) -> String {
    let mut charray = EMPTY_BOARD.clone();
    charray.set_origin(Origin::BottomLeft);

    for s in Square::by_rank_and_file() {
        let occ = board.get(s);
        charray.set(1 + s.rank(), 2 * s.file() + 2, occ.to_string().as_bytes()[0]);
    }

    charray.to_string()
}

pub fn to_fen_position(board: &impl Query) -> String {
    let mut f = String::default();
    let mut empty = 0;

    for s in Square::by_rank_and_file().downward() {
        let occ = board.get(s);
        if matches!(occ, Occupant::Empty) {
            empty += 1
        } else {
            if empty != 0 {
                f.push_str(&empty.to_string());
                empty = 0;
            }
            f.push_str(&occ.to_string());
        }

        if s.file() == 7 && s != A8 {
            if empty != 0 {
                f.push_str(&empty.to_string());
                empty = 0;
            }
            f.push('/');
        }
    }

    f.pop(); // remove the last slash
    f
}

pub fn to_alterations<Q>(board: &Q) -> impl Iterator<Item = Alteration> where Q : Query {
    // this should do 'clear' and 'assert(metadata)', query should require metadata query
    // functions? maybe optional?
    
    let mut ret = vec![ Alteration::Clear];


    ret.extend(
        Square::by_rank_and_file()
           .filter(|s| board.is_occupied(s))
           .map(|s| Alteration::place(s, board.get(s)) )
    );

    if let Some(metadata) = board.try_metadata() {
        ret.push(Alteration::Assert(metadata));
    }

    ret.into_iter()
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

    mod to_fen_position {
        use crate::notation::ben::BEN;

        use super::*;

        #[test]
        fn to_fen_test() {
            let mut p = PieceBoard::default();
            p.set_startpos();

            let actual = to_fen_position(&p);
            let expected = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

            assert_eq!(format!("{}", actual), expected);
        }

        #[test]
        fn to_fen_test_kiwipete() {
            let mut p = PieceBoard::default();
            p.set_fen(BEN::new(POS2_KIWIPETE_FEN));

            let actual = to_fen_position(&p);
            let expected = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R";

            assert_eq!(format!("{}", actual), expected);
        }

        #[test]
        fn is_empty() {
            let mut p = PieceBoard::default();
            p.set_startpos();

            assert!(p.is_empty(A3));
            assert!(!p.is_empty(A2));
        }

        #[test]
        fn is_occupied() {
            let mut p = PieceBoard::default();
            p.set_startpos();

            assert!(!p.is_occupied(A3));
            assert!(p.is_occupied(A2));
        }
    }
}
