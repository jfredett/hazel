use crate::ui::state::entry::Entry;

#[derive(Debug, Default)]
pub struct Grid {
    entries: Vec<Entry>
}

impl Grid {
    pub fn new() -> Self {
        Self {
            entries: vec![]
        }
    }
}
