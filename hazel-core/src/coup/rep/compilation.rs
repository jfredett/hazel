use hazel_basic::position_metadata::PositionMetadata;
use hazel_basic::square::*;

use super::*;

impl Move {

    // TODO: Query metadata situation needs addressing, should have like, a gamestate trait?
    pub fn new_compile<C>(&self, context: &C, metadata: &PositionMetadata) -> Vec<Alteration> where C : Query {
        let mut ret : Vec<Alteration> = vec![];

        // A turn looks like this:
        //
        // Turn
        //    // Current Move Metadata checks, this asserts the current state of the board
        //    Assert(SideToMove(Color)) // present every turn
        //    Assert(CastleRights(KQkq)) // present every turn
        //    Assert(EnPassant(File)) // only present if there is a valid file
        //    Assert(HalfMoveCount(u8)) // present every turn
        //    Assert(FullMoveCount(u16)) // present every turn
        //    Assert(InCheck) // present only if the position starts with the STM's king in check
        //
        //    // Move section many place/remove instructions possible.
        //    Remove(Square, Piece)
        //    Place(Square, Piece)
        //
        //    // Information section, where new metadata facts are provided
        //    Inform(MoveType(MoveType)) // the movetype, present unless it's quiet
        //    Inform(CastulRights(KQkq)) // present if they've changed
        //    Inform(Check) // only if the move is a check
        //    Inform(HalfMoveReset)
        //    // etc
        //
        // When a turn is processed, the position hash for the completed turn is when the familiar
        // is on the `End` token. The metadata in the `Assert` section must be sufficient to
        // transform a `PositionMetadata::default()` to match the provided `metadata` via the
        // `#alter_mut` method.

        // Turn
        ret.push(Alteration::Turn);

        // Assert section
        // Plan:
        // 1. use exisitng methods to start
        // 2. inline them
        // 3. refactor.
        let alters : Vec<Alteration> = metadata.into();
        ret.extend(alters);

        // Move section
        let alters : Vec<Alteration> = self.compile(context);
        ret.extend(alters);

        // Information section
        let mut new_metadata = *metadata;
        {
            // HACK: This is a direct inline, it should be factored down
            let this = &mut new_metadata;
            let mov = self;
            let board = context;
            // Clear the EP square, we'll re-set it if necessary later.
            this.en_passant = None;


            if this.side_to_move == Color::BLACK {
                this.fullmove_number += 1;
            }
            this.side_to_move = !this.side_to_move;

            // rely on the color of the piece being moved, rather than reasoning about the side-to-move
            // or delaying it till the end.

            let Occupant::Occupied(piece, color) = board.get(mov.source()) else { panic!("Move has no source piece: {:?}\n on: \n{}", mov, hazel_basic::interface::query::display_board(board)); };


            if mov.is_capture() || piece == Piece::Pawn {
                this.halfmove_clock = 0;
            } else {
                this.halfmove_clock += 1;
            }

            let source = mov.source();
            match piece {
                Piece::King => {
                    match color  {
                        Color::WHITE => {
                            this.castling.white_short = false;
                            this.castling.white_long = false;
                        }
                        Color::BLACK => {
                            this.castling.black_short = false;
                            this.castling.black_long = false;
                        }
                    }
                }
                Piece::Rook if source == H1 => { this.castling.white_short = false; }
                Piece::Rook if source == H8 => { this.castling.black_short = false; }
                Piece::Rook if source == A1 => { this.castling.white_long = false; }
                Piece::Rook if source == A8 => { this.castling.black_long = false; }
                Piece::Rook => {}
                Piece::Pawn => {
                    this.en_passant = if mov.is_double_pawn_push_for(color) {
                        mov.target().shift(color.pawn_direction()).map(|target| File::from(target.file()))
                    } else {
                        None
                    }
                }
                _ => {}
            }
        };
        ret.push(Alteration::Inform(new_metadata));

        // NOTE: Finer grained approach here...
        // let mut new_metadata = *metadata;
        // new_metadata.update(self, context);
        // if let Some(file) = new_metadata.en_passant {
        //     ret.push(Alteration::Inform(MetadataAssertion::EnPassant(file)));
        // }
        // ret.push(Alteration::Inform(MetadataAssertion::FiftyMoveCount(new_metadata.halfmove_clock)));
        // ret.push(Alteration::Inform(MetadataAssertion::MoveType(self.move_metadata())));
        // ret.push(Alteration::Inform(MetadataAssertion::SideToMove(new_metadata.side_to_move)));

        ret
    }

