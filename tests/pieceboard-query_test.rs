mod fixtures;
use hazel_representation::board::simple::PieceBoard;

use fixtures::POS2_KIWIPETE_FEN;

use hazel_representation::extensions::query::display_board;
use hazel_core::square::*;
use hazel_core::interface::Query;

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

    use hazel_core::ben::BEN;

    use super::*;

    #[test]
    fn to_fen_test() {
        let mut p = PieceBoard::default();
        p.set_startpos();

        let actual = hazel_core::interface::query::to_fen_position(&p);
        let expected = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn to_fen_test_kiwipete() {
        let mut p = PieceBoard::default();
        p.set_fen(BEN::new(POS2_KIWIPETE_FEN));

        let actual = hazel_core::interface::query::to_fen_position(&p);
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
