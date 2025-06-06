use std::sync::Arc;
use std::{fmt::Debug, sync::RwLock};

use hazel_core::ben::BEN;
use hazel_core::color::Color;
use hazel_core::direction::Direction;
use hazel_core::interface::query::to_fen_position;
use hazel_core::occupant::Occupant;
use hazel_core::piece::Piece;
use hazel_core::position_metadata::PositionMetadata;
use hazel_core::square::Square;
use hazel_core::interface::{alter, query, Alter, Alteration, Query};
use hazel_core::zobrist::Zobrist;
use hazel_bitboard::bitboard::Bitboard;
use hazel_bitboard::ColorMasks;
use hazel_bitboard::constants::move_tables::{KING_ATTACKS, KNIGHT_MOVES};
use spell::cursorlike::Cursorlike as _;
use spell::familiar::{Familiar, Quintessence};
use spell::tapelike::Tapelike;

use crate::board::PieceBoard;
use crate::coup::rep::Move;
use spell::Tape;
use crate::game::state::position_zobrist::PositionZobrist;


// generator is supposed to depend on core, but I need to reference generator here to build
// position efficiently.
//
// I think position -> engine?
//
// Except enging/gen/eval should all boil down to providing a single actor
//
// Except all I need here are cache/atm, which should be in util anyway, so time to pull out util
use hazel_util::cache::Cache;
use hazel_util::cache::atm::ATM;

pub struct Position {
    // necessaries
    pub initial: BEN,
    // caches
    // FIXME: pub only for testing.
    pub tape: Arc<RwLock<Tape>>,
    inner: RwLock<InnerPosition>,

    // this should live on movegen?
    atm: ATM<'static, InnerPosition>
}

// this should work like:
//
// 1. A familiar exists for inner position, we have it set to move by turn to the current write
//    head
// 2. familiar also calculates zobrists, lazily evaluates board/metadata/other stuff
// 3. need some way to do 'a collection of alterables is also alterable' so I can add stuff freely
//    to the familiar
// 4. when updating, calculate zobrist first, check cache, and only update if needed.


#[derive(Clone, Default, Debug, PartialEq)]
pub struct InnerPosition {
    // this should be a familiar, implementing `alter`, and then we store it's quintessence instead
    // of the whole struct.
    pub board: PieceBoard,
    pub metadata: PositionMetadata,
}

impl InnerPosition {
    pub fn new(board: PieceBoard, metadata: PositionMetadata) -> Self {
        InnerPosition {
            board,
            metadata,
        }
    }
}

impl Query for InnerPosition {
    fn get(&self, square: impl Into<Square>) -> Occupant {
        self.board.get(square)
    }

