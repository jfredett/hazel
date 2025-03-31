#![allow(dead_code, unused_imports)]

use hazel_basic::{ben::BEN, color::Color, interface::Query, occupant::Occupant, piece::Piece};
use hazel_bitboard::bitboard::Bitboard;
use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::opt, IResult};

use hazel_representation::coup::rep::{Move, MoveType};

use crate::pgn::parsers::*;

use hazel_basic::square::*;

mod disambiguator;

use disambiguator::Disambiguator;

#[derive(Debug, Clone)]
pub struct SAN {
    source_piece: Option<Piece>,
    captured_piece: Option<Piece>,
    disambiguator: Option<Disambiguator>,
    capturing: bool,
    source_sq: Option<Square>,
    target_sq: Option<Square>,
    ambiguous_sq: Option<Square>,
    promotion: Option<Piece>,
    castle_short: bool,
    castle_long: bool,
    context: BEN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleMove {
    Short,
    Long
}

impl CastleMove {
    pub fn is_short(&self) -> bool {
        matches!(self, CastleMove::Short)
    }

    pub fn is_long(&self) -> bool {
        matches!(self, CastleMove::Long)
    }
}

impl SAN {
    pub fn is_ambiguous(&self) -> bool {
        !self.is_castle() && (self.source_sq.is_none() || self.ambiguous_sq.is_some())
    }

    pub fn is_castle(&self) -> bool {
        self.castle_short || self.castle_long
    }

    pub fn new(fen: impl Into<BEN>) -> Self {
        SAN {
            source_piece: None,
            captured_piece: None,
            disambiguator: None,
            capturing: false,
            source_sq: None,
            target_sq: None,
            ambiguous_sq: None,
            promotion: None,
            castle_short: false,
            castle_long: false,
            context: fen.into(),
        }
    }

    fn promotion(input: &str) -> IResult<&str, Piece> {
        let (input, _) = char('=')(input)?;
        Piece::parse(input)
    }

