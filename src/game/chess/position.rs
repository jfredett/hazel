use crate::{board::PieceBoard, constants::move_tables::{KING_ATTACKS, KNIGHT_MOVES}, coup::rep::Move, notation::{ben::BEN, Square}, types::{pextboard, Bitboard, Color, Direction, Occupant, Piece}, Alter, Alteration, Query};

use super::position_metadata::PositionMetadata;


#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    // necessaries
    pub initial: BEN,
    pub moves: Vec<Move>,
    metadata: PositionMetadata,
    // caches

    // Alteration Cache should be by piece and color, so I can selectively reconstruct bitboards
    // from the alterations.
    pub(crate) board: PieceBoard,
    pub(crate) alteration_cache: Vec<Alteration>
}

// adding a move should lazily update cached representations, we might get several moves at once.
// We also need to be able to un-apply moves from the alteration cache piecemeal.
//
// TODO: 22-FEB-2025 - Refactor the heck out of the 'slow' versions of these methods and figure out
// where they should really be living. Lots of duplication to reduce here.
//

impl Query for Position {
    fn get(&self, square: impl Into<Square>) -> Occupant {
        self.board.get(square)
    }

    fn metadata(&self) -> Option<PositionMetadata> {
        Some(self.metadata)
    }
}


// TODO: this'll implement play at some point

impl Position {
    pub fn new(fen: impl Into<BEN>, moves: Vec<Move>) -> Self {
        let fen = fen.into();
        let mut board = PieceBoard::default();
        board.set_position(fen);

        let mut metadata = fen.metadata();
        let mut alteration_cache : Vec<Alteration> = fen.to_alterations().collect();

        for mov in &moves {
            let alterations = mov.compile(&board);
            for alteration in alterations {
                alteration_cache.push(alteration);
                board.alter_mut(alteration);
            }
            metadata.update(mov, &board);
        }
        Self { initial: fen.into(), moves, metadata, board, alteration_cache }
    }

    #[inline(always)]
    pub fn hero(&self) -> Color {
        // FIXME: Unwrap is safe because it's a quirk of query, this always has metadata. Probably query
        // shouldn't work this way.
        self.metadata().unwrap().side_to_move
    }

    #[inline(always)]
    pub fn villain(&self) -> Color {
        !self.hero()
    }

    // FIXME: the searches are super inefficient and very ugly. A better world is possible, you
    // just have to build it yourself.

    pub fn find(&self, pred: impl Fn(&(Square, Occupant)) -> bool) -> Bitboard {
        let mut bb = Bitboard::empty();
        for (sq, _) in self.board.by_occupant().filter(pred) {
            bb.set(sq);
        }
        bb
    }


    pub fn all_pieces_of(&self, color: &Color) -> Bitboard {
        self.find(|(sq, occ)| {
            occ.color() == Some(*color)
        })
    }

    pub fn intervention_squares(&self) -> Bitboard {
        todo!()
    }

    pub fn all_attacked_squares(&self, color: &Color) -> Bitboard {
        todo!()
    }

    pub fn pawns_for(&self, color: &Color) -> Bitboard {
        self.find(|(sq, occ)| {
            *occ == Occupant::Occupied(Piece::Pawn, *color)
        })
    }

    pub fn knights_for(&self, color: &Color) -> Bitboard {
        self.find(|(sq, occ)| {
            *occ == Occupant::Occupied(Piece::Knight, *color)
        })
    }

    pub fn rooks_for(&self, color: &Color) -> Bitboard {
        self.find(|(sq, occ)| {
            *occ == Occupant::Occupied(Piece::Rook, *color)
        })
    }

    pub fn bishops_for(&self, color: &Color) -> Bitboard {
        self.find(|(sq, occ)| {
            *occ == Occupant::Occupied(Piece::Bishop, *color)
        })
    }

    pub fn queens_for(&self, color: &Color) -> Bitboard {
        self.find(|(sq, occ)| {
            *occ == Occupant::Occupied(Piece::Queen, *color)
        })
    }

    pub fn pawn_attacks_for(&self, color: &Color) -> Bitboard {
        let advance = self.pawns_for(color).shift(color.pawn_direction());
        advance.shift(Direction::E) | advance.shift(Direction::W)
    }

    pub fn all_blockers(&self) -> Bitboard {
        self.find(|(sq, occ)| { occ.is_occupied() })
    }

    pub fn friendlies(&self) -> Bitboard {
        self.all_pieces_of(&self.hero())
    }

    pub fn enemies(&self) -> Bitboard {
        self.all_pieces_of(&self.villain())
    }


    // ### OUR HERO'S MOVES, ATTACKS, AND THE LIKE ### //

