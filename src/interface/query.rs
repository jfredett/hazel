use ben::BEN;

use crate::{
    notation::*,
    types::{pextboard::attacks_for, Color, Direction, Occupant, Piece},
    util::charray::{Charray, Origin},
};


/*
*
*  Idea:
*
*  1. Query should have a sublanguage, similar to Alter, that specifies how one queries against the
*     boardrep.
*  2. Minimally, a boardrep _must_ support `Get`, and then all other items should be able to
*     'coompile' to some sequence of `Get` operations. E.g., a 'OccupantsOf(File)` might return a
*     vector of Occupants filtered to the specific file, this is just a sequence of `Get`
*     operations.
*  3. Different representations can implement different clusters of requests in different ways,
*     perhaps even async to allow for some amount of batching? The result should always be an
*     iterator over result objects which contain info about the square, it's occupant, etc.
*
*  Minimally it should support:
*  Primitives:
*   Get(Sq) -> Occupant // primitive
*   Find(Occupant) -> Vec<Sq> // primitive
*  Advanced
*   AttacksFor(Piece, Sq) -> Vec<(Sq, Occupant)> // occupant should never be Empty
*       - Get(Sq.left_oblique())
*       - Get(Sq.right_oblique())
*   NonAttackMovesFor(Piece, Sq) -> Vec<(Sq, _)>          // occupant should always be Empty
*   MoveFor(Piece, Sq) -> Vec<(Sq, Occupant)>
*       - AttacksFor(Piece, Sq) + NonAttackMovesFor(Piece, Sq)
*       - NonAttackMovesFor(Piece, Sq) + AttacksFor(Piece, Sq)
*   OccupantsOf(Start, Direction) -> Vec<(Sq, Occupant)>
*       - for i in 0..8 { Get(Start + i * Direction) }
*   OccupiedSquares(Color) -> Vec<Sq>
*       - for sq in all_squares { Get(sq) if color matches color }
*
* All these exist in some 'language' (probably just an enum) and a particular represnetation can do
* whatever it likes to implement the more advanced items, but should minimimally provide
* implementations of all the primitives. Different reps will do better with different subcommands,
* and a common benchmarking system can be built to find the 'best' represnetations for each type of
* thing.
*
* Ideally those benchmarks measure:
*   1. CPU/GPU use ("Compute")
*   2. Memory use ("Memory")
*   3. ELO of a 'pure' engine using only that representation.
*   4. Latency if the rep if it's wrapped in a process.
*
* The idea is that some constellation of representations should be more than the sum of their
* parts, and having a single framework for comparing them will allow for the best possible
* combination to be found.
*
* `get` can remain as a legacy method for now, but eventually it should be replaced by `query`
* which takes a Query enum variant and returns an iterator over the results. A result is a struct
* that optionally contains an occupant and square.
*
* The backend implementation can then eventually be proxied into a sort of 'meta' representation
* that uses internal representations at various states, updating them to the current state only as
* needed, and often by copying from other states.
*
* So we might have a simple piece-list rep for optimizing the `OccupiedSquares` query and general
* gameplay, and a bitboard representation for movegen. Then, when we need to calculate, say, a
* piece-graph, we copy the state as a BEN from the Piece-list model which is ostensibly quite fast,
* and then update our graph-rep board from the BEN, then do our graph calculations.
*
* The metarep is responsible for keeping everything aligned and can run in it's own process that
* others communicate to
*
*
* More notes: 16-FEB-2025 0023
*
* I think I need this to work a bit like `messagefor` w/ Witch, but no async stuff. I want to
* contain the logic local to each piece, because how a piece moves 'belongs' to the piece, and it
* prevents the big, impossible to maintain move-logic function.
*
* However, for optimization, I need to do as much as possible in the representation, so that I can
* use bitboards where it makes sense, etc.
*
* I think this really comes down to messages you send to `position`, which then calls the piece
* code which also queries position. Position is responsible for finding the moves and mostly
* returning square/occupant pairs in iterators. Ideally it mostly uses the iterator API to
* manipulate the position, which should lend itself to `rayon` paralellization.
*
* So the movegen would then work like
*
*  MoveGenerator::generate_moves "Generate all legal moves for the position"
*       Position -> pawn::generate_moves(position, color) "Generate all legal moves for all pawns of the given color in this position"
*           Pawn::generate_moves(position, color, square) "For this pawn, find all it's legal moves"
*               <- returns a list of squares and move metadata. The squares are the target squares,
*                   the move metadata describes if it's a capture, promotion, EP, etc.
*           ... n times
*       Position -> bishop::generate_moves(position, color) "Generate all legal moves for all the bishops of the given color in this position"
*       ...
*
*  That way the caching lives at the top layer in the movegen, the position can be pulled from a
*  position cache in MG or from some other source (a PGN/whatever). it becomes the main
*  representation of a position, internally it stores whatever representations it needs, and
*  supports the more advanced query methods. I'll bake it in directly, but I do eventually want
*  it's API to be hidden behind `Query` instead of directly being used.
*
*  To do all this, I think the plan is:
*
*  1. Extract Position out from it's current home to it's own file.
*  2. Implement the Query methods below there, adding whatever representation caching makes sense.
*  3. I think module-per-piece should be fine here, I don't need types, these already exist in the
*     enum, and I can maybe have a generic `moves_for` on the enum that dispatches to the correct
*     module.
*
*/




/// implementing Query states that the implementor can provide the occupant of a square on the
/// board using standard 'index' notation with the 0th square being a1 and the 63rd square being
/// h8.
pub trait Query {
    fn get(&self, square: impl Into<Square>) -> Occupant;

    fn is_empty(&self, square: impl Into<Square>) -> bool {
        self.get(square).is_empty()
    }

    fn is_occupied(&self, square: impl Into<Square>) -> bool {
        self.get(square).is_occupied()
    }

    // NOTE: (Square, Occupant) is essentially 'placed piece'
    //
    fn find(&self, occ: &Occupant) -> impl Iterator<Item = (Square, Occupant)> {
        Square::by_rank_and_file().filter(|s| self.get(*s) == *occ).map(|s| (s, *occ))
    }

    fn attacks_for(&self, piece: Piece, square: impl Into<Square>) -> impl Iterator<Item = (Square, Occupant)> {
        vec![].into_iter()
    }

    fn non_attack_moves_for(&self, piece: Piece, square: impl Into<Square>) -> impl Iterator<Item = (Square, Occupant)> {
        vec![].into_iter()
    }

    fn moves_for(&self, piece: Piece, square: impl Into<Square>) -> impl Iterator<Item = (Square, Occupant)> {
        let sq = square.into();
        self.attacks_for(piece, sq).chain(
            self.non_attack_moves_for(piece, sq)
        )
    }

    fn occupants_along(&self, start: impl Into<Square>, direction: Direction) -> impl Iterator<Item = (Square, Occupant)> {
        vec![].into_iter()
    }

    fn occupied_squares(&self, color: Color) -> impl Iterator<Item = (Square, Occupant)> {
        vec![].into_iter()
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

pub fn to_fen_string(board: &impl Query) -> String {
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
pub fn to_fen(board: &impl Query) -> BEN {
    BEN::with_default_metadata(&to_fen_string(board))
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
        p.set_fen(&BEN::new(POS2_KIWIPETE_FEN));

        let actual = to_fen(&p);
        let expected = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

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

