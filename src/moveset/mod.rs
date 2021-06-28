use crate::{constants::{Color, Piece}, movement::{Move, MoveType}};


/// a container for moves
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct MoveSet {
    pub(crate) moves: Vec<Move>
}

impl MoveSet {
    pub fn empty() -> MoveSet {
        return MoveSet { moves: vec![] };
    }
    
    /// Adds a quiet move from the source square to the target square
    pub fn add_move(&mut self, source: usize, target: usize) {
        // dbg!("shh", source, target);
        self.moves.push(Move::from(source as u16, target as u16, false, MoveType::quiet().bits()));
    }
    
    /// Adds a short castle move
    pub fn add_short_castle(&mut self, color: Color) {
        self.moves.push(Move::short_castle(color));
    }
    
    /// Adds a long castle move
    pub fn add_long_castle(&mut self, color: Color) {
        self.moves.push(Move::long_castle(color));     
    }
    
    /// Adds a capture move from the source square to the target square
    pub fn add_capture(&mut self, source: usize, target: usize) {
        // dbg!("cap", source, target);
        self.moves.push(Move::from(source as u16, target as u16, false, MoveType::capture().bits()));
    }

    /// Adds a check move from the source square to the target square
    pub fn add_check(&mut self, source: usize, target: usize) {
        // dbg!("chk", source, target);
        self.moves.push(Move::from(source as u16, target as u16, false, MoveType::check().bits()));
    }

    /// Adds a attacking move from the source square to the target square
    pub fn add_attack(&mut self, source: usize, target: usize) {
        // dbg!("att", source, target);
        self.moves.push(Move::from(source as u16, target as u16, false, MoveType::attack().bits()));
    }
    
    /// Adds all promotion moves from the source square to the target square
    pub fn add_promotion(&mut self, source: usize, target: usize) {
        // dbg!("prom", source, target);
        self.moves.push(Move::from(source as u16, target as u16, true, Piece::Queen as u16));
        self.moves.push(Move::from(source as u16, target as u16, true, Piece::Rook as u16));
        self.moves.push(Move::from(source as u16, target as u16, true, Piece::Bishop as u16));
        self.moves.push(Move::from(source as u16, target as u16, true, Piece::Knight as u16));
    }
    
    pub fn contains(&self, m : &Move) -> bool {
        self.moves.contains(m)
    }
    
    pub fn len(&self) -> usize {
        self.moves.len()
    }
}

impl Iterator for MoveSet {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&m) = self.moves.iter().next() {
            Some(m)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod test {
    use either::Either;

    use crate::{assert_is_subset, constants::{NOTATION_TO_INDEX, Piece}, movement::Move};

    use super::*;
    
    
    #[test]
    fn add_promotion_adds_promtion_moves_refactor() {
        let mut ml = MoveSet::empty();
        
        ml.add_promotion(56, 64);
        
        let expected = vec![
            Move::from(56, 64, true, Piece::Queen as u16),
            Move::from(56, 64, true, Piece::Rook as u16),
            Move::from(56, 64, true, Piece::Bishop as u16),
            Move::from(56, 64, true, Piece::Knight as u16)
        ];

        assert_is_subset!(&ml.moves, &expected);
        assert_is_subset!(&expected, &ml.moves);
    }
    
    
    
    #[test]
    fn add_capture_adds_capture() {
        let mut ml = MoveSet::empty();
        let expected_move = Move::from_notation("b4", "d5", Either::Left(MoveType::CAPTURE));

        ml.add_capture(
            NOTATION_TO_INDEX("b4"),
            NOTATION_TO_INDEX("d5")
        );
        
        assert_eq!(ml.moves.len(), 1);
        assert!(ml.moves.contains(&expected_move));
    }

    #[test]
    fn add_move_adds_move() {
        let mut ml = MoveSet::empty();
        let expected_move = Move::from_notation("b4", "d5", Either::Left(MoveType::quiet()));

        ml.add_move(
            NOTATION_TO_INDEX("b4"),
            NOTATION_TO_INDEX("d5")
        );
        
        assert_eq!(ml.moves.len(), 1);
        assert!(ml.moves.contains(&expected_move));
    }

    #[test]
    fn add_attack_adds_attack() {
        let mut ml = MoveSet::empty();
        let expected_move = Move::from_notation("b4", "d5", Either::Left(MoveType::ATTACK));

        ml.add_attack(
            NOTATION_TO_INDEX("b4"),
            NOTATION_TO_INDEX("d5")
        );
        
        assert_eq!(ml.moves.len(), 1);
        assert!(ml.moves.contains(&expected_move));
    }
    #[test]
    fn add_check_adds_check() {
        let mut ml = MoveSet::empty();
        let expected_move = Move::from_notation("b4", "d5", Either::Left(MoveType::CHECK));

        ml.add_check(
            NOTATION_TO_INDEX("b4"),
            NOTATION_TO_INDEX("d5")
        );
        
        assert_eq!(ml.moves.len(), 1);
        assert!(ml.moves.contains(&expected_move));
    }
}