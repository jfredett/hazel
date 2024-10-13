///! various bitboard masks
// I don't know why this triggers this lint, this fails if I don't import the macro.
// Macro imports are blerg.

use crate::types::Bitboard;
use crate::types::Direction;

use crate::bitboard;

lazy_static! {
    pub static ref A_FILE : Bitboard = bitboard!("a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8");
    pub static ref B_FILE : Bitboard = A_FILE.shift(Direction::E);
    pub static ref C_FILE : Bitboard = B_FILE.shift(Direction::E);
    pub static ref D_FILE : Bitboard = C_FILE.shift(Direction::E);
    pub static ref E_FILE : Bitboard = D_FILE.shift(Direction::E);
    pub static ref F_FILE : Bitboard = E_FILE.shift(Direction::E);
    pub static ref G_FILE : Bitboard = F_FILE.shift(Direction::E);
    pub static ref H_FILE : Bitboard = G_FILE.shift(Direction::E);

    pub static ref FILE_MASKS : [Bitboard; 8] = [
        *A_FILE,
        *B_FILE,
        *C_FILE,
        *D_FILE,
        *E_FILE,
        *F_FILE,
        *G_FILE,
        *H_FILE
    ];

    pub static ref BACKRANKS : Bitboard = *RANK_1 | *RANK_8;
    pub static ref EDGES : Bitboard = *A_FILE | *H_FILE | *RANK_1 | *RANK_8;
    pub static ref CORNERS : Bitboard = bitboard!("a1", "a8", "h1", "h8");

    pub static ref RANK_1 : Bitboard = bitboard!("a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1");

    pub static ref RANK_2 : Bitboard = RANK_1.shift(Direction::N);
    pub static ref RANK_3 : Bitboard = RANK_2.shift(Direction::N);
    pub static ref RANK_4 : Bitboard = RANK_3.shift(Direction::N);
    pub static ref RANK_5 : Bitboard = RANK_4.shift(Direction::N);
    pub static ref RANK_6 : Bitboard = RANK_5.shift(Direction::N);
    pub static ref RANK_7 : Bitboard = RANK_6.shift(Direction::N);
    pub static ref RANK_8 : Bitboard = RANK_7.shift(Direction::N);

    pub static ref INITIAL_PAWN_RANK : [Bitboard; 2] = [*RANK_2, *RANK_7];

    pub static ref RANK_MASKS : [Bitboard; 8] = [
        *RANK_1,
        *RANK_2,
        *RANK_3,
        *RANK_4,
        *RANK_5,
        *RANK_6,
        *RANK_7,
        *RANK_8,
    ];

    // Taken from: https://www.chess.com/forum/view/general/which-diagonals-have-names
    //    a8-a8
    pub static ref A8_A8_DIAG : Bitboard = bitboard!("a8");
    //    a6-c8
    pub static ref A6_A8_DIAG : Bitboard = bitboard!("a6", "b7", "c8");
    //    a4-e8 "Spanish Diagonal"
    pub static ref A4_E8_DIAG : Bitboard = bitboard!("a4", "b5", "c6", "d7", "e8");
    //    a2-g8 "Italian Diagonal"
    pub static ref A2_G8_DIAG : Bitboard = bitboard!("a2", "b3", "c4", "d5", "e6", "f7", "g8");
    //    b1-h7
    pub static ref B1_H7_DIAG : Bitboard = bitboard!("b1", "c2", "d3", "e4", "f5", "g6", "h7");
    //    d1-h5
    pub static ref D1_H5_DIAG : Bitboard = bitboard!("d1", "e2", "f3", "g4", "h5");
    //    f1-h3
    pub static ref F1_H3_DIAG : Bitboard = bitboard!("f1", "g2", "h3");
    //    h1-h1
    pub static ref H1_H1_DIAG : Bitboard = bitboard!("h1");
    //
    //    a7-b8
    pub static ref A7_B8_DIAG : Bitboard = bitboard!("a7", "b8");
    //    a5-d8 "Caro-Kann Diagonal"
    pub static ref A5_D8_DIAG : Bitboard = bitboard!("a5", "b6", "c7", "d8");
    //    a3-f8
    pub static ref A3_F8_DIAG : Bitboard = bitboard!("a3", "b4", "c5", "d6", "e7", "f8");
    //    a1-h8 "Larsen Diagonal"
    pub static ref A1_H8_DIAG : Bitboard = bitboard!("a1", "b2", "c3", "d4", "e5", "f6", "g7", "h8");
    //    c1-h6
    pub static ref C1_H6_DIAG : Bitboard = bitboard!("c1", "d2", "e3", "f4", "g5", "h6");
    //    e1-h4 "Fool's Mate Diagonal"
    pub static ref E1_H4_DIAG : Bitboard = bitboard!("e1", "f2", "g3", "h4");
    //    g1-h2
    pub static ref G1_H2_DIAG : Bitboard = bitboard!("g1", "h2");
    //
    //    g8-h7
    pub static ref G8_H7_DIAG : Bitboard = bitboard!("g8", "h7");
    //    e8-h5 "Scholar's Mate Diagonal"
    pub static ref E8_H5_DIAG : Bitboard = bitboard!("e8", "f7", "g6", "h5");
    //    c8-h3
    pub static ref C8_H3_DIAG : Bitboard = bitboard!("c8", "d7", "e6", "f5", "g4", "h3");
    //    a8-h1 "Hungarian Diagonal"
    pub static ref A8_H1_DIAG : Bitboard = bitboard!("a8", "b7", "c6", "d5", "e4", "f3", "g2", "h1");
    //    a6-f1
    pub static ref A6_F1_DIAG : Bitboard = bitboard!("a6", "b5", "c4", "d3", "e2", "f1");
    //    a4-d1 "Rubinstein Diagonal"
    pub static ref A4_D1_DIAG : Bitboard = bitboard!("a4", "b3", "c2", "d1");
    //    a2-b1
    pub static ref A2_B1_DIAG : Bitboard = bitboard!("a2", "b1");

    //    a1-a1
    pub static ref A1_A1_DIAG : Bitboard = bitboard!("a1");
    //    a3-c1
    pub static ref A3_C1_DIAG : Bitboard = bitboard!("a3", "b2", "c1");
    //    a5-e1 "Bogo-Indian Diagonal"
    pub static ref A5_E1_DIAG : Bitboard = bitboard!("a5", "b4", "c3", "d2", "e1");
    //    a7-g1
    pub static ref A7_G1_DIAG : Bitboard = bitboard!("a7", "b6", "c5", "d4", "e3", "f2", "g1");
    //    b8-h2
    pub static ref B8_H2_DIAG : Bitboard = bitboard!("b8", "c7", "d6", "e5", "f4", "g3", "h2");
    //    d8-h4
    pub static ref D8_H4_DIAG : Bitboard = bitboard!("d8", "e7", "f6", "g5", "h4");
    //    f8-h6
    pub static ref F8_H6_DIAG : Bitboard = bitboard!("f8", "g7", "h6");
    //    h8-h8
    pub static ref H8_DIAG : Bitboard = bitboard!("h8");
}
