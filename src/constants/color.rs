#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    WHITE = 0,
    BLACK = 1
}

pub const COLORS : [Color; 2] = [
    Color::WHITE,
    Color::BLACK
];