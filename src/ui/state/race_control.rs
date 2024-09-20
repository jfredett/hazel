use crate::ui::state::grid::Grid;

#[derive(Debug, Default)]
pub struct RaceControl {
    grid: Grid,
}

impl RaceControl {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
        }
    }
}
