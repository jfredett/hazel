
use nom::{character::complete::char, combinator::opt, IResult};
use tracing::{debug, instrument};

use crate::{board::Query, coup::rep::{Move, MoveType}, types::{Color, Occupant, Piece}};

use super::Square;


mod disambiguator;

use disambiguator::Disambiguator;

#[derive(Debug)]
pub struct SAN {
    source_sq: Option<Square>,
    target_sq: Square,
    movetype: MoveType
}

impl SAN {
    #[instrument(skip(context))]
    pub fn piece_move<'a>(input: &'a str, side_moving: Color, context: impl Query) -> IResult<&'a str, Self> {
        // The first character is always the piece, and for this parser, never the pawn.
        let (input, piece) = Piece::parse(input)?;
        assert!(piece != Piece::Pawn);

        // We're going to talk about this again in a minute. SAN is a stupid design.
        let (input, mystery) = opt(Disambiguator::parse)(input)?;
        // If there is an `x`, it's a capture, so we know the previous item must be a
        // disambiguator. Even if `x` is not present, if the next line is another full square
        // notation, we know that the previous item must be a disambiguator.
        let (input, is_capture) = opt(char('x'))(input)?;
        let is_capture = !is_capture.is_none();

        // try to parse the target square.
        let (input, target_sq) = opt(Square::parse)(input)?;

        // The next item is either a disambiguator or the target square, depending on if this is a
        // capture and whether or not we have an ambiguous quiet move (consider a board with two
        // knights on d2 and d6, and a move to e4. Ne4 is ambiguous, but is not a capture). So we
        // hold this item until we see what else the notation has.
        let source_disambiguator;
        let target_sq = match target_sq {
            Some(sq) => {
                // we have a target square set, so the mystery item is a disambiguator for the
                // soure square, and so the mystery item will definitely have it's 'square' set.
                source_disambiguator = mystery;
                sq
            },
            None => {
                // no target square parsed, so the mystery item is the target square.
                source_disambiguator = None;
                mystery.unwrap().square()
            }
        };

        debug!("input: {}, piece: {piece:?}, source_disambiguator: {source_disambiguator:?}", input);

        let source_occupant = Occupant::Occupied(piece, side_moving);


        let mut possible_source_squares = vec![];
        for sq in target_sq.moves_for(&piece, &side_moving) {
            if context.get(sq) == source_occupant {
                possible_source_squares.push(sq);
            }
        }

        let m = match source_disambiguator {
            Some(disambiguator) => {
                match disambiguator {
                    Disambiguator::File(file) => {
                        // The disambiguator is a file, so we need to find the piece on that file.
                        Square::along_file(file).find(|sq| {
                            context.get(sq) == Occupant::Occupied(piece, side_moving)
                        }).unwrap()
                    },
                    Disambiguator::Rank(rank) => {
                        Square::along_rank(rank).find(|sq| {
                            context.get(sq) == Occupant::Occupied(piece, side_moving)
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
                        panic!("No possible source squares found for piece {:?}", piece);
                    }
                    1 => possible_source_squares[0],
                    _ => {
                        // We have multiple possible source squares, so we need to disambiguate.
                        panic!("Move is ambiguous without disambiguator");
                    }
                }
            }
        };

        let ret = SAN {
            source_sq: Some(m),
            target_sq,
            movetype: if is_capture { MoveType::capture() } else { MoveType::quiet() },
        };


        Ok((input, ret))
    }

    pub fn pawn_push(&self, color: Color, context: impl Query) -> IResult<&str, Self> {
        todo!()
    }

    pub fn pawn_capture(&self, color: Color, context: impl Query) -> IResult<&str, Self> {
        todo!()
    }

    #[instrument(skip(v))]
    pub fn parse(input: &str, v: impl Query + Clone) -> IResult<&str, Self> {
        // Cases:
        // Piece Move : ex: Nf3, Qxd4, etc. basically <piece>(<from square>)?x?<to square>
        debug!("{}", input);
        if let (input, Some(piece_move)) = opt(|input| Self::piece_move(input, Color::WHITE, v.clone()))(input)? {
            return Ok((input, piece_move));
        }
        debug!("{}", input);
        /*
        // Pawn Capture : ex: a3xb4, exf8=Q, etc. basically  <from file>[<from rank>]?x?<to square>[=piece]}
        if let (input, Some(pawn_capture)) = opt(|input| Self::pawn_capture(input, Color::WHITE, v.clone()))(input)? {
            return Ok((input, pawn_capture));
        }
        // Pawn Push : ex: e4, e8=Q
        if let (input, Some(pawn_push)) = opt(|input| Self::pawn_push(input, Color::WHITE, v.clone()))(input)? {
            return Ok((input, pawn_push));
        }
        */
        // SAN { movetxt: input.to_string(), context: v }
        todo!()

    }
}

#[derive(Debug)]
pub enum SANConversionError {
    MissingSourceSq,
    MissingMoveType,
}

impl TryFrom<SAN> for Move {
    type Error = SANConversionError;

    // This expects that the context has already been embedded and the movetype is disambiguated.
    // That should happen at parse-time for SAN, since we only see this in PGNs, really. UCI does a
    // LAN-style which is much easier to convert since the source_sq is already there.
    fn try_from(san: SAN) -> Result<Move, SANConversionError> {
        if san.source_sq.is_none() {
            return Err(SANConversionError::MissingSourceSq);
        }

        if san.movetype == MoveType::UCI_AMBIGUOUS {
            return Err(SANConversionError::MissingMoveType);
        }

        Ok(Move::new(san.source_sq.unwrap(), san.target_sq, san.movetype))
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
            context.setup(FEN::new($fen)).commit();
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
            assert_parses!("Qxd4", Move::new(D1, D4, MoveType::CAPTURE), "8/8/8/3p4/8/8/8/3Q4 w - - 0 1");
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
            assert_parses!("Qg1d4", Move::new(G1, D4, MoveType::QUIET), "k7/8/8/8/42Q1/8/8/K2Q2Q1 w - - 0 1");
        }

        #[test]
        fn parses_non_capture_with_disambiguator_and_file() {
            assert_parses!("Q4d4", Move::new(G4, D4, MoveType::QUIET), "k7/8/8/8/42Q1/8/8/K2Q2Q1 w - - 0 1");
        }

        #[test]
        fn parses_non_capture_with_disambiguator_and_rank() {
            assert_parses!("Qdd4", Move::new(D1, D4, MoveType::QUIET), "k7/8/8/8/42Q1/8/8/K2Q2Q1 w - - 0 1");
        }
    }

    mod pawn_captures {
        use super::*;

    }

    mod pawn_pushes {
        use super::*;

    }
}
