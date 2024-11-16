
use nom::{character::complete::char, combinator::opt, IResult};
use tracing::{debug, instrument};

use crate::{board::Query, coup::rep::{Move, MoveType}, types::{Occupant, Piece}};

use super::{ben::BEN, Square};


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
    context: BEN,
}

impl SAN {
    pub fn is_ambiguous(&self) -> bool {
        self.source_sq.is_none() || self.ambiguous_sq.is_some()
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
            context: fen.into(),
        }
    }

    fn promotion(input: &str) -> IResult<&str, Piece> {
        let (input, _) = char('=')(input)?;
        Piece::parse(input)
    }

    pub fn parse(input: &str, context: impl Into<BEN>) -> IResult<&str, Self> {
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
            promotion,
            context: context.into(),
        };
        let _ = san.disambiguate();

        Ok((input, san))
    }

    /// Disambiguate the _source square_ and _only_ the source square. This rest of disambiguation
    /// is
    #[instrument]
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
            debug!("Checking square: {:?}", sq);
            if self.context.get(sq) == source_occupant {
                debug!("Found possible source square: {:?}", sq);
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
                debug!("Possible source squares: {:?}", possible_source_squares);
                match possible_source_squares.len() {
                    0 => {
                        return Err(SANConversionError::NoPossibleSourceSquares);
                    }
                    1 => possible_source_squares[0],
                    _ => {
                        // We have multiple possible source squares, so we need to disambiguate.
                        return Err(SANConversionError::IsAmbiguous("Multiple possible source squares"));
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

    use crate::coup::rep::MoveType;
    use crate::{board::PieceBoard, constants::START_POSITION_FEN, notation::fen::FEN};
    use crate::notation::*;
    use crate::notation::uci::UCI;
    use crate::game::variation::Variation;

    use super::*;

    macro_rules! assert_parses {
        ($input:expr, $expected:expr, $fen:expr, $moves:expr) => {
        let mut context = Variation::default();
        debug!("Setting context to FEN: {}", $fen);
        context.setup(FEN::new($fen)).commit();
        for m in $moves.iter().map(|m| UCI::try_from(*m).unwrap()) {
        debug!("Making move: {:?}", m);
        context.make(m.into());
        }
        debug!("Parsing: {}", $input);
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

        #[tracing_test::traced_test]
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

        #[tracing_test::traced_test]
        #[test]
        fn parses_simple_pawn_capture() {
            assert_parses!("axb4", Move::new(A3, B4, MoveType::CAPTURE), "8/8/8/8/1p6/P7/8/8 w - - 0 1");
        }

        #[tracing_test::traced_test]
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

        #[tracing_test::traced_test]
        #[test]
        fn parses_pawn_double_push() {
            assert_parses!("e4", Move::new(E2, E4, MoveType::DOUBLE_PAWN));
        }

        #[tracing_test::traced_test]
        #[test]
        fn parses_pawn_push() {
            assert_parses!("e3", Move::new(E2, E3, MoveType::QUIET));
        }

        #[test]
        fn parses_pawn_push_and_promotion() {
            assert_parses!("e8=N", Move::new(E7, E8, MoveType::PROMOTION_KNIGHT), "8/4P3/8/8/8/8/8/8 w - - 0 1");
        }
    }
}
