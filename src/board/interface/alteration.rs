use crate::types::Occupant;
use crate::notation::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alteration {
    Place { square: Square, occupant: Occupant },
    Remove { square: Square, occupant: Occupant },
    Done
}

impl Alteration {
    pub fn place(square: Square, occupant: Occupant) -> Self {
        Self::Place { square, occupant }
    }

    pub fn remove(square: Square, occupant: Occupant) -> Self {
        Self::Remove { square, occupant }
    }

    pub fn done() -> Self {
        Self::Done
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Place { square, occupant } => Self::Remove { square: *square, occupant: *occupant },
            Self::Remove { square, occupant } => Self::Place { square: *square, occupant: *occupant },
            _ => self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place() {
        let alteration = Alteration::place(A1, Occupant::black_king());
        assert_eq!(alteration, Alteration::Place { square: A1, occupant: Occupant::black_king() });
    }

    #[test]
    fn remove() {
        let alteration = Alteration::remove(A1, Occupant::black_king());
        assert_eq!(alteration, Alteration::Remove { square: A1, occupant: Occupant::black_king() });
    }

    #[test]
    fn done() {
        let alteration = Alteration::done();
        assert_eq!(alteration, Alteration::Done);
    }

    #[test]
    fn inverse() {
        let place = Alteration::place(A1, Occupant::black_king());
        let remove = Alteration::remove(A1, Occupant::black_king());
        assert_eq!(place.inverse(), remove);
        assert_eq!(remove.inverse(), place);
    }
}
