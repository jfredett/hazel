use crate::ui::model::occupant::Occupant;

#[derive(Debug, Clone, Copy)]
pub enum Alteration {
    Move((usize, usize), (usize, usize)),
    Remove((usize, usize)),
    Place((usize, usize), Occupant)
}
