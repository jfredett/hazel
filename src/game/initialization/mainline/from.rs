use pgn_reader::Role;

use crate::constants::Piece;

impl From<Role> for Piece {
    fn from(val: Role) -> Self {
        match val {
            Role::Pawn => Piece::Pawn,
            Role::Knight => Piece::Knight,
            Role::Bishop => Piece::Bishop,
            Role::Rook => Piece::Rook,
            Role::Queen => Piece::Queen,
            Role::King => Piece::King,
        }
    }
}
