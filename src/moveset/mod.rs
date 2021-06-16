use crate::{constants::Piece, movement::{Move, MoveType}};


/// a container for moves
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct MoveList {
    moves: Vec<Move>
}

impl MoveList {
    pub fn empty() -> MoveList {
        return MoveList { moves: vec![] };
    }
    
    pub fn add_move(&mut self, source: usize, target: usize) {
        self.moves.push(Move::from(source as u16, target as u16, false, MoveType::quiet().bits()));
    }
    
    pub fn add_capture(&mut self, source: usize, target: usize) {
        self.moves.push(Move::from(source as u16, target as u16, false, MoveType::capture().bits()));
    }

    pub fn add_check(&mut self, source: usize, target: usize) {
        self.moves.push(Move::from(source as u16, target as u16, false, MoveType::check().bits()));
    }

    pub fn add_attack(&mut self, source: usize, target: usize) {
        self.moves.push(Move::from(source as u16, target as u16, false, MoveType::attack().bits()));
    }
    
    pub fn add_promotion(&mut self, source: usize, target: usize) {
        self.moves.push(Move::from(source as u16, target as u16, true, Piece::Queen as u16));
        self.moves.push(Move::from(source as u16, target as u16, true, Piece::Rook as u16));
        self.moves.push(Move::from(source as u16, target as u16, true, Piece::Bishop as u16));
        self.moves.push(Move::from(source as u16, target as u16, true, Piece::Knight as u16));
    }
}


#[cfg(test)]
mod test {
    use either::Either;

    use crate::{constants::{NOTATION_TO_INDEX, Piece}, movement::Move};

    use super::*;
    
    /// passes if the left is a subset of the right
    macro_rules! assert_is_subset {
        ($left:expr, $right:expr) => (
            let mut missing = vec![];
            for m in $left {
               if !$right.contains(&m) {
                    missing.push(m);
               } 
            } 
            
            if missing.len() > 0 {
                panic!("assertion failed, set difference: {:?}", missing);
            }
        );
    }
    
    /// This is essentially assert_eq but doesn't care about order differences
    macro_rules! assert_are_equal_sets {
        ($left:expr, $right:expr) => (
            assert_is_subset!(&$left, &$right);
            assert_is_subset!(&$right, &$left);
        );
    }
    
    #[test]
    fn add_promotion_adds_promtion_moves_refactor() {
        let mut ml = MoveList::empty();
        // h7->h8
        ml.add_promotion(56, 64);
        
        let expected = vec![
            Move::from(56, 64, true, Piece::Queen as u16),
            Move::from(56, 64, true, Piece::Rook as u16),
            Move::from(56, 64, true, Piece::Bishop as u16),
            Move::from(56, 64, true, Piece::Knight as u16)
        ];

        assert_are_equal_sets!(ml.moves, expected);
    }
    
    #[test]
    fn add_capture_adds_capture() {
        let mut ml = MoveList::empty();
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
        let mut ml = MoveList::empty();
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
        let mut ml = MoveList::empty();
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
        let mut ml = MoveList::empty();
        let expected_move = Move::from_notation("b4", "d5", Either::Left(MoveType::CHECK));

        ml.add_check(
            NOTATION_TO_INDEX("b4"),
            NOTATION_TO_INDEX("d5")
        );
        
        assert_eq!(ml.moves.len(), 1);
        assert!(ml.moves.contains(&expected_move));
    }
}