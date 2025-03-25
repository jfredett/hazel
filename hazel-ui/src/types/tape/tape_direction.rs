
#[derive(PartialEq, Debug, Clone, Copy, Eq)]
pub enum TapeDirection {
    Advancing,
    Rewinding
}

impl TapeDirection {
    pub fn advancing(self) -> bool {
        self == TapeDirection::Advancing
    }

    pub fn rewinding(self) -> bool {
        self == TapeDirection::Rewinding
    }
}