    /// Disambiguates the move in the context of the provided query. If the move is not marked ambiguous,
    /// the move is returned as is. If the move is ambiguous, the context is used to determine the
    /// correct metadata for the move. No effort is made to ensure legality of the move.
    ///
    /// TODO: Does not look for check states.
    pub fn disambiguate<C>(&self, context: &C) -> Option<MoveType> where C: Query {
        // If we are not ambiguous, just return the move as is.
        if !self.is_ambiguous() { return Some(self.move_metadata()); }

        let source = context.get(self.source());
        let target = context.get(self.target());

        // If the source square is empty, we can't disambiguate
        if source.is_empty() { return None; }

        let capturing = !target.is_empty(); // If there is a piece on the target square, then we're capturing

        match source.piece().unwrap() {
            Piece::Pawn => {
                // we might still be capturing en passant, we can check to see if we're moving
                // diagonally. This can be done by checking the difference between the source
                // and target. We can also determine color here. -- if source > target, then
                // we're moving black pieces.
                let delta = self.target_idx() as isize - self.source_idx() as isize;

                if delta.abs() == 0o20 {
                    // now we can also check for a double-pawn move. the delta is just 2 column
                    // moves, which is 0o20, or a <2,0> vector, if you like.
                    return Some(MoveType::DOUBLE_PAWN);
                } else if delta.abs() == 0o11 {
                    // This implies `capturing`. Since a diagonal move is always a capture for a
                    // pawn. However, `capturing` may be unset if the move is `en passant`.
                    //
                    if !capturing {
                        // So if the capture flag is unset, but we are doing a capture move, it
                        // must be EP_CAPTURE.
                        return Some(MoveType::EP_CAPTURE);
                    } else if self.target().backrank() {
                        // If we are moving to the backrank, and we are capturing, we must be capture-promoting
                        // BUG: This doesn't let you promote to anything other than a queen, and
                        // in fact it is known that some positions promotion to a non-queen is the
                        // only way to avoid a stalemate, so this is a bug.
                        return Some(MoveType::PROMOTION_CAPTURE_QUEEN);
                    } else {
                        // Otherwise, we are just capturing as normal
                        // BUG: This doesn't let you promote to anything other than a queen, and
                        // in fact it is known that some positions promotion to a non-queen is the
                        // only way to avoid a stalemate, so this is a bug.
                        return Some(MoveType::CAPTURE);
                    }
                } else if self.target().backrank() {
                    return Some(MoveType::PROMOTION_QUEEN);
                } else {
                    // No capture, no double-pawn, no promotion, no en passant, just a quiet move.
                    return Some(MoveType::QUIET);
                }
            },
            Piece::King => {
                // Castling is a king move in UCI, so it's a king move as far as I'm concerned.
                match self.source() {
                    E1 => {
                        if self.target() == G1 {
                            return Some(MoveType::SHORT_CASTLE);
                        } else if self.target() == C1 {
                            return Some(MoveType::LONG_CASTLE);
                        }
                    },
                    E8 => {
                        if self.target() == G8 {
                            return Some(MoveType::SHORT_CASTLE);
                        } else if self.target() == C8 {
                            return Some(MoveType::LONG_CASTLE);
                        }
                    },
                    _ => { },
                }
            },
            _ => { },
        };
        // Otherwise, moves are just captures or quiet, simple as.
        if capturing {
            Some(MoveType::CAPTURE)
        } else {
            Some(MoveType::QUIET)
        }
    }

    pub fn compile<C>(&self, context: &C) -> Vec<Alteration> where C : Query {
        let source = self.source();
        let target = self.target();

        let source_occupant = context.get(source);
        let target_occupant = context.get(target);

        let contextprime = self.disambiguate(context).unwrap();

        match contextprime {
            MoveType::QUIET => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, source_occupant),
            ],
            MoveType::DOUBLE_PAWN => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, source_occupant),
            ],
            MoveType::SHORT_CASTLE => {
                let color = source_occupant.color().unwrap();
                let rook_source = match color {
                    Color::WHITE => H1,
                    Color::BLACK => H8,
                };
                let rook_target = match color {
                    Color::WHITE => F1,
                    Color::BLACK => F8,
                };
                vec![
                    Alteration::remove(rook_source, Occupant::rook(color)),
                    Alteration::remove(source, source_occupant),
                    Alteration::place(target, source_occupant),
                    Alteration::place(rook_target, Occupant::rook(color)),
                ]
            },
            MoveType::LONG_CASTLE => { 
                let color = source_occupant.color().unwrap();
                let rook_source = match color {
                    Color::WHITE => A1,
                    Color::BLACK => A8
                };
                let rook_target = match color {
                    Color::WHITE => D1,
                    Color::BLACK => D8
                };
                vec![
                    // remove the rook
                    Alteration::remove(rook_source, Occupant::rook(color)),
                    // remove the king
                    Alteration::remove(source, source_occupant),
                    // place the king
                    Alteration::place(target, source_occupant),
                    // place the rook
                    Alteration::place(rook_target, Occupant::rook(color))
                ]
            },
            MoveType::CAPTURE => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, source_occupant),
            ],
            MoveType::EP_CAPTURE => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, source_occupant),
            ],
            MoveType::PROMOTION_KNIGHT => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, Occupant::knight(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_BISHOP => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, Occupant::bishop(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_ROOK => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, Occupant::rook(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_QUEEN => vec![
                Alteration::remove(source, source_occupant),
                Alteration::place(target, Occupant::queen(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_CAPTURE_KNIGHT => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, Occupant::knight(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_CAPTURE_BISHOP => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, Occupant::bishop(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_CAPTURE_ROOK => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, Occupant::rook(source_occupant.color().unwrap())),
            ],
            MoveType::PROMOTION_CAPTURE_QUEEN => vec![
                Alteration::remove(source, source_occupant),
                Alteration::remove(target, target_occupant),
                Alteration::place(target, Occupant::queen(source_occupant.color().unwrap())),
            ],
            MoveType::NULLMOVE => vec![],
            _ => { unreachable!(); }
        }
    }
}
