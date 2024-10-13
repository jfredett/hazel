#![allow(dead_code, unused_imports)]

pub mod line;
pub mod variation;
pub mod interface;

use line::Line;
use variation::Variation;

struct Game {
    mainline: Line,
    variations: Vec<Variation>,
}
