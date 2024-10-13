use crate::types::Occupant;

/// implementing Query states that the implementor can provide the occupant of a square on the
/// board using standard 'index' notation with the 0th square being a1 and the 63rd square being
/// h8.
pub trait Query {
    // TODO: Eventually this should take a Notation object, index is just a notation
    fn get(&self, index: usize) -> Occupant;
}

/// For a variety of, I'm sure, very good reasons, I can't provide a generic `impl Debug for T where T: Query`.
/// Something about orphans, I'm sure there is some kind of hack.
/// For now, this does what's needed.
pub fn display_board(board: &impl Query) -> String {
    let mut f = String::new();

    f.push_str("\n");
    for rank in (0..=7).rev() {
        f.push_str(&format!(" {}", rank + 1));
        for file in 0..=7 {
            f.push_str(&format!(" {}", board.get(rank * 8 + file)));
        }
        f.push_str("\n");
    }
    f.push_str("   a b c d e f g h");
    f
}

pub fn to_fen(board: &impl Query) -> String {
    let mut f = String::new();
    let mut empty = 0;

    for rank in (0..=7).rev() {
        for file in 0..=7 {
            let occ = board.get(rank * 8 + file);
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
        }
        if empty > 0 {
            f.push_str(&empty.to_string());
            empty = 0;
        }
        if rank > 0 {
            f.push_str("/");
        }
    }
    f
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
        let expected = "
 8 r n b q k b n r
 7 p p p p p p p p
 6 . . . . . . . .
 5 . . . . . . . .
 4 . . . . . . . .
 3 . . . . . . . .
 2 P P P P P P P P
 1 R N B Q K B N R
   a b c d e f g h";

        assert_eq!(actual, expected);
    }

    #[test]
    fn to_fen_test() {
        let mut p = PieceBoard::default();
        p.set_startpos();

        let actual = to_fen(&p);
        let expected = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        assert_eq!(actual, expected);
    }

    #[test]
    fn to_fen_test_kiwipete() {
        let mut p = PieceBoard::default();
        p.set_fen(POS2_KIWIPETE_FEN);

        println!("{}", display_board(&p));

        let actual = to_fen(&p);
        let expected = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R";

        assert_eq!(actual, expected);
    }
}