    fn try_metadata(&self) -> Option<PositionMetadata> {
        Some(self.metadata)
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        // FIXME: Ideally we'd actually just keep a reference to this cached thing instead of copying it
        // all over creation
        //
        // OQ: with the `Arc` here on tape, and the explicit clone here, I'm asserting that
        // '#clone' means "Create a _new copy_", which I think are the correct semantics but I'm
        // not sure.
        let new_inner = self.inner.read().unwrap().clone();
        let new_tape = self.tape.read().unwrap().clone();

        Position {
            initial: self.initial,
            tape: Arc::new(RwLock::new(new_tape)),
            inner: RwLock::new(new_inner),
            atm: self.atm
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        // NOTE: This is, very technically speaking, wrong.
        //
        // A zobrist is a 64 bit signature of a ~153 (19bytes + 1 bit) bit space (the number of
        // reachable chess positions is around 10e46, which is about 2e153), which means that this
        // has a nonzero (and indeed nontrivial) probability of collision. However, in any given
        // exploration we are not likely to run into a situation where this collision hurts, and I
        // have longer term plans to simply use a larger zobrist hash or BCH codes or something
        // similar to avoid collisions.
        //
        // For reference, a BEN is 36 bytes, or about 50% efficient. This is 8 bytes (200%
        // efficient) but with a nonzero, small error rate. BCH codes add a few bytes to lower that
        // error rate, but this should be sufficient for the short term...
        //
        // He said, awaiting the inevitable point where this becomes a multi-day bughunt.
        self.zobrist().position == other.zobrist().position
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.initial)?;
        writeln!(f, "{:?}", self.zobrist())?;
        writeln!(f, "{:?}", self.tape)
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

lazy_static!(
    pub static ref POSITION_CACHE : Cache<InnerPosition> = Cache::new();
);


impl From<Position> for BEN {
    // TODO: This could probably be better managed by a familiar.
    fn from(value: Position) -> Self {
        let mut ben : BEN = alter::setup(query::to_alterations(&value.board()));
        ben.set_metadata(value.metadata());
        ben
    }
}


impl Position {
    pub fn new(fen: impl Into<BEN>) -> Self {
        let fen = fen.into();
        let alters : Vec<Alteration> = fen.to_alterations().collect();

        let mut inner = InnerPosition::default();
        let mut tape = Tape::default();
        tape.write_all(&alters);

        for alter in alters {
            inner.board.alter_mut(alter);
            inner.metadata.alter_mut(alter);
        }

        Self {
            initial: fen,
            inner: inner.into(),
            tape: Arc::new(tape.into()),
            atm: POSITION_CACHE.atm()
        }
    }

    pub fn conjure<S>(&self) -> Familiar<RwLock<Tape>, S> where S : Default {
        spell::familiar::conjure(self.tape.clone())
    }

    pub fn resummon<S>(&self, quintessence: &Quintessence<S>) -> Familiar<RwLock<Tape>, S> where S : Clone {
        spell::familiar::resummon_on(self.tape.clone(), quintessence)
    }

    pub fn with_moves(fen: impl Into<BEN>, moves: Vec<Move>) -> Self {
        let mut ret = Self::new(fen);
        for m in moves {
            ret.make(m);
        }
        ret
    }

    pub fn zobrist(&self) -> PositionZobrist {
        // TODO: this is not ideal, it should cache this somewhere, probably as a quintessence.
        let mut fam : Familiar<RwLock<Tape>, PositionZobrist> = self.conjure();
        fam.seek(self.tape.read().unwrap().writehead());
        *fam.get()
    }

    pub fn metadata(&self) -> PositionMetadata {
        self.inner.read().unwrap().metadata
    }

    pub fn board(&self) -> PieceBoard {
        self.inner.read().unwrap().board
    }

    pub fn make(&mut self, mov: Move) {
        tracing::debug!("making: {:?}", mov);

        let new_alterations: Vec<Alteration>;

        {   // Incremental Update Calculation
            // Inner is read-locked
            let inner = self.inner.read().unwrap();
            new_alterations = mov.new_compile(&inner.board, &inner.metadata)
        }

        {   // Write phase
            // Tape is write-locked
            let mut tape = self.tape.write().unwrap();
            tape.write_all(&new_alterations);
        }

        // Cache Management
        // Tape read-locked, this syncs the head to the 'end of the tape', which should match the 
        // current write head position, which is presently at the end of the turn we just wrote.
        let position_hash: Zobrist = self.zobrist().position;

        // TODO: Ideally this is lazy, so we only update the board as we roll the associated
        // boardfamiliar forward.
        match self.atm.get(position_hash) {
            Some(cached_inner) => {
                // Atomic, TODO: Handle Result
                tracing::trace!("Cache hit {:?} -> {:?}", position_hash, to_fen_position(&cached_inner));
                _ = self.inner.replace(cached_inner.clone());
            },
            None => {
                tracing::trace!("Cache miss");
                // Inner is write-locked
                let mut inner = self.inner.write().unwrap();
                for alter in new_alterations {
                    inner.board.alter_mut(alter);
                    inner.metadata.alter_mut(alter);
                }

                tracing::trace!("Cache set {:?} -> {:?}", position_hash, to_fen_position(&inner.clone()));
                self.atm.set(position_hash, inner.clone());
            },
        }
    }

    // FIXME: this, if anything, should probably return a result type.
    pub fn unmake(&mut self) {
        tracing::debug!("Unmaking");

        /*
        * 1. For this to work better, it should use two familiars, a positionzobrist familiar and a
        *    gamestate familiar
        * 2. When unmaking, resummon the PZ (or maybe keep it live, idk, benchmark), send it to the
        *    previous move and check cache.
        * 3. Cache contains a GameState which can be assumed to be a Quintessence at the current
        *    tape position, this gives us a tape-agnostic cache, so multiple positions can share
        *    it. We use this to update our quintessence with a new on with this cache entry
        *    (perhaps stored by zobrist + offset?)
        * 4. If we cache miss, we grab our current quintessence and seek to the desired location.
        * 5. If we then `make` a new move (overwriting the old), we jump the tape to the position
        *    of the PZ familiar (which we've stored as a Quintessence of PZ on the position), and
        *    write starting from there, resummoning the PZ familiar to calculate and cache the new
        *    position.
        *       - Not sure if I should be resetting the HWM to 'truncate' the tape when we finish
        *       an unmake, maybe worth doing, it's more of an 'end of stack' marker, since this is
        *       increasingly 'just a stack' in this usecase. I suspect it'll be more tape-y with
        *       the variation side.
        *
        */

        let mut unmoves = vec![];

        { // Tape is write-locked
            let mut tape = self.tape.write().unwrap();
            loop { // this is inelegant, but hopefully effective.

                if tape.at_bot() {
                    // TODO: this probably should try to do cache magic in the tape first? IDK
                    panic!("Cannot unwind from Beginning of Tape");
                }

                tape.step_backward();

                let alter = tape.read();
                unmoves.push(alter);

                if matches!(alter, Alteration::Turn) {
                    break;
                }

            }
        }
        tracing::trace!("Tape unlocked from write");

        // FIXME: this causes a sync to the writehead, I don't love the spook, but I think it should
        // work for now
        let unmove_hash : Zobrist = self.zobrist().position;

        // TODO: This is an exact copy of the above, mod the #inverse calls on `alter`. definitely
        // could be extracted to something like `#withdraw_or_deposit(hash, &alters)`, unmake would
        // just map-inverse first.
        // TODO: I also think if the `Entry` type on Tape can be kept small enough, then the buffer
        // size can be quite large, which might mean indexing is worthwhile, something to think
        // about. I think that might be easier once I've moved to the generic familiar system I've
        // been cooking.
        match self.atm.get(unmove_hash) {
            Some(cached_inner) => {
                tracing::trace!("Unmake cache hit {:?}", unmove_hash);
                // Atomic, TODO: Handle Result
                _ = self.inner.replace(cached_inner.clone());
            },
            None => {
                tracing::trace!("Unmake cache miss");
                // Inner is write-locked
                let mut inner = self.inner.write().unwrap();

                for alter in unmoves {
                    inner.metadata.alter_mut(alter.inverse());
                    inner.board.alter_mut(alter.inverse());
                }

                self.atm.set(unmove_hash, inner.clone());
            }
        }
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
        // 3. the A2-G8 diag up to E6, exclusive (rook of opposite color can't check us from
        //    there), but the blocked squares are still interesting, so it might be worthwhile to
        //    optionally have a blocker mask.
        // 4. the A6-F1 diag
        // 5. A3, A6, B2, B7, D2, D7, E3, E6 - the knight-moves around the king.
        //
        // In order for the king to be checked, a piece of the correct type must be present on the
        // correct square, this calculates all the valid check squares, assuming that it is
        // possible for an enemy piece to get there. It _does not ensure that_. That is, this table
        // does not imply that there are moves which make put us in check, only that any of these
        // squares have _a_ piece which _could_ check the king _if_ it were there.
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
        hazel_bitboard::pextboard::attacks_for(piece, self.our_king(), blockers) & mask
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
            .map(|sq| { hazel_bitboard::pextboard::attacks_for(piece, sq, blockers) })
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
    use crate::coup::rep::MoveType;

    use hazel_core::square::*;
    use hazel_bitboard::bitboard;
    use hazel_bitboard::constants::masks::*;


    mod make_unmake {
        use super::*;

        #[test]
        #[tracing_test::traced_test]
        fn make_unmake() {
            let start = BEN::start_position();
            let moves = vec![
                Move::new(D2, D4, MoveType::DOUBLE_PAWN), Move::new(D7, D5, MoveType::DOUBLE_PAWN),
                Move::new(C1, F4, MoveType::QUIET), Move::new(E7, E6, MoveType::QUIET)
            ];

            let mut p = Position::with_moves(start, moves);
            let z_prior = p.zobrist();
            let m = Move::new(E2, E3, MoveType::QUIET);

            p.make(m);
            p.unmake();

            assert_eq!(z_prior.position, p.zobrist().position);
        }

        #[test]
        #[tracing_test::traced_test]
        fn unwinding_black_second_move_repro() {
            let movs = vec![
                Move::new(A2, A4, MoveType::DOUBLE_PAWN),
                Move::new(A7, A5, MoveType::DOUBLE_PAWN),
                Move::new(B2, B4, MoveType::DOUBLE_PAWN)
            ];

            let mut p = Position::with_moves(BEN::start_position(), movs);
            let p_prior = p.clone();

            p.make(Move::new(B7, B5, MoveType::DOUBLE_PAWN));

            p.unmake();


            assert_eq!(p_prior, p);
        }

        #[test]
        #[tracing_test::traced_test]
        fn perft_unwind_bug_repro() {
            let movs = vec![
                Move::new(A2, A4, MoveType::DOUBLE_PAWN),
                Move::new(A7, A5, MoveType::DOUBLE_PAWN),
            ];
            let mut p = Position::with_moves(BEN::start_position(), movs);


            let p_prior = p.clone();
            p.make(Move::new(B2, B4, MoveType::DOUBLE_PAWN));
            p.unmake();

            assert_eq!(p_prior, p);
        }
    }

    mod gamestate {
        use hazel_core::{ben::BEN, color::Color};
        use hazel_core::square::*;

        use crate::{board::PieceBoard, coup::rep::{Move, MoveType}, game::position::Position};

        #[test] // I entered the moves wrong, I don't know where.
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

            let position = Position::with_moves(start, moves);

            assert_eq!(position.board(), pb);
            assert_eq!(position.metadata(), target.metadata());
        }
    }

    mod pawns {
        use super::*;

        mod our_pawns {
            use super::*;

            #[test]
            fn startpos() {
                let pos = Position::new(BEN::start_position());
                assert_eq!(pos.our_pawns(), Color::WHITE.pawn_mask());
            }
        }

        mod our_pawn_direction {
            use super::*;

            #[test]
            fn startpos_white() {
                let pos = Position::new(BEN::start_position());
                assert_eq!(pos.our_pawn_direction(), Direction::N);
            }

            #[test]
            fn startpos_black() {
                let pos = Position::new(BEN::start_position());
                assert_eq!(pos.their_pawn_direction(), Direction::S);
            }
        }

        mod our_pawn_attacks {

            use super::*;

            #[test]
            fn startpos_white() {
                let pos = Position::new(BEN::start_position());
                let expected = *RANK_3;
                assert_eq!(pos.our_pawn_attacks(), expected);
            }

            #[test]
            fn startpos_black() {
                let pos = Position::new(BEN::start_position());
                let expected = *RANK_6;
                assert_eq!(pos.their_pawn_attacks(), expected);
            }
        }

        mod pawns_for {
            use super::*;

            #[test]
            fn startpos() {
                let pos = Position::new(BEN::start_position());
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
                let pos = Position::new(BEN::start_position());
                let expected = Bitboard::from(C3) | Bitboard::from(A3) | Bitboard::from(F3) | Bitboard::from(H3);
                assert_eq!(pos.our_knight_moves(), expected);
            }

            #[test]
            fn startpos_black() {
                let pos = Position::new(BEN::start_position());
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
                let pos = Position::new(BEN::new("8/8/1k6/P1P1p3/3K4/8/8/8 w - - 0 1"));
                assert_eq!(pos.our_king_attacks(), Bitboard::from(E5));
                assert_eq!(pos.their_king_attacks(),  Bitboard::from(A5) | Bitboard::from(C5));
            }

            #[test]
            fn black() {
                let pos = Position::new(BEN::new("8/8/1k6/P1P1p3/3K4/8/8/8 b - - 0 1"));
                assert_eq!(pos.our_king_attacks(),  Bitboard::from(A5) | Bitboard::from(C5));
                assert_eq!(pos.their_king_attacks(), Bitboard::from(E5));
            }
        }
    }

    mod bishop {
        use super::*;

        mod moves {

            use super::*;

            #[test]
            fn light_square() {
                let pos = Position::new(BEN::new("8/1b6/8/8/8/1B6/8/8 w - - 0 1"));
                assert_eq!(pos.our_bishop_moves(), bitboard!(A4, A2, C4, C2, D1, D5, E6, F7, G8));
                assert_eq!(pos.their_bishop_moves(),  bitboard!(A8, A6, C8, C6, D5, E4, F3, G2, H1));
            }

            #[test]
            fn dark_square() {
                let pos = Position::new(BEN::new("8/2b5/8/8/8/2B5/8/8 w - - 0 1"));
                assert_eq!(pos.our_bishop_moves(),  bitboard!(B2, D2, A1, E1, B4, A5, D4, E5, F6, G7, H8));
                assert_eq!(pos.their_bishop_moves(), bitboard!(B8, D8, B6, A5, D6, E5, F4, G3, H2));
            }

            #[test]
            fn with_blocker() {
                let pos = Position::new(BEN::new("8/2b5/8/4B3/8/8/8/8 w - - 0 1"));
                assert_eq!(pos.our_bishop_moves(),  bitboard!(C7, D6, F6, G7, H8, D4, C3, B2, A1, F4, G3, H2));
                assert_eq!(pos.their_bishop_moves(), bitboard!(B8, B6, D8, D6, E5, A5));
            }
        }
    }

    mod rook {
        use super::*;

        mod moves {


            use super::*;

            #[test]
            fn open_files() {
                let pos = Position::new(BEN::new("8/2r5/8/4R3/8/8/8/8 w - - 0 1"));
                assert_eq!(pos.our_rook_moves(), *RANK_5 ^ *E_FILE);
                assert_eq!(pos.their_rook_moves(),  *RANK_7 ^ *C_FILE);
            }


            #[test]
            fn with_blocker() {
                let pos = Position::new(BEN::new("8/1P1r1p2/8/8/1p1R1P2/8/8/8 w - - 0 1"));
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
                let pos = Position::new(BEN::new("8/2q5/8/4Q3/8/8/8/8 w - - 0 1"));
                assert_eq!(pos.our_queen_moves(), (*RANK_5 ^ *E_FILE) | (*A1_H8_DIAG ^ *B8_H2_DIAG) & !bitboard!(B8));
                assert_eq!(pos.their_queen_moves(),  (*RANK_7 ^ *C_FILE) | bitboard!(B8, D8, B6, D6, A5, E5));
            }


            #[test]
            fn with_blocker() {
                let pos = Position::new(BEN::new("8/1P1q1p2/8/8/1p1Q1P2/8/8/8 w - - 0 1"));
                assert_eq!(pos.our_queen_moves(), ((*RANK_4 ^ *D_FILE) | (*A1_H8_DIAG ^ *A7_G1_DIAG)) & !bitboard!(D8,A4, G4, H4));
                assert_eq!(pos.their_queen_moves(), bitboard!(A4, B5, B7, C6, C7, C8, D4, D5, D6, D8, E6, E7, E8, F5, F7, G4, H3));
            }
        }

    }

}
