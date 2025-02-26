use std::{fmt::Debug, sync::RwLock};


use crate::{board::PieceBoard, constants::move_tables::{KING_ATTACKS, KNIGHT_MOVES}, coup::rep::Move, notation::{ben::BEN, Square}, types::{pextboard, Bitboard, Color, Direction, Occupant, Piece}, Alter, Alteration, Query};
use crate::types::zobrist::Zobrist;

use super::position_metadata::PositionMetadata;
use crate::coup::gen::cache::Cache;


pub struct Position {
    // necessaries
    pub initial: BEN,
    pub moves: Vec<Move>,
    // caches
    zobrist: RwLock<Zobrist>,

    // this should actually be an ATM, and the cache lives on Movegen?
    state_cache: Cache<(PieceBoard, PositionMetadata, Vec<Alteration>)>
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position::new(self.initial, self.moves.clone())
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        // Eventually this might just compare zobrists to start?
        self.initial == other.initial && self.moves == other.moves
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.initial);
        writeln!(f, "{:?}", self.moves);
        writeln!(f, "{:?}", self.zobrist())
    }
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
        // BUG: constructing this is a nightmare.
        //
        // I have to build the position and it's RWLock, but add the moves one-by-one, and build
        // the cache. I expect there will only be a few total positions, and the cache should
        // eventually have a proper cache object, the only locally cached thing should be the
        // Zobrist, I suppose I could wrap it in an optional, or maybe push the mutability _into_
        // zobrist?
        //
        //
        let mut ret = Self {
            initial: fen,
            moves: vec![],
            state_cache: Cache::new(Self::calculate_boardstate),
            // this is a dummy
            zobrist: RwLock::new(Zobrist::empty())
        };
        for mov in moves {
            ret.make(mov);
        }

        let zobrist = Zobrist::new(&ret);

        ret.zobrist = RwLock::new(zobrist);

        ret
    }

    pub fn zobrist(&self) -> Zobrist {
        *self.zobrist.read().unwrap()
    }

    pub fn board(&self) -> PieceBoard {
        self.current_boardstate().0
    }

    pub fn metadata(&self) -> PositionMetadata {
        self.current_boardstate().1
    }

    pub fn cached_alterations(&self) -> Vec<Alteration> {
        self.current_boardstate().2
    }

    pub fn current_boardstate(&self) -> (PieceBoard, PositionMetadata, Vec<Alteration>) {
        // FIXME: Current bug is definitely somewhere in the caching logic. Turning it off yields
        // a 900x slowdown w/ the alteration caching.
        //
        // I suspect we may be hitting a cache collision somewhere, or maybe I should embed the
        // metadata into the cache, also need to check side-to-move calculation
        self.state_cache.get(self)
    }

    pub fn calculate_boardstate(position: &Position) -> (PieceBoard, PositionMetadata, Vec<Alteration>) {
        let mut board = PieceBoard::default();
        let mut meta = position.initial.metadata();
        let mut out_alterations : Vec<Alteration> = position.initial.to_alterations().collect();

        board.set_position(position.initial);

        for mov in &position.moves {
            let alterations = mov.compile(&board);

            out_alterations.push(Alteration::StartTurn);

            meta.update(mov, &board);

            out_alterations.push(Alteration::Assert(meta));

            for alteration in &alterations {
                out_alterations.push(*alteration);
                board.alter_mut(*alteration);
            }

        }

        (board, meta, out_alterations)
    }


    pub fn make(&mut self, mov: Move) {
        let new_alterations = mov.compile(&self.board());
        let mut z = self.zobrist.write().unwrap();

        z.update(&new_alterations);

        self.moves.push(mov);
        // everything is computed on-demand, no cache yet, so this is all that's needed.
    }

    pub fn unmake(&mut self) {
        // BUG: This is probably where my issue is w/ perft3 and zobrist, I'm not correctly
        // reverting the change here.
        // In the calculate boardstate I add end-turn markers, I think the procedure is grab the
        // current cache of alterations, grab the slice from the current EOT marker to the
        // previous, invert them and apply them to the zobrist, then we can check cache for the 
        // position, and build it if it isn't there.
        
        // We want to check cache first, and if we miss, we don't mind building, so we want to find
        // the zobrist corresponding to the state where the last move hasn't applied.
        //
        // When we cache the alterations, we include turn markers. We can just pop off the
        // alteration stack until we hit one.
        let mut unmove = vec![];

        { // we're going to scope this to prevent any kind of leakage, we're going to hit cache
          // twice w/ different keys
            let mut alterations = self.cached_alterations();
            loop {
                match alterations.pop() {
                    Some(Alteration::StartTurn) => { break; }
                    Some(alter) => { unmove.push(alter); }
                    None => { panic!("trying to unmake with no moves"); }
                }
            }
        }

        // We now have all the necessaries to de-zobrist ourselves. Since XOR is it's own inverse,
        // We can just run these again. Order doesn't matter either, since XOR commutes.
        let mut z = self.zobrist.write().unwrap();
        z.update(&unmove);
        
        // Now our zobrist points to the prior position. We can finally pop the move from the stack
        self.moves.pop();
        // and check cache will happen the next time we need it automatically
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
        self.find(|(_sq, occ)| {
            occ.color() == Some(*color)
        })
    }

    pub fn pawns_for(&self, color: &Color) -> Bitboard {
        self.find(|(_sq, occ)| {
            *occ == Occupant::Occupied(Piece::Pawn, *color)
        })
    }

    pub fn knights_for(&self, color: &Color) -> Bitboard {
        self.find(|(_sq, occ)| {
            *occ == Occupant::Occupied(Piece::Knight, *color)
        })
    }

    pub fn rooks_for(&self, color: &Color) -> Bitboard {
        self.find(|(_sq, occ)| {
            *occ == Occupant::Occupied(Piece::Rook, *color)
        })
    }

    pub fn bishops_for(&self, color: &Color) -> Bitboard {
        self.find(|(_sq, occ)| {
            *occ == Occupant::Occupied(Piece::Bishop, *color)
        })
    }

    pub fn queens_for(&self, color: &Color) -> Bitboard {
        self.find(|(_sq, occ)| {
            *occ == Occupant::Occupied(Piece::Queen, *color)
        })
    }

    pub fn pawn_attacks_for(&self, color: &Color) -> Bitboard {
        let advance = self.pawns_for(color).shift(color.pawn_direction());
        advance.shift(Direction::E) | advance.shift(Direction::W)
    }

    pub fn all_blockers(&self) -> Bitboard {
        self.find(|(_sq, occ)| { occ.is_occupied() })
    }

    pub fn friendlies(&self) -> Bitboard {
        self.all_pieces_of(&self.hero())
    }

    pub fn enemies(&self) -> Bitboard {
        self.all_pieces_of(&self.villain())
    }

    pub fn our_checks(&self) -> Bitboard {
        let blockers = self.all_blockers();
        let potential_attackers = self.enemies();
        // To calculate all the squares from which a piece of a given type might give a check to
        // our king. Consider:
        //
        // 8 . . . . . . . .
        // 7 . . . . . . . .
        // 6 . . . . R . . .
        // 5 . . . . . . . .
        // 4 . . k . . . . .
        // 3 . . . . . . . .
        // 2 . . . . . . . .
        // 1 . . . . . . . .
        //   a b c d e f g h
        //
        // In this board, the critical check squares are:
        //
        // 1. All of the C file
        // 2. All of the 4 rank
        // 3. the A2-G8 diag
        // 4. the A6-F1 diag
        // 5. A3, A6, B2, B7, D2, D7, E3, E6 - the knight-moves around the king.
        //
        // In order for the king to be checked, a piece of the correct type must be present on the
        // correct square, this calculates all the valid check squares,
        self.our_assassin_squares(Piece::Bishop) |
        self.our_assassin_squares(Piece::Rook) |
        self.our_assassin_squares(Piece::Queen) |
        self.our_assassin_squares(Piece::Knight) |
        self.our_assassin_squares(Piece::Pawn)
    }


    /// Squares from which our king may be checked
    pub fn our_assassin_squares(&self, piece: Piece) -> Bitboard {
        let blockers = self.all_blockers();
        let mask = match piece {
            Piece::Knight => self.their_knight_moves(),
            Piece::Bishop => self.their_bishop_moves() | self.their_queen_moves(),
            Piece::Rook => self.their_rook_moves() | self.their_queen_moves(),
            Piece::Queen => self.their_queen_moves(),
            Piece::Pawn => { 
                let their_pawn_quiet_moves = self.their_pawn_advances() | self.their_pawn_double_advances();
                let their_pawn_captures = self.their_pawn_attacks() & self.friendlies();

                let their_pawn_moves = their_pawn_quiet_moves | their_pawn_captures;

                let their_threatened_squares = their_pawn_moves.shift(self.their_pawn_direction());

                // promotion checks
                // capture promotion checks
                
                their_threatened_squares.shift(Direction::E) | their_threatened_squares.shift(Direction::W)
            }
            Piece::King => { Bitboard::empty() /* Kings can't check kings. */ }
        };
        pextboard::attacks_for(piece, self.our_king(), blockers) & mask
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
        let res = self.find(|(_sq, occ)| {
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

    pub fn our_pawn_advances(&self) -> Bitboard {
        let blockers = self.all_blockers();
        self.pawns_for(&self.hero()).shift(self.our_pawn_direction()) & !blockers
    }

    pub fn our_first_rank_pawns(&self) -> Bitboard {
        self.pawns_for(&self.hero()) & self.our_pawn_rank()
    }

    pub fn our_pawn_rank(&self) -> Bitboard {
        self.hero().pawn_mask()
    }

    pub fn our_pawn_double_advances(&self) -> Bitboard {
        let blockers = self.all_blockers();
        let first_advance = self.our_first_rank_pawns().shift(self.our_pawn_direction()) & !blockers;
        first_advance.shift(self.our_pawn_direction()) &!blockers
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
        let res = self.find(|(_sq, occ)| {
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

    pub fn their_pawn_advances(&self) -> Bitboard {
        let blockers = self.all_blockers();
        self.pawns_for(&self.villain()).shift(self.their_pawn_direction()) & !blockers
    }

    pub fn their_first_rank_pawns(&self) -> Bitboard {
        self.pawns_for(&self.villain()) & self.their_pawn_rank()
    }

    pub fn their_pawn_rank(&self) -> Bitboard {
        self.villain().pawn_mask()
    }

    pub fn their_pawn_double_advances(&self) -> Bitboard {
        let blockers = self.all_blockers();
        let first_advance = self.their_first_rank_pawns().shift(self.their_pawn_direction()) & !blockers;
        first_advance.shift(self.their_pawn_direction()) &!blockers
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
