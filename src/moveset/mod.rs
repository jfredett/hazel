use crate::{constants::Piece, movement::Move};


/// a container for moves
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct MoveList {
    moves: Vec<Move>
}

impl MoveList {
    pub fn empty() -> MoveList {
        return MoveList { moves: vec![] };
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

    use crate::{constants::Piece, movement::Move};

    use super::*;
    
    #[test]
    fn add_promotion_adds_promtion_moves() {
        let mut ml = MoveList::empty();
        // h7->h8
        ml.add_promotion(56, 64);
        
        let expected = vec![
            Move::from(56, 64, true, Piece::Queen as u16),
            Move::from(56, 64, true, Piece::Rook as u16),
            Move::from(56, 64, true, Piece::Bishop as u16),
            Move::from(56, 64, true, Piece::Knight as u16)
        ];
        
        let mut should_be_present_but_missing = vec![];
        let mut should_be_missing_but_present = vec![];

        let moves = ml.moves.clone();

        for m in ml.moves {
            if !expected.contains(&m) {
                should_be_missing_but_present.push(m);
            }
        }
        
        for m in expected {
            if !moves.contains(&m) {
                should_be_present_but_missing.push(m);
            }
        }
        
        assert_eq!(should_be_present_but_missing, vec![]);
        assert_eq!(should_be_missing_but_present, vec![]);

    }
}