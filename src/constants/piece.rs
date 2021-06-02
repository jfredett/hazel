#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    King    = 0,
    Queen   = 1,
    Rook    = 2,
    Bishop  = 3,
    Knight  = 4,
    Pawn    = 5
}

pub const PIECES : [Piece; 6]= [
    Piece::King,
    Piece::Queen,
    Piece::Rook,
    Piece::Bishop,
    Piece::Knight,
    Piece::Pawn     
];

pub const ASCII_PIECE_CHARS : [[char; 6]; 2] = [
    [ 'K', 'Q', 'R', 'B', 'N', 'P' ],
    [ 'k', 'q', 'r', 'b', 'n', 'p' ]
];

// BUG: I couldn't get the formatting right w/ unicode, but I don't want to lose
// track of the unicode characters, so I'm keeping them here.
#[allow(unused_variables)]
pub const UNICODE_PIECE_CHARS : [[char; 6]; 2] = [
    [
        '\u{2654}', //'♔';
        '\u{2655}', //'♕';
        '\u{2656}', //'♖';
        '\u{2657}', //'♗';
        '\u{2658}', //'♘';
        '\u{2659}'  //'♙';
    ], [
        '\u{265A}', //'♚';
        '\u{265B}', //'♛';
        '\u{265C}', //'♜';
        '\u{265D}', //'♝';
        '\u{265E}', //'♞';
        '\u{265F}'  //'♟︎';
    ]
];