    // OQ: How should I sort these? Position so far has been 'find the pieces on the board', but it
    // also needs to know about attacks to some extent to calculate other usefuls. So far these
    // functions have been specifically encoding movement rules, maybe that's the line?

    pub fn our_king_moves(&self) -> Bitboard {
        KING_ATTACKS[self.our_king().index()] & self.enemies()
    }

    pub fn our_king(&self) -> Square {
        let res = self.find(|(sq, occ)| {
            *occ == Occupant::Occupied(Piece::King, self.hero())
        }).all_set_squares();
        assert_eq!(res.len(), 1);
        res[0]
    }


    /// a bitboard showing the location of all our pawns
    pub fn our_pawns(&self) -> Bitboard {
        self.pawns_for(&self.hero())
    }

    /// the direction our pawns travel
    pub fn our_pawn_direction(&self) -> Direction {
        self.hero().pawn_direction()
    }

    /// all squares attacked by at least one of our pawns
    pub fn our_pawn_attacks(&self) -> Bitboard {
        self.pawn_attacks_for(&self.hero())
    }

    /// Bitboard showing the location of all our knights
    pub fn our_knights(&self) -> Bitboard {
        self.knights_for(&self.hero())
    }

    pub fn our_knight_moves(&self) -> Bitboard {
        let friendlies = self.friendlies();

        self.find(|(_, occ)| { *occ == Occupant::Occupied(Piece::Knight, self.hero()) })
            .into_iter()
            .map(|sq| { KNIGHT_MOVES[sq.index()] & !friendlies })
            .fold(Bitboard::empty(), |acc, e| acc | e)
    }

    pub fn our_bishop_moves(&self) -> Bitboard {
        self.slide_attacks_for(Piece::Bishop, self.hero())
    }

    pub fn our_rook_moves(&self) -> Bitboard {
        self.slide_attacks_for(Piece::Rook, self.hero())
    }

    pub fn our_queen_moves(&self) -> Bitboard {
        self.slide_attacks_for(Piece::Queen, self.hero())
    }

    /// The set of all squares we attack, does not include our pieces positions, nor does it
    /// account for contested squares, nor does it count the number of attackers attacking a
    /// particular square.
    pub fn our_reach(&self) -> Bitboard {
        self.our_king_moves() |
        self.our_pawn_attacks() |
        self.our_knight_moves() |
        self.our_bishop_moves() |
        self.our_rook_moves() |
        self.our_queen_moves()
    }

    // ### THE VILLAIN'S HENCHMEN, MOVES, AND SO ON ### //

    pub fn their_king_moves(&self) -> Bitboard {
        KING_ATTACKS[self.their_king().index()] & self.friendlies()
    }

    pub fn their_king(&self) -> Square {
        let res = self.find(|(sq, occ)| {
            *occ == Occupant::Occupied(Piece::King, self.villain())
        }).all_set_squares();
        assert_eq!(res.len(), 1);
        res[0]
    }


    /// a bitboard showing the location of all their pawns
    pub fn their_pawns(&self) -> Bitboard {
        self.pawns_for(&self.villain())
    }

    /// the direction their pawns travel
    pub fn their_pawn_direction(&self) -> Direction {
        self.villain().pawn_direction()
    }

    /// all squares attacked by at least one of their pawns
    pub fn their_pawn_attacks(&self) -> Bitboard {
        let advance = self.their_pawns().shift(self.their_pawn_direction());
        advance.shift(Direction::E) | advance.shift(Direction::W)

    }

    /// all squares attacked by at least one of their knights
    pub fn their_knight_moves(&self) -> Bitboard {
        let enemies = self.enemies();
        self.find(|(_, occ)| { *occ == Occupant::Occupied(Piece::Knight, self.villain()) })
            .into_iter()
            .map(|sq| { KNIGHT_MOVES[sq.index()]  & !enemies })
            .fold(Bitboard::empty(), |acc, e| acc | e)
    }

    pub fn their_bishop_moves(&self) -> Bitboard {
        self.slide_attacks_for(Piece::Bishop, self.villain())
    }

    pub fn their_rook_moves(&self) -> Bitboard {
        self.slide_attacks_for(Piece::Rook, self.villain())
    }

    pub fn their_queen_moves(&self) -> Bitboard {
        self.slide_attacks_for(Piece::Queen, self.villain())
    }

    fn slide_attacks_for(&self, piece: Piece , color: Color) -> Bitboard {
        let blockers = self.all_blockers();
        self.find(|(_, occ)| { *occ == Occupant::Occupied(piece, color) })
            .into_iter()
            .map(|sq| { pextboard::attacks_for(piece, sq, blockers) })
            .fold(Bitboard::empty(), |acc, e| acc | e)
    }

