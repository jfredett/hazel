pub mod action;
pub mod delim;
pub mod familiar;
pub mod position;
pub mod reason;
pub mod variation;
pub mod state;

use action::Action;
use hazel_core::ben::BEN;
use hazel_core::color::Color;
use hazel_core::file::File;
use hazel_core::interface::{Alter, Query};
use hazel_core::occupant::Occupant;
use hazel_core::piece::Piece;
use hazel_core::position_metadata::PositionMetadata;
use hazel_core::square::*;

use crate::coup::rep::Move;
use crate::extensions::query::display_board;
use crate::interface::play::Play;

#[derive(Clone, Default)]
pub struct ChessGame<T> where T: Alter + Query + Default + Clone {
    // FIXME: This is bad, I don't like it.
    pub rep: T,
    pub metadata: PositionMetadata,
}


/*
* In this design, ChessGame can only roll _forward_, the unplay trait would require a bunch more
* context I don't have and don't want to put in ChessGame, so I suppose it'll have to be
* implemented further up in Familiar or something.
*/

// TODO: Maybe wrap the constraint in it's own typeclass?
impl<T> Play for ChessGame<T> where T: Alter + Query + Default + Clone {
    type Metadata = PositionMetadata;

    fn apply(&self, action: &Action<Move, BEN>) -> Self {
        let mut new_game = self.clone();
        new_game.apply_mut(action);
        new_game
    }

    fn apply_mut(&mut self, action: &Action<Move, BEN>) -> &mut Self {
        match action {
            Action::Setup(fen) => {
                let alts = fen.to_alterations();
                for a in alts {
                    self.rep.alter_mut(a);
                }
                self.metadata = fen.metadata();
            }
            Action::Make(mov) => {
                let alts = mov.compile(&self.rep);
                // Order matters, the metadata must be updated before the board
                { // HACK: This has been hard-inlined to support the move of `PositionMetadata` to
                    // -basics, it should be refactored before using. I'm pretty sure all this is
                    // going to go away though, so not a huge deal.
                    let this = &mut self.metadata;
                    let mov: &Move = mov;
                    let board = &self.rep;
                    // Clear the EP square, we'll re-set it if necessary later.
                    this.en_passant = None;


                    if this.side_to_move == Color::BLACK {
                        this.fullmove_number += 1;
                    }
                    this.side_to_move = !this.side_to_move;

                    // rely on the color of the piece being moved, rather than reasoning about the side-to-move
                    // or delaying it till the end.

                    let Occupant::Occupied(piece, color) = board.get(mov.source()) else { panic!("Move has no source piece: {:?}\n on: \n{}", mov, display_board(board)); };


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
                for a in alts {
                    self.rep.alter_mut(a);
                }
            }
            _ => {}
        }
        self
    }

    fn metadata(&self) -> PositionMetadata {
        self.metadata
    }
}


#[cfg(test)]
mod tests {
    use crate::board::PieceBoard;
    use crate::{coup::rep::{Move, MoveType}, game::ChessGame};
    use hazel_core::square::*;

    use super::*;


    #[test]
    fn correctly_calculates_position_after_several_moves() {
        let mut game : ChessGame<PieceBoard> = ChessGame::default();
        game.apply_mut(&Action::Setup(BEN::start_position()))
            .apply_mut(&Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN)));

        let expected_fen = BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");
        let actual_fen : BEN = BEN::from(game);

        similar_asserts::assert_eq!(actual_fen, expected_fen);
    }

    mod from_into {
        use super::*;

        #[test]
        fn from_ben() {
            let ben = BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq d3 0 2");
            let game : ChessGame<PieceBoard> = ben.into();
            let expected_fen = BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq d3 0 2");
            let actual_fen = BEN::from(game);

            similar_asserts::assert_eq!(actual_fen, expected_fen);
        }

        #[test]
        fn into_ben() {
            let mut game : ChessGame<PieceBoard> = ChessGame::default();
            game.apply_mut(&Action::Setup(BEN::start_position()));

            let ben : BEN = game.clone().into();
            let expected_fen = BEN::start_position();

            similar_asserts::assert_eq!(ben, expected_fen);
        }
    }

    mod play_impl {

        use super::*;

        #[test]
        fn play_applies_correctly() {
            let game = ChessGame::<PieceBoard>::from(BEN::start_position());
            let action = Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN));
            let new_game = game.apply(&action);
            let actual_ben : BEN = new_game.into();
            assert_eq!(actual_ben, BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1"));
        }

        #[test]
        fn play_applies_mutably_correctly() {
            let mut game = ChessGame::<PieceBoard>::from(BEN::start_position());
            let action = Action::Make(Move::new(D2, D4, MoveType::DOUBLE_PAWN));
            game.apply_mut(&action);
            let actual_ben : BEN = game.into();
            assert_eq!(actual_ben, BEN::new("rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1"));
        }
    }
}