    pub fn castle(input: &str) -> IResult<&str, CastleMove> {
        // order matters.
        let (input, long) = opt(alt((tag("O-O-O"), tag("0-0-0"), tag("o-o-o"))))(input)?;
        let (input, short) = opt(alt((tag("O-O"), tag("0-0"), tag("o-o"))))(input)?;

        let ret = if short.is_some() {
            CastleMove::Short
        } else if long.is_some() {
            CastleMove::Long
        } else {
            return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)));
        };
        Ok((input, ret))
    }

    pub fn parse(input: &str, context: impl Into<BEN>) -> IResult<&str, Self> {
        let (input, castling_move) = opt(Self::castle)(input)?;
        if let Some(m) = castling_move {
            return Ok((input, SAN {
                source_piece: None,
                captured_piece: None,
                disambiguator: None,
                capturing: false,
                source_sq: None,
                target_sq: None,
                ambiguous_sq: None,
                promotion: None,
                castle_short: m.is_short(),
                castle_long: m.is_long(),
                context: context.into(),
            }));
        }


        let (input, piece) = opt(Piece::parse)(input)?;
        // We either have a disambiguator or this is the target square
        let (input, disambiguator) = opt(Disambiguator::parse)(input)?;
        // if there is an `x`, it's a capture, so we know the previous item must be a
        // source_disambiguator
        let (input, is_capture) = opt(char('x'))(input)?;
        let capturing = is_capture.is_some();
        // We can just try to parse the target square if it's there
        let (input, ambiguous_sq) = opt(Square::parse)(input)?;
        // And try to parse the promotion if it's there
        let (input, promotion) = opt(Self::promotion)(input)?;

        let mut san = SAN {
            source_piece: piece.or(Some(Piece::Pawn)),
            captured_piece: None,
            disambiguator,
            capturing,
            source_sq: None,
            target_sq: None,
            ambiguous_sq,
            castle_short: false,
            castle_long: false,
            promotion,
            context: context.into(),
        };
        let _ = san.disambiguate();

        Ok((input, san))
    }

    /// Disambiguate the _source square_ and _only_ the source square. This rest of disambiguation
    /// is
    fn disambiguate(&mut self) -> Result<(), SANConversionError> {
        // <Piece><Disambiguator><x?><AmbiguousSq><Promotion?>
        // We Know Piece, if we're capturing, if we're promoting, and what the disambiguator and
        // ambiguous square have parsed to. We need to figure out the source and target square, and
        // the captured piece if we're capturing.

        // Calculating the target square is relatively easy, we only need to consider 8 main cases
        //
        // Cases:
        //
        // Capturing | Disambiguator | AmbiguousSq  => Target Sq      | Example
        //   true    | None          | None         => Not Possible   | Err: "Qx", invalid, missing target
        //   true    | Some(Sq(s))   | None         => Not Possible   | Err: "Qd1x", invalid, missing target
        //   true    | Some(Sq(s))   | Some(t)      => Some(t)        | Qd1xd4
        //   true    | None          | Some(t)      => Some(t)        | Qxd4
        //   false   | Some(Sq(s))   | Some(t)      => Some(t)        | Qd1d4
        //   false   | None          | Some(t)      => Not Possible   | Impossible to parse this in the current parser
        //   false   | Some(Sq(s))   | None         => Some(s)        | Qd1
        //   false   | None          | None         => Not Possible   | Err: "Q", invalid, missing target
        //
        // If the disambiguator is ever a non-square, then it must not be the target.
        if self.castle_short || self.castle_long {
            // these moves are already not ambiguous.
            return Ok(());
        }

        self.target_sq = Some(if !self.capturing && self.ambiguous_sq.is_none() {
            let sq = self.disambiguator.unwrap().square();
            // We have consumed this, so we need to set it to None
            self.disambiguator = None;
            sq
        } else {
            self.ambiguous_sq.unwrap()
        });


        // This square is no longer ambiguous.
        self.ambiguous_sq = None;

        // This may have left some `nones` floating around. We need to calculate the source square
        // if we haven't already

        let source_occupant = Occupant::Occupied(self.source_piece.unwrap(), self.context.side_to_move());

        let mut possible_source_squares = vec![];
        for sq in self.target_sq.unwrap().unmoves_for(self.capturing, &self.source_piece.unwrap(), &self.context.side_to_move()) {
            if self.context.get(sq) == source_occupant {
                possible_source_squares.push(sq);
            }
        }

        // BUG: issue for pawn pushes is that we're using the disambiguator twice
        let m = match self.disambiguator {
            Some(disambiguator) => {
                match disambiguator {
                    Disambiguator::File(file) => {
                        // The disambiguator is a file, so we need to find the piece on that file.
                        Square::along_file(file).find(|sq| {
                            self.context.get(sq) == source_occupant
                        }).unwrap()
                    },
                    Disambiguator::Rank(rank) => {
                        Square::along_rank(rank).find(|sq| {
                            self.context.get(sq) == source_occupant
                        }).unwrap()
                    },
                    Disambiguator::Sq(sq) => {
                        // Easy, we are told the square.
                        sq
                    }
                }
            },
            None => {
                // just search everything, the move claims it is unambiguous.
                match possible_source_squares.len() {
                    0 => {
                        return Err(SANConversionError::NoPossibleSourceSquares);
                    }
                    1 => possible_source_squares[0],
                    _ => {
                        // TODO: Disambiguate blocked slider situations.

                        // Take each move and try to slide the piece to the target square, if it
                        // can, it is the correct move.
                        //
                        // Pawns and Knights need not apply (would always require a disambiguator),
                        // Kings can never be ambiguous with multiple source moves, since there is
                        // only ever one of them on the board.
                        //
                        // I'm going to use the bitboard API for this because I have `pextboards`
                        // do the calculation already, but it does need a blocker bitboard we'll
                        // have to construct. PGN is such a stupid format.
                        let mut blocks = Bitboard::empty();
                        for sq in Square::by_rank_and_file() {
                            if let Occupant::Occupied(..) = self.context.get(sq) {
                                blocks.set(sq);
                            }
                        }

                        self.slide_attacks(&possible_source_squares, blocks)?
                    }
                }
            }
        };
        self.source_sq = Some(m);

        // At this point, we know source and target squares, and we know if we're capturing. So we
        // can populate the captured piece if we are
        if self.capturing {
            let target_occupant = self.context.get(self.target_sq.unwrap());

            match target_occupant {
                Occupant::Occupied(piece, _) => {
                    self.captured_piece = Some(piece);
                },
                _ => {
                    return Err(SANConversionError::MissingTargetPiece);
                }
            }
        }

        Ok(())
    }

    fn slide_attacks(&self, possible_source_squares: &[Square], blocks: Bitboard) -> Result<Square, SANConversionError> {
        match self.source_piece.unwrap() {
            Piece::Rook | Piece::Bishop | Piece::Queen => {
                for source_sq in possible_source_squares {
                    let attacks = hazel_bitboard::pextboard::attacks_for(self.source_piece.unwrap(), *source_sq, blocks);
                    if attacks.is_set(self.target_sq.unwrap()) {
                        return Ok(*source_sq);
                    }
                }
                Err(SANConversionError::IsAmbiguous("All source squares are blocked"))
            },
            _ => {
                Err(SANConversionError::IsAmbiguous("Multiple possible source squares"))
            }
        }
    }
}