    pub fn their_reach(&self) -> Bitboard {
        self.their_king_moves() |
        self.their_pawn_attacks() |
        self.their_knight_moves() |
        self.their_bishop_moves() |
        self.their_rook_moves() |
        self.their_queen_moves()
    }
}


#[cfg(test)]
mod tests {
    // NOTE: These tests are largely temporary, since I intend to refactor these to use test
    // positions and calculate all the moves in aggregate, these tests only cover the simple cases
    // and specific, narrower testcases will replace the startpos stuff eventually.
    // This ties to a move to, e.g., rstest or some other framework.
    use super::*;
    use crate::notation::*;

    mod new {
        use crate::game::castle_rights::CastleRights;

        use super::*;


        #[test]
        fn correctly_caches_start_position() {
            let pos = Position::new(BEN::start_position(), vec![]);

            assert_eq!(pos.alteration_cache, vec![
                Alteration::Clear,
                Alteration::Assert(PositionMetadata {
                    side_to_move: Color::WHITE,
                    castling: CastleRights {
                        white_short: true,
                        white_long: true,
                        black_short: true,
                        black_long: true
                    }, 
                    en_passant: None,
                    halfmove_clock: 0,
                    fullmove_number: 1
                }),
                Alteration::place(A1, Occupant::white_rook()),
                Alteration::place(B1, Occupant::white_knight()),
                Alteration::place(C1, Occupant::white_bishop()),
                Alteration::place(D1, Occupant::white_queen()),
                Alteration::place(E1, Occupant::white_king()),
                Alteration::place(F1, Occupant::white_bishop()),
                Alteration::place(G1, Occupant::white_knight()),
                Alteration::place(H1, Occupant::white_rook()),
                Alteration::place(A2, Occupant::white_pawn()),
                Alteration::place(B2, Occupant::white_pawn()),
                Alteration::place(C2, Occupant::white_pawn()),
                Alteration::place(D2, Occupant::white_pawn()),
                Alteration::place(E2, Occupant::white_pawn()),
                Alteration::place(F2, Occupant::white_pawn()),
                Alteration::place(G2, Occupant::white_pawn()),
                Alteration::place(H2, Occupant::white_pawn()),
                Alteration::place(A7, Occupant::black_pawn()),
                Alteration::place(B7, Occupant::black_pawn()),
                Alteration::place(C7, Occupant::black_pawn()),
                Alteration::place(D7, Occupant::black_pawn()),
                Alteration::place(E7, Occupant::black_pawn()),
                Alteration::place(F7, Occupant::black_pawn()),
                Alteration::place(G7, Occupant::black_pawn()),
                Alteration::place(H7, Occupant::black_pawn()),
                Alteration::place(A8, Occupant::black_rook()),
                Alteration::place(B8, Occupant::black_knight()),
                Alteration::place(C8, Occupant::black_bishop()),
                Alteration::place(D8, Occupant::black_queen()),
                Alteration::place(E8, Occupant::black_king()),
                Alteration::place(F8, Occupant::black_bishop()),
                Alteration::place(G8, Occupant::black_knight()),
                Alteration::place(H8, Occupant::black_rook()),
            ]);

        }
    }

    mod pawns {
        use super::*;

        mod our_pawns {
            use super::*;

            #[test]
            fn startpos() {
                let pos = Position::new(BEN::start_position(), vec![]);
                assert_eq!(pos.our_pawns(), Color::WHITE.pawn_mask());
            }
        }

        mod our_pawn_direction {
            use super::*;

            #[test]
            fn startpos_white() {
                let pos = Position::new(BEN::start_position(), vec![]);
                assert_eq!(pos.our_pawn_direction(), Direction::N);
            }

            #[test]
            fn startpos_black() {
                let pos = Position::new(BEN::start_position(), vec![]);
                assert_eq!(pos.their_pawn_direction(), Direction::S);
            }
        }

        mod our_pawn_attacks {
            use crate::constants::{RANK_3, RANK_6};

            use super::*;

            #[test]
            fn startpos_white() {
                let pos = Position::new(BEN::start_position(), vec![]);
                let expected = *RANK_3;
                assert_eq!(pos.our_pawn_attacks(), expected);
            }

            #[test]
            fn startpos_black() {
                let mut pos = Position::new(BEN::start_position(), vec![]);
                let expected = *RANK_6;
                assert_eq!(pos.their_pawn_attacks(), expected);
            }
        }

        mod pawns_for {
            use super::*;

            #[test]
            fn startpos() {
                let pos = Position::new(BEN::start_position(), vec![]);
                assert_eq!(
                    pos.pawns_for(&Color::WHITE),
                    Color::WHITE.pawn_mask()
                );

                assert_eq!(
                    pos.pawns_for(&Color::BLACK),
                    Color::BLACK.pawn_mask()
                );
            }
        }
    }

