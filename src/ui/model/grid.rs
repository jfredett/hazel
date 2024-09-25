#![allow(dead_code)]
use crate::ui::model::entry::Entry;

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
