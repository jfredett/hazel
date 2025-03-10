use crate::{board::PieceBoard, constants::move_tables::{KING_ATTACKS, KNIGHT_MOVES}, coup::rep::Move, notation::{ben::BEN, Square}, types::{pextboard, Bitboard, Color, Direction, Occupant, Piece}, Alter, Alteration, Play, Query};

use super::position_metadata::PositionMetadata;


#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    // necessaries
    pub initial: BEN,
    pub moves: Vec<Move>,
    // caches

    // Alteration Cache should be by piece and color, so I can selectively reconstruct bitboards
    // from the alterations.
    // The cache itself should live on a movegenerator, to which we should inject at run-time,
    // since the movegen might be running in separate threads with thread-local caches, or even on
    // different machines.
    // Caches can be stored by zobrist, eventually I can have a metacache that can allow
    // cross-thread cache lookups
}

// adding a move should lazily update cached representations, we might get several moves at once.
// We also need to be able to un-apply moves from the alteration cache piecemeal.
//
// TODO: 22-FEB-2025 - Refactor the heck out of the 'slow' versions of these methods and figure out
// where they should really be living. Lots of duplication to reduce here.
//

impl Query for Position {
    fn get(&self, square: impl Into<Square>) -> Occupant {
        self.board().get(square)
    }

    fn try_metadata(&self) -> Option<PositionMetadata> {
        Some(self.metadata())
    }
}


impl Position {
    pub fn new(fen: impl Into<BEN>, moves: Vec<Move>) -> Self {
        let fen = fen.into();

        let mut metadata_cache = fen.metadata();
        let mut alteration_cache : Vec<Alteration> = fen.to_alterations().collect();

        Self { initial: fen.into(), moves }
    }


    pub fn board(&self) -> PieceBoard {
        self.current_boardstate().0
    }

    pub fn metadata(&self) -> PositionMetadata {
        self.current_boardstate().1
    }

    pub fn current_boardstate(&self) -> (PieceBoard, PositionMetadata) {
        let mut board = PieceBoard::default();
        let mut meta = self.initial.metadata();

        board.set_position(self.initial);

        for mov in &self.moves {
            let alterations = mov.compile(&board);
            meta.update(mov, &board);
            for alteration in alterations {
                board.alter_mut(alteration);
            }
        }

        (board, meta)
    }

    pub fn make(&mut self, mov: Move) {
        self.moves.push(mov);
        // everything is computed on-demand, no cache yet, so this is all that's needed.
    }

    pub fn unmake(&mut self) {
        self.moves.pop();
    }

    #[inline(always)]
    pub fn hero(&self) -> Color {
        self.metadata().side_to_move
    }

    #[inline(always)]
    pub fn villain(&self) -> Color {
        !self.hero()
    }

    // FIXME: the searches are super inefficient and very ugly. A better world is possible, you
    // just have to build it yourself.

    pub fn find(&self, pred: impl Fn(&(Square, Occupant)) -> bool) -> Bitboard {
        let mut bb = Bitboard::empty();
        for (sq, _) in self.board().by_occupant().filter(pred) {
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
        KING_ATTACKS[self.our_king().index()] & !self.friendlies()
    }

    pub fn our_king_attacks(&self) -> Bitboard {
        self.our_king_moves() & self.enemies()
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
        KING_ATTACKS[self.their_king().index()] & !self.enemies()
    }

    pub fn their_king_attacks(&self) -> Bitboard {
        self.their_king_moves() & self.friendlies()
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
    use crate::coup::rep::MoveType;

    mod gamestate {
        use super::*;

        #[test]
        fn kiwi() {
            let kiwi = BEN::new(POS2_KIWIPETE_FEN);
            let mut pb = PieceBoard::default();
            pb.set_position(kiwi);

            let position = Position::new(kiwi, vec![]);

            assert_eq!(position.board(), pb);
            assert_eq!(position.metadata(), kiwi.metadata());
        }

        // #[test] // I entered the moves wrong, I don't know where.
        fn d4() {
            let start = BEN::start_position();
            let target = BEN::new("rnbqk2r/pp2bppp/2p1pn2/3p4/3P1B2/3BPN2/PPP2PPP/RN1Q1RK1 b kq - 1 6");
            let moves = vec![
                Move::new(D2, D4, MoveType::DOUBLE_PAWN), Move::new(D7, D5, MoveType::DOUBLE_PAWN),
                Move::new(C1, F4, MoveType::QUIET), Move::new(E7, E6, MoveType::QUIET),
                Move::new(E2, E3, MoveType::QUIET), Move::new(G8, F6, MoveType::QUIET),
                Move::new(G1, F3, MoveType::QUIET), Move::new(F8, E7, MoveType::QUIET),
                Move::new(F1, D3, MoveType::QUIET), Move::new(C7, C6, MoveType::QUIET),
                Move::short_castle(Color::WHITE)
            ];

            let mut pb = PieceBoard::default();
            pb.set_position(target);

            let position = Position::new(start, moves);

            assert_eq!(position.board(), pb);
            assert_eq!(position.metadata(), start.metadata());
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

        mod attacks {
            use super::*;

            #[test]
            fn white() {
                let pos = Position::new(BEN::new("8/8/1k6/P1P1p3/3K4/8/8/8 w - - 0 1"), vec![]);
                assert_eq!(pos.our_king_attacks(), Bitboard::from(E5));
                assert_eq!(pos.their_king_attacks(),  Bitboard::from(A5) | Bitboard::from(C5));
            }

            #[test]
            fn black() {
                let pos = Position::new(BEN::new("8/8/1k6/P1P1p3/3K4/8/8/8 b - - 0 1"), vec![]);
                assert_eq!(pos.our_king_attacks(),  Bitboard::from(A5) | Bitboard::from(C5));
                assert_eq!(pos.their_king_attacks(), Bitboard::from(E5));
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