#[derive(Debug, Default)]
pub enum SANConversionError {
    #[default] UnknownError,
    MissingSourceSq,
    // FIXME: I don't know if I want to detect an illegal move at the parser level.
    MissingTargetPiece,
    MissingMoveType,
    IsAmbiguous(&'static str),
    NoPossibleSourceSquares,
    Unreachable,
    /// This is a catch-all for errors that don't fit into the other categories. They should be
    /// factored out when possible.
    NewError(&'static str),
}

impl TryFrom<SAN> for Move {
    type Error = SANConversionError;

    // This expects that the context has already been embedded and the movetype is disambiguated.
    // That should happen at parse-time for SAN, since we only see this in PGNs, really. UCI does a
    // LAN-style which is much easier to convert since the source_sq is already there.
    fn try_from(san: SAN) -> Result<Move, SANConversionError> {
        if san.is_ambiguous() {
            return Err(SANConversionError::IsAmbiguous("SAN is ambiguous"));
        }

        // TODO: Make the naming consistent
        if san.castle_short {
            match san.context.side_to_move() {
                Color::WHITE => { return Ok(Move::new(E1, G1, MoveType::SHORT_CASTLE)); },
                Color::BLACK => { return Ok(Move::new(E8, G8, MoveType::SHORT_CASTLE)); },
            }
        }

        if san.castle_long {
            match san.context.side_to_move() {
                Color::WHITE => { return Ok(Move::new(E1, C1, MoveType::LONG_CASTLE)); },
                Color::BLACK => { return Ok(Move::new(E8, C8, MoveType::LONG_CASTLE)); },
            }
        }

        let mut mov = Move::new(san.source_sq.unwrap(), san.target_sq.unwrap(), MoveType::UCI_AMBIGUOUS);

        let metadata = mov.disambiguate(&san.context).unwrap();

        let metadata = if metadata.is_promotion() {
            match san.promotion.unwrap() {
                Piece::Queen => {
                    if san.capturing {
                        MoveType::PROMOTION_CAPTURE_QUEEN
                    } else {
                        MoveType::PROMOTION_QUEEN
                    }
                },
                Piece::Rook => {
                    if san.capturing {
                        MoveType::PROMOTION_CAPTURE_ROOK
                    } else {
                        MoveType::PROMOTION_ROOK
                    }
                },
                Piece::Bishop => {
                    if san.capturing {
                        MoveType::PROMOTION_CAPTURE_BISHOP
                    } else {
                        MoveType::PROMOTION_BISHOP
                    }
                },
                Piece::Knight => {
                    if san.capturing {
                        MoveType::PROMOTION_CAPTURE_KNIGHT
                    } else {
                        MoveType::PROMOTION_KNIGHT
                    }
                },
                _ => {
                    return Err(SANConversionError::UnknownError);
                }
            }
        } else { metadata };

        mov.set_metadata(metadata);

        Ok(mov)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hazel_representation::game::variation::Variation;
    use crate::uci::UCI;
    use hazel_basic::constants::START_POSITION_FEN;


    #[test]
    fn new_works() {
        let ben = BEN::start_position();
        let san = SAN::new(ben);
        assert_eq!(san.context, ben);
        assert_eq!(san.source_piece, None);
        assert_eq!(san.captured_piece, None);
        assert_eq!(san.disambiguator, None);
        assert!(!san.capturing);
        assert_eq!(san.source_sq, None);
        assert_eq!(san.target_sq, None);
        assert_eq!(san.ambiguous_sq, None);
        assert_eq!(san.promotion, None);
        assert!(!san.castle_short);
        assert!(!san.castle_long);
        assert!(san.is_ambiguous());
    }

    macro_rules! assert_parses {
        ($input:expr, $expected:expr, $fen:expr, $moves:expr) => {
            let mut context = Variation::default();
            context.setup(BEN::new($fen)).commit();
            for m in $moves.iter().map(|m| UCI::try_from(*m).unwrap()) {
                context.make(m.into());
            }
            let (_, san) = SAN::parse($input, context.current_position()).unwrap();
            assert_eq!(Move::try_from(san).unwrap(), $expected);
    };
        ($input:expr, $expected:expr, $fen:expr) => {
            assert_parses!($input, $expected, $fen, Vec::<&str>::new());
        };
        ($input:expr, $expected:expr) => {
            assert_parses!($input, $expected, START_POSITION_FEN);
        };
    }

    mod piece_moves {
        use super::*;


        #[test]
        fn parses_san_piece_move() {
            assert_parses!("Nf3", Move::new(G1, F3, MoveType::QUIET));
        }

        #[test]
        fn parses_san_piece_capture() {
            assert_parses!("Qxd4", Move::new(D1, D4, MoveType::CAPTURE), "8/8/8/8/3p4/8/8/3Q4 w - - 0 1");
        }

        #[test]
        fn parses_san_piece_capture_with_disambiguator() {
            assert_parses!("Qg1xd4", Move::new(G1, D4, MoveType::CAPTURE), "k7/8/8/8/3p2Q1/8/8/K2Q2Q1 w - - 0 1");
        }

        #[test]
        fn parses_san_piece_capture_with_disambiguator_and_file() {
            assert_parses!("Q4xd4", Move::new(G4, D4, MoveType::CAPTURE), "k7/8/8/8/3p2Q1/8/8/K2Q2Q1 w - - 0 1");
        }

        #[test]
        fn parses_san_piece_capture_with_disambiguator_and_rank() {
            assert_parses!("Qdxd4", Move::new(D1, D4, MoveType::CAPTURE), "k7/8/8/8/3p2Q1/8/8/K2Q2Q1 w - - 0 1");
        }

        // NOTE: Subtle change in FEN below, removing the pawn.
        #[test]
        fn parses_non_capture_with_disambiguator() {
            assert_parses!("Qg1d4", Move::new(G1, D4, MoveType::QUIET), "k7/8/8/8/6Q1/8/8/K2Q2Q1 w - - 0 1");
        }

        #[test]
        fn parses_non_capture_with_disambiguator_and_file() {
            assert_parses!("Q4d4", Move::new(G4, D4, MoveType::QUIET), "k7/8/8/8/6Q1/8/8/K2Q2Q1 w - - 0 1");
        }

        #[test]
        fn parses_non_capture_with_disambiguator_and_rank() {
            assert_parses!("Qdd4", Move::new(D1, D4, MoveType::QUIET), "k7/8/8/8/6Q1/8/8/K2Q2Q1 w - - 0 1");
        }
    }

    mod pawn_captures {
        use super::*;

        #[test]
        fn parses_simple_pawn_capture() {
            assert_parses!("axb4", Move::new(A3, B4, MoveType::CAPTURE), "8/8/8/8/1p6/P7/8/8 w - - 0 1");
        }

        #[test]
        fn parses_pawn_capture_with_disambiguator() {
            // FIXME: I don't think this is a valid fen that would generate this move notation
            assert_parses!("a3xb4", Move::new(A3, B4, MoveType::CAPTURE), "8/8/8/8/1p6/P1P5/8/8 w - - 0 1");
        }

        #[test]
        fn parses_pawn_capture_and_promotion() {
            assert_parses!("exf8=R", Move::new(E7, F8, MoveType::PROMOTION_CAPTURE_ROOK), "5q2/4P3/8/8/8/7k/8/7K w - - 0 1");
        }

        #[test]
        fn parses_pawn_capture_with_disambiguator_and_promotion() {
            assert_parses!("e7xf8=B", Move::new(E7, F8, MoveType::PROMOTION_CAPTURE_BISHOP), "5q2/4P3/8/8/8/7k/8/7K w - - 0 1");
        }
    }

    mod pawn_pushes {
        use super::*;

        #[test]
        fn parses_pawn_double_push() {
            assert_parses!("e4", Move::new(E2, E4, MoveType::DOUBLE_PAWN));
        }

        #[test]
        fn parses_pawn_push() {
            assert_parses!("e3", Move::new(E2, E3, MoveType::QUIET));
        }

        #[test]
        fn parses_pawn_push_and_promotion() {
            assert_parses!("e8=N", Move::new(E7, E8, MoveType::PROMOTION_KNIGHT), "8/4P3/8/8/8/8/8/8 w - - 0 1");
        }
    }

    mod slide_attacks {
        use super::*;

        #[test]
        fn parses_slide_attack() {
            // set up an ambiguous scenario that requires a slide attack to be calculated for each
            // of the possible sliding attackers
            assert_parses!("Qd1", Move::new(A1, D1, MoveType::QUIET), "4k3/8/8/8/8/8/8/Q3P1QK w - - 0 1");
            assert_parses!("Rd1", Move::new(A1, D1, MoveType::QUIET), "4k3/8/8/8/8/8/8/R3P1RK w - - 0 1");
            assert_parses!("Bd4", Move::new(A1, D4, MoveType::QUIET), "4k3/8/5B2/4P3/8/8/8/B6K w - - 0 1");
        }
    }

    mod castles {
        use super::*;

        #[test]
        fn parses_short_castle() {
            assert_parses!("O-O", Move::new(E1, G1, MoveType::SHORT_CASTLE));
        }

        #[test]
        fn parses_long_castle() {
            assert_parses!("O-O-O", Move::new(E1, C1, MoveType::LONG_CASTLE));
        }
    }
}
