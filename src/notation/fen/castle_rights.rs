use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastleRights {
    pub white_short: bool,
    pub white_long: bool,
    pub black_short: bool,
    pub black_long: bool,
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rights = String::new();
        if self.white_short {
            rights.push('K');
        }
        if self.white_long {
            rights.push('Q');
        }
        if self.black_short {
            rights.push('k');
        }
        if self.black_long {
            rights.push('q');
        }
        write!(f, "{}", rights)
    }
}

impl From<u32> for CastleRights {
    fn from(castling: u32) -> Self {
        CastleRights {
            white_short: castling & 0b1000 != 0,
            white_long: castling & 0b0100 != 0,
            black_short: castling & 0b0010 != 0,
            black_long: castling & 0b0001 != 0,
        }
    }
}


impl From<CastleRights> for u32 {
    fn from(rights: CastleRights) -> u32 {
        let mut castling = 0;
        if rights.white_short {
            castling |= 0b1000;
        }
        if rights.white_long {
            castling |= 0b0100;
        }
        if rights.black_short {
            castling |= 0b0010;
        }
        if rights.black_long {
            castling |= 0b0001;
        }
        castling
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_test() {
        let rights = CastleRights {
            white_short: true,
            white_long: true,
            black_short: true,
            black_long: true,
        };
        assert_eq!(rights.to_string(), "KQkq");

        let rights = CastleRights {
            white_short: false,
            white_long: false,
            black_short: false,
            black_long: false,
        };
        assert_eq!(rights.to_string(), "");
    }
}