    mod knights {
        use super::*;
        mod moves {
            use super::*;

            #[test]
            fn startpos_white() {
                let pos = Position::new(BEN::start_position(), vec![]);
                let expected = Bitboard::from(C3) | Bitboard::from(A3) | Bitboard::from(F3) | Bitboard::from(H3);
                assert_eq!(pos.our_knight_moves(), expected);
            }

            #[test]
            fn startpos_black() {
                let pos = Position::new(BEN::start_position(), vec![]);
                let expected = Bitboard::from(C6) | Bitboard::from(A6) | Bitboard::from(F6) | Bitboard::from(H6);
                assert_eq!(pos.their_knight_moves(), expected);
            }
        }
    }

    mod king {
        use super::*;

        mod moves {
            use super::*;

            #[test]
            fn white() {
                let pos = Position::new(BEN::new("8/8/1k6/P1P1p3/3K4/8/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_king_moves(), Bitboard::from(E5));
                assert_eq!(pos.their_king_moves(),  Bitboard::from(A5) | Bitboard::from(C5));
            }

            #[test]
            fn black() {
                let pos = Position::new(BEN::new("8/8/1k6/P1P1p3/3K4/8/8/8 b - - 0 1"), vec![]);
                assert_eq!(pos.our_king_moves(),  Bitboard::from(A5) | Bitboard::from(C5));
                assert_eq!(pos.their_king_moves(), Bitboard::from(E5));
            }
        }
    }

    mod bishop {
        use super::*;

        mod moves {
            use crate::bitboard;

            use super::*;

            #[test]
            fn light_square() {
                let pos = Position::new(BEN::new("8/1b6/8/8/8/1B6/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_bishop_moves(), bitboard!(A4, A2, C4, C2, D1, D5, E6, F7, G8));
                assert_eq!(pos.their_bishop_moves(),  bitboard!(A8, A6, C8, C6, D5, E4, F3, G2, H1));
            }

            #[test]
            fn dark_square() {
                let pos = Position::new(BEN::new("8/2b5/8/8/8/2B5/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_bishop_moves(),  bitboard!(B2, D2, A1, E1, B4, A5, D4, E5, F6, G7, H8));
                assert_eq!(pos.their_bishop_moves(), bitboard!(B8, D8, B6, A5, D6, E5, F4, G3, H2));
            }

            #[test]
            fn with_blocker() {
                let pos = Position::new(BEN::new("8/2b5/8/4B3/8/8/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_bishop_moves(),  bitboard!(C7, D6, F6, G7, H8, D4, C3, B2, A1, F4, G3, H2));
                assert_eq!(pos.their_bishop_moves(), bitboard!(B8, B6, D8, D6, E5, A5));
            }
        }
    }

    use crate::{bitboard, constants::*};
    mod rook {
        use super::*;

        mod moves {

            use super::*;

            #[test]
            fn open_files() {
                let pos = Position::new(BEN::new("8/2r5/8/4R3/8/8/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_rook_moves(), *RANK_5 ^ *E_FILE);
                assert_eq!(pos.their_rook_moves(),  *RANK_7 ^ *C_FILE);
            }


            #[test]
            fn with_blocker() {
                let pos = Position::new(BEN::new("8/1P1r1p2/8/8/1p1R1P2/8/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_rook_moves(),  (*RANK_4 ^ *D_FILE) & !bitboard!(A4, D8, G4, H4));
                assert_eq!(pos.their_rook_moves(), bitboard!(D8, B7, C7, E7, F7, D6, D5, D4));
            }
        }

    }

    mod queen {
        use super::*;
        mod moves {

            use super::*;

            #[test]
            fn open_files() {
                let pos = Position::new(BEN::new("8/2q5/8/4Q3/8/8/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_queen_moves(), (*RANK_5 ^ *E_FILE) | (*A1_H8_DIAG ^ *B8_H2_DIAG) & !bitboard!(B8));
                assert_eq!(pos.their_queen_moves(),  (*RANK_7 ^ *C_FILE) | bitboard!(B8, D8, B6, D6, A5, E5));
            }


            #[test]
            fn with_blocker() {
                let pos = Position::new(BEN::new("8/1P1q1p2/8/8/1p1Q1P2/8/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_queen_moves(), ((*RANK_4 ^ *D_FILE) | (*A1_H8_DIAG ^ *A7_G1_DIAG)) & !bitboard!(D8,A4, G4, H4));
                assert_eq!(pos.their_queen_moves(), bitboard!(A4, B5, B7, C6, C7, C8, D4, D5, D6, D8, E6, E7, E8, F5, F7, G4, H3));
            }
        }

    }

}
