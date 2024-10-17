use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
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
