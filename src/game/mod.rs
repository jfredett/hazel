#![allow(dead_code, unused_imports)]

use pgn_reader::{San, Visitor, Role, CastlingSide};

pub mod line;
pub mod interface;

use crate::board::Query;
use crate::game::line::Line;
use crate::game::interface::Chess;
use crate::notation::MoveNotation;
use crate::coup::rep::Move;
use crate::types::{Color, Piece};
/*
*
    Normal {
        role: Role,
        file: Option<File>,
        rank: Option<Rank>,
        capture: bool,
        to: Square,
        promotion: Option<Role>,
    },
    Castle(CastlingSide),
    Put {
        role: Role,
        to: Square,
    },
    Null,
*/

impl From<Role> for Piece {
    fn from(role: Role) -> Self {
        match role {
            Role::Pawn => Piece::Pawn,
            Role::Knight => Piece::Knight,
            Role::Bishop => Piece::Bishop,
            Role::Rook => Piece::Rook,
            Role::Queen => Piece::Queen,
            Role::King => Piece::King,
        }
    }
}

pub fn interpret_san<Q : Query>(san: &San, context: Q, to_play: Color) -> Move {
    match san {
        San::Normal {
            role: _,
            file: _,
            rank: _,
            capture: _,
            to: _,
            promotion: _,
        } => {
            todo!()
        },
        San::Castle(side) => {
            match (to_play, side) {
                (Color::WHITE, CastlingSide::KingSide) => Move::short_castle(Color::WHITE),
                (Color::WHITE, CastlingSide::QueenSide) => Move::long_castle(Color::WHITE),
                (Color::BLACK, CastlingSide::KingSide) => Move::short_castle(Color::BLACK),
                (Color::BLACK, CastlingSide::QueenSide) => Move::long_castle(Color::BLACK),
            }
        },
        San::Put { role: _, to: _ } => {
            todo!()
        }
        San::Null => { Move::null() }
    }
}


struct Game {
    mainline: Line,
    variations: Vec<Line>,
}

impl Game {

    // needs to return a reference to some object which takes the input move and updates the parent
    // game before dying, I think this is a lifetime problem.
    pub fn get_mut(&mut self) -> &mut Line {
        &mut self.mainline
    }

    pub fn add<M : MoveNotation>(&mut self, mov: M) {
        // self.mainline.push(mov.into());
    }




}


/*
*
* plan:
*
* Game will _not_ implement Chess
* Game will just record the moves as halfplies, it's our internal representation of a PGN.
* Game will have a function to read a PGN mainline and hold it as halfplies.
*
* // maybe, but alternatively a #[] and #<< model might work too, and gets us a way to mutate the
* tree.
* Games should be iterators with a cursor (so next and prev?), since a game is a tree
*   - next() -> Show (but do not consume) next available moves in the game (i.e., mainline + variations) returns Option<Vec<Move>>
*   - prev() -> Previously selected move. returns Option<Move>
*       - Prev can be implemented by `GameRep` retaining a log of all the `Alteration`s it has made,
*       then popping that stack back to the last marked move.
*       - It can be implemented similarly in `Game` by keeping a stack of plies chosen, then
*       popping as needed.
*   - select(Move) -> Choose a move to play from the list provided by next(). If the move is not in the list of available moves, create a new variation
*
* I'm seeing a distinction that needs making between a PGN type (this type) and a GameRep type. I'm
* debatig which I want to build here. I think the GameRep doesn't need to keep track of the future
* moves, this type is more about reading everything, but I don't want to work on that part right
* now, I want to build the perft function, so 
*
*
* This should focus on being able to fully load a PGN using our notation systems but the pgnreader
* crate
*
*
* To do that, I'll need to implement a way to convert a PGN move to a MoveNotation.
* Then I'll write a visitor that visits the game tree, when it encounters a variation, it
*   recursively:
*       1. Copies the current line
*       2. Descends down into the variation, adding the line to the list of Lines to be added to
*          the list.
*       3. When it reaches the end, it returns the line which is appeneded to the back of the list
*          of variations and the mainline exploration continues
* The final result is a Vec<Line> with the first line being the mainline and all others being
* all variations in 'left-most' order.
*
*
* Since Games need to have some amount of context, a PieceBoard can be used during creation, but
* once the Game is created, it should not have any boardstate left with it, just halfplies full of
* unambiguous moves. During actual play, we'll always have the full UCI move to record, so
* ambiguity own't be possible.
*/


#[cfg(test)]
mod tests {
    use super::*;



}
