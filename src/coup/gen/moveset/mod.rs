use crate::{
    constants::{Color, Piece},
    movement::{Move, MoveType},
};

/// a container for moves
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct MoveSet {
    // FIXME: Move this to a smallvec/stack allocated vector
    pub(crate) moves: [Vec<Move>; 6],
}

impl MoveSet {
    pub fn empty() -> MoveSet {
        MoveSet {
            moves: [vec![], vec![], vec![], vec![], vec![], vec![]],
        }
    }

    pub fn merge(&mut self, other: MoveSet) -> &mut MoveSet {
        for (i, subset) in self.moves.iter_mut().enumerate() {
            for m in other.moves[i].clone() {
                subset.push(m);
            }
        }
        self
    }

    /// Adds a quiet move from the source square to the target square
    pub fn add_move(&mut self, piece: Piece, source: usize, target: usize) {
        self.moves[piece as usize].push(Move::from(source as u16, target as u16, MoveType::QUIET));
    }

    /// Adds a short castle move
    pub fn add_short_castle(&mut self, color: Color) {
        self.moves[Piece::King as usize].push(Move::short_castle(color));
    }

    /// Adds a long castle move
    pub fn add_long_castle(&mut self, color: Color) {
        self.moves[Piece::King as usize].push(Move::long_castle(color));
    }

    pub fn add_pawn_double_move(&mut self, source: usize, color: Color) {
        let offset: isize = match color {
            Color::WHITE => 16,
            Color::BLACK => -16,
        };
        self.moves[Piece::Pawn as usize].push(Move::from(
            source as u16,
            (source as isize + offset) as u16,
            MoveType::DOUBLE_PAWN,
        ))
    }

    /// Adds a capture move from the source square to the target square
    pub fn add_capture(&mut self, piece: Piece, source: usize, target: usize) {
        self.moves[piece as usize].push(Move::from(
            source as u16,
            target as u16,
            MoveType::CAPTURE,
        ));
    }

    pub fn add_en_passant_capture(&mut self, source: usize, target: usize) {
        self.moves[Piece::Pawn as usize].push(Move::from(
            source as u16,
            target as u16,
            MoveType::EP_CAPTURE,
        ));
    }

    /// Adds all promotion moves from the source square to the target square
    /// FIXME: Don't pass a bool, have a separate method for promotion-captures.
    pub fn add_promotion(&mut self, source: usize, target: usize, is_capture: bool) {
        if is_capture {
            self.moves[Piece::Pawn as usize].push(Move::from(
                source as u16,
                target as u16,
                MoveType::PROMOTION_CAPTURE_QUEEN,
            ));
            self.moves[Piece::Pawn as usize].push(Move::from(
                source as u16,
                target as u16,
                MoveType::PROMOTION_CAPTURE_ROOK,
            ));
            self.moves[Piece::Pawn as usize].push(Move::from(
                source as u16,
                target as u16,
                MoveType::PROMOTION_CAPTURE_BISHOP,
            ));
            self.moves[Piece::Pawn as usize].push(Move::from(
                source as u16,
                target as u16,
                MoveType::PROMOTION_CAPTURE_KNIGHT,
            ));
        } else {
            self.moves[Piece::Pawn as usize].push(Move::from(
                source as u16,
                target as u16,
                MoveType::PROMOTION_QUEEN,
            ));
            self.moves[Piece::Pawn as usize].push(Move::from(
                source as u16,
                target as u16,
                MoveType::PROMOTION_ROOK,
            ));
            self.moves[Piece::Pawn as usize].push(Move::from(
                source as u16,
                target as u16,
                MoveType::PROMOTION_BISHOP,
            ));
            self.moves[Piece::Pawn as usize].push(Move::from(
                source as u16,
                target as u16,
                MoveType::PROMOTION_KNIGHT,
            ));
        }
    }

    pub fn contains(&self, m: &Move) -> bool {
        self.moves.iter().any(|v| v.contains(m))
    }

    pub fn len(&self) -> usize {
        self.moves.iter().map(|e| e.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.moves.iter().all(|e| e.is_empty())
    }

    pub fn find_by_target(&self, piece: Piece, idx: usize) -> Search {
        let mut movs = vec![];
        for mov in &self.moves[piece as usize] {
            let target_idx = mov.target_idx();
            if target_idx == idx {
                movs.push(*mov);
            }
        }

        if movs.is_empty() {
            return Search::Empty;
        }
        if movs.len() == 1 {
            return Search::Unambiguous(movs[0]);
        }
        Search::Ambiguous(movs)
    }

    pub fn as_vec(&self) -> Vec<Move> {
        self.moves.concat()
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Search {
    Unambiguous(Move),
    Ambiguous(Vec<Move>),
    Empty,
}
#[cfg(test)]
mod test {
    use either::Either;
    use crate::assert_are_equal_sets;

    use crate::{
        assert_is_subset,
        movement::Move,
    };

    use super::*;

    #[test]
    fn add_promotion_adds_promtion_moves_refactor() {
        let mut ml = MoveSet::empty();

        ml.add_promotion(56, 64, false);

        let expected = vec![
            Move::from(56, 64, MoveType::PROMOTION_QUEEN),
            Move::from(56, 64, MoveType::PROMOTION_ROOK),
            Move::from(56, 64, MoveType::PROMOTION_BISHOP),
            Move::from(56, 64, MoveType::PROMOTION_KNIGHT),
        ];

        assert_are_equal_sets!(ml.moves[Piece::Pawn as usize], expected);
    }

    #[test]
    fn add_capture_adds_capture() {
        let mut ml = MoveSet::empty();
        let expected_move = Move::from_notation("b4", "d5", MoveType::CAPTURE);

        ml.add_capture(
            Piece::Bishop,
            B4,
            D5,
        );

        assert_eq!(ml.moves[Piece::Bishop as usize].len(), 1);
        assert!(ml.moves[Piece::Bishop as usize].contains(&expected_move));
    }

    #[test]
    fn add_move_adds_move() {
        let mut ml = MoveSet::empty();
        let expected_move = Move::from_notation("b4", "d5", MoveType::QUIET);

        ml.add_move(
            Piece::Pawn,
            B4,
            D5,
        );

        assert_eq!(ml.moves[Piece::Pawn as usize].len(), 1);
        assert!(ml.moves[Piece::Pawn as usize].contains(&expected_move));
    }
}
