use crate::types::Occupant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alteration {
    Place { index: usize, occupant: Occupant },
    Remove { index: usize, occupant: Occupant },
    Done
}

impl Alteration {
    pub fn place(index: usize, occupant: Occupant) -> Self {
        Self::Place { index, occupant }
    }

    pub fn remove(index: usize, occupant: Occupant) -> Self {
        Self::Remove { index, occupant }
    }

    pub fn done() -> Self {
        Self::Done
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Place { index, occupant } => Self::Remove { index: *index, occupant: *occupant },
            Self::Remove { index, occupant } => Self::Place { index: *index, occupant: *occupant },
            _ => self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place() {
        let alteration = Alteration::place(0, Occupant::black_king());
        assert_eq!(alteration, Alteration::Place { index: 0, occupant: Occupant::black_king() });
    }

    #[test]
    fn remove() {
        let alteration = Alteration::remove(0, Occupant::black_king());
        assert_eq!(alteration, Alteration::Remove { index: 0, occupant: Occupant::black_king() });
    }

    #[test]
    fn done() {
        let alteration = Alteration::done();
        assert_eq!(alteration, Alteration::Done);
    }

    #[test]
    fn inverse() {
        let place = Alteration::place(0, Occupant::black_king());
        let remove = Alteration::remove(0, Occupant::black_king());
        assert_eq!(place.inverse(), remove);
        assert_eq!(remove.inverse(), place);
    }
}