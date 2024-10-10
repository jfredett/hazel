use crate::board::occupant::Occupant;
use crate::movegen::alteration::Alteration;
use crate::movement::Move;

/// A Board is an object which can be altered and unaltered by a Move. Both methods are specified,
/// but if the underlying representation is capable of using interpreting Alterations (i.e., is
/// Alterable) and can be queried for the Occupant at a given index (i.e., is Queryable), then the
/// Board has a canonical implementation of make and unmake via Alterable.
pub trait Board where Self: Sized {
    fn make(&self, mov: Move) -> Self;
    fn unmake(&self, mov: Move) -> Self;

    fn make_mut(&mut self, mov: Move) -> &mut Self {
        *self = self.make(mov);
        self
    }

    fn unmake_mut(&mut self, mov: Move) -> &mut Self {
        *self = self.unmake(mov);
        self
    }
}

pub trait Alter where Self: Sized {
    fn alter(&self, mov: Alteration) -> Self;

    fn alter_mut(&mut self, mov: Alteration) -> &mut Self {
        *self = self.alter(mov);
        self
    }
}

pub trait Query {
    // TODO: Eventually this should take a Notation object, index is just a notation
    fn get(&self, index: usize) -> Occupant;
}

/// The canonical implementation of Board for any type which is Alterable and Queryable. The
/// algorithm is straightforward:
/// 1. Compile the move in the context of the board, yielding a vector of Alterations.
/// 2. Apply each alteration in sequence to the board, returning the final board state.
///
/// Unmaking is trivial because Alterations are reversible. It's the same algorithm, but applying
/// `inverse` first.
impl<T> Board for T where T: Alter + Query + Clone {
    fn make(&self, mov: Move) -> T {
        let alterations = mov.compile(self);
        alterations.iter().fold(self.clone(), |board, alteration| board.alter(alteration.clone()))
    }

    fn unmake(&self, mov: Move) -> T {
        let alterations = mov.compile(self);
        alterations.iter().fold(self.clone(), |board, alteration| board.alter(alteration.inverse()))
    }
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
    use crate::{board::pieceboard::PieceBoard, constants::POS2_KIWIPETE_FEN};

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